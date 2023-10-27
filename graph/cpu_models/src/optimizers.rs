use express_measures::ThreadFloat;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub trait Optimizer<V>: Serialize
where
    V: Send + Sync + ?Sized + Serialize + DeserializeOwned,
    Self: Send + Sync + Clone,
{
    type T: ?Sized;

    fn get_update(&mut self, variations: &mut Self::T);
    fn set_capacity(&mut self, capacity: usize);
}

#[derive(Clone, Deserialize, Serialize)]
pub struct StocaticGradientDescent<F>
where
    F: ThreadFloat,
{
    learning_rate: F,
}

impl<F> StocaticGradientDescent<F>
where
    F: ThreadFloat,
{
    pub fn new(learning_rate: F) -> Self {
        Self { learning_rate }
    }
}

impl<F> Optimizer<Vec<F>> for StocaticGradientDescent<F>
where
    F: ThreadFloat + Serialize + DeserializeOwned,
{
    type T = [F];

    fn set_capacity(&mut self, _capacity: usize) {}

    fn get_update(&mut self, variations: &mut Self::T) {
        variations
            .iter_mut()
            .for_each(|value| *value *= self.learning_rate);
    }
}

impl<F> Optimizer<F> for StocaticGradientDescent<F>
where
    F: ThreadFloat + Serialize + DeserializeOwned,
{
    type T = F;

    fn set_capacity(&mut self, _capacity: usize) {}

    fn get_update(&mut self, variation: &mut Self::T) {
        *variation *= self.learning_rate;
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Momentum<F, V>
where
    F: ThreadFloat,
{
    learning_rate: F,
    decay_factor: F,
    momentum: V,
}

impl<F, V> From<Momentum<F, Vec<V>>> for Momentum<F, V>
where
    V: Default,
    F: ThreadFloat,
{
    fn from(other: Momentum<F, Vec<V>>) -> Self {
        Self::new(other.learning_rate, other.decay_factor)
    }
}

impl<F, V> Momentum<F, V>
where
    V: Default,
    F: ThreadFloat,
{
    pub fn new(learning_rate: F, decay_factor: F) -> Self {
        Self {
            learning_rate,
            decay_factor,
            momentum: V::default(),
        }
    }
}

impl<F> Optimizer<F> for Momentum<F, F>
where
    F: ThreadFloat + Serialize + DeserializeOwned,
{
    type T = F;

    fn set_capacity(&mut self, capacity: usize) {
        if capacity != 1 {
            unimplemented!(
                "Scalar optimizer is only implemented for objects with capacity equal to one."
            );
        }
        self.momentum = F::zero();
    }

    fn get_update(&mut self, variation: &mut Self::T) {
        self.momentum = self.decay_factor * self.momentum + self.learning_rate * (*variation);
        *variation = self.momentum;
    }
}

impl<F> Optimizer<Vec<F>> for Momentum<F, Vec<F>>
where
    F: ThreadFloat + Serialize + DeserializeOwned,
    Vec<F>: Serialize,
{
    type T = [F];

    fn set_capacity(&mut self, capacity: usize) {
        self.momentum = vec![F::zero(); capacity]
    }

    fn get_update(&mut self, variations: &mut Self::T) {
        self.momentum
            .iter_mut()
            .zip(variations.iter_mut())
            .for_each(|(value, variation)| {
                *value = self.decay_factor * (*value) + self.learning_rate * (*variation);
                *variation = *value;
            });
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Adam<F, V>
where
    V: Default,
    F: ThreadFloat,
{
    learning_rate: F,
    first_order_decay_factor: F,
    second_order_decay_factor: F,
    time: F,
    first_moment: V,
    second_moment: V,
}

impl<F, V> From<Adam<F, Vec<V>>> for Adam<F, V>
where
    V: Default,
    F: ThreadFloat,
{
    fn from(other: Adam<F, Vec<V>>) -> Self {
        Self::new(
            Some(other.learning_rate),
            Some(other.first_order_decay_factor),
            Some(other.second_order_decay_factor),
        )
    }
}

impl<F, V> Adam<F, V>
where
    V: Default,
    F: ThreadFloat,
{
    pub fn new(
        learning_rate: Option<F>,
        first_order_decay_factor: Option<F>,
        second_order_decay_factor: Option<F>,
    ) -> Self {
        Self {
            learning_rate: learning_rate.unwrap_or(F::from(0.001).unwrap()),
            first_order_decay_factor: first_order_decay_factor.unwrap_or(F::from(0.9).unwrap()),
            second_order_decay_factor: second_order_decay_factor.unwrap_or(F::from(0.999).unwrap()),
            time: F::zero(),
            first_moment: V::default(),
            second_moment: V::default(),
        }
    }

    fn get_elementwise_update(
        variation: &mut F,
        first_moment: &mut F,
        second_moment: &mut F,
        first_order_decay_factor: F,
        second_order_decay_factor: F,
        learning_rate: F,
        time: F,
    ) {
        *first_moment = first_order_decay_factor * (*first_moment)
            + (F::one() - first_order_decay_factor) * (*variation);
        *second_moment = second_order_decay_factor * (*second_moment)
            + (F::one() - second_order_decay_factor) * (*variation).powi(F::one() + F::one());
        let alpha = learning_rate * (F::one() - second_order_decay_factor.powi(time)).sqrt()
            / (F::one() - first_order_decay_factor.powi(time));
        *variation = alpha * (*first_moment) / (F::epsilon() + (*second_moment).sqrt());
    }
}

impl<F> Optimizer<F> for Adam<F, F>
where
    F: ThreadFloat + Serialize + DeserializeOwned,
{
    type T = F;

    fn set_capacity(&mut self, capacity: usize) {
        if capacity != 1 {
            unimplemented!(
                "Scalar optimizer is only implemented for objects with capacity equal to one."
            );
        }
        self.first_moment = F::zero();
        self.second_moment = F::zero();
    }

    fn get_update(&mut self, variation: &mut Self::T) {
        self.time += F::one();
        Self::get_elementwise_update(
            variation,
            &mut self.first_moment,
            &mut self.second_moment,
            self.first_order_decay_factor,
            self.second_order_decay_factor,
            self.learning_rate,
            self.time,
        );
    }
}

impl<F> Optimizer<Vec<F>> for Adam<F, Vec<F>>
where
    F: ThreadFloat + Serialize + DeserializeOwned,
    Vec<F>: Serialize,
{
    type T = [F];

    fn set_capacity(&mut self, capacity: usize) {
        self.first_moment = vec![F::zero(); capacity];
        self.second_moment = vec![F::zero(); capacity];
    }

    fn get_update(&mut self, variations: &mut Self::T) {
        self.time += F::one();
        variations
            .iter_mut()
            .zip(
                self.first_moment
                    .iter_mut()
                    .zip(self.second_moment.iter_mut()),
            )
            .for_each(|(variation, (first_moment_value, second_moment_value))| {
                Self::get_elementwise_update(
                    variation,
                    first_moment_value,
                    second_moment_value,
                    self.first_order_decay_factor,
                    self.second_order_decay_factor,
                    self.learning_rate,
                    self.time,
                );
            });
    }
}

use super::*;
use num_traits::Zero;

#[allow(non_snake_case)]
pub(crate) mod EdgeWeightParser {

    use super::*;

    pub fn ignore<T, E>(
        value: Result<(usize, (T, T, E, WeightT))>,
    ) -> Result<(usize, (T, T, E, WeightT))> {
        value
    }

    pub fn validate<T: std::fmt::Display, E>(
        value: Result<(usize, (T, T, E, WeightT))>,
    ) -> Result<(usize, (T, T, E, WeightT))> {
        let (line_number, (src, dst, edge_type, weight)) = value?;
        if weight.is_zero() {
            return Err(format!(
                concat!(
                    "The weights cannot be zero.\n",
                    "The edge weight of the edge on line {} ",
                    "source node `{}` and destination node `{}` ",
                    "is zero."
                ),
                line_number, src, dst
            ));
        }
        if weight.is_nan() {
            return Err(format!(
                concat!(
                    "The weights cannot be NaN.\n",
                    "The edge weight of the edge on line {} ",
                    "source node `{}` and destination node `{}` ",
                    "is NaN."
                ),
                line_number, src, dst
            ));
        }
        if weight.is_infinite() {
            return Err(format!(
                concat!(
                    "The weights cannot be infinite.\n",
                    "The edge weight of the edge on line {} ",
                    "source node `{}` and destination node `{}` ",
                    "is infinite."
                ),
                line_number, src, dst
            ));
        }
        Ok((line_number, (src, dst, edge_type, weight)))
    }
}

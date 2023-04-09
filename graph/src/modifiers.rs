use super::*;

impl Graph {
    /// Enable extra perks that buys you time as you accept to spend more memory.
    ///
    /// # Arguments
    /// * `vector_sources`: Option<bool> - Whether to cache sources into a vector for faster walks.
    /// * `vector_reciprocal_sqrt_degrees`: Option<bool> - Whether to cache reciprocal_sqrt_degrees into a vector for faster laplacian kernel computation.
    pub fn enable(
        &mut self,
        vector_sources: Option<bool>,
        vector_reciprocal_sqrt_degrees: Option<bool>,
    ) {
        //let vector_sources = vector_sources.unwrap_or(false);
        let vector_reciprocal_sqrt_degrees = vector_reciprocal_sqrt_degrees.unwrap_or(false);

        // TODO!:
        //if vector_sources {
        //    self.edges.enable_sources();
        //} else {
        //    self.edges.disable_sources();
        //}
        if vector_reciprocal_sqrt_degrees {
            if self.reciprocal_sqrt_degrees.is_none() {
                self.reciprocal_sqrt_degrees = Arc::new(Some(self.get_reciprocal_sqrt_degrees()));
            }
        } else {
            self.reciprocal_sqrt_degrees = Arc::new(None);
        }
    }

    /// Disable all extra perks, reducing memory impact but incresing time requirements.
    pub fn disable_all(&mut self) {
        // TODO!:
        //self.edges.disable_sources();
        self.reciprocal_sqrt_degrees = Arc::new(None);
    }
}

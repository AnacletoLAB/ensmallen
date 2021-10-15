use super::*;

impl Graph {
    /// Enable extra perks that buys you time as you accept to spend more memory.
    ///
    /// # Arguments
    /// * `vector_sources`: Option<bool> - Whether to cache sources into a vector for faster walks.
    /// * `vector_destinations`: Option<bool> - Whether to cache destinations into a vector for faster walks.
    /// * `vector_cumulative_node_degrees`: Option<bool> - Whether to cache cumulative_node_degrees into a vector for faster walks.
    /// * `vector_reciprocal_sqrt_degrees`: Option<bool> - Whether to cache reciprocal_sqrt_degrees into a vector for faster laplacian kernel computation.
    pub fn enable(
        &mut self,
        vector_sources: Option<bool>,
        vector_destinations: Option<bool>,
        vector_cumulative_node_degrees: Option<bool>,
        vector_reciprocal_sqrt_degrees: Option<bool>,
    ) -> Result<()> {
        let vector_sources = vector_sources.unwrap_or(false);
        let vector_destinations = vector_destinations.unwrap_or(true);
        let vector_cumulative_node_degrees = vector_cumulative_node_degrees.unwrap_or(true);
        let vector_reciprocal_sqrt_degrees = vector_reciprocal_sqrt_degrees.unwrap_or(false);

        if vector_destinations {
            if self.destinations.is_none() {
                self.destinations = Some(self.get_directed_destination_node_ids());
            }
        } else {
            self.destinations = None;
        }
        if vector_sources {
            if self.sources.is_none() {
                self.sources = Some(self.get_directed_source_node_ids());
            }
        } else {
            self.sources = None;
        }
        if vector_cumulative_node_degrees {
            if self.cumulative_node_degrees.is_none() {
                self.cumulative_node_degrees = Some(self.get_cumulative_node_degrees());
            }
        } else {
            self.cumulative_node_degrees = None;
        }
        if vector_reciprocal_sqrt_degrees {
            if self.reciprocal_sqrt_degrees.is_none() {
                self.reciprocal_sqrt_degrees = Some(self.get_reciprocal_sqrt_degrees());
            }
        } else {
            self.reciprocal_sqrt_degrees = None;
        }
        Ok(())
    }

    /// Disable all extra perks, reducing memory impact but incresing time requirements.
    pub fn disable_all(&mut self) {
        self.destinations = None;
        self.sources = None;
        self.cumulative_node_degrees = None;
        self.reciprocal_sqrt_degrees = None;
    }
}

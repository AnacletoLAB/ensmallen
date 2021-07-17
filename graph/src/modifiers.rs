use super::*;

impl Graph {
    /// Enable extra perks that buys you time as you accept to spend more memory.
    ///
    /// # Arguments
    /// * `vector_sources`: Option<bool> - Whether to cache sources into a vector for faster walks.
    /// * `vector_destinations`: Option<bool> - Whether to cache destinations into a vector for faster walks.
    /// * `vector_cumulative_node_degrees`: Option<bool> - Whether to cache cumulative_node_degrees into a vector for faster walks.
    pub fn enable(
        &mut self,
        vector_sources: Option<bool>,
        vector_destinations: Option<bool>,
        vector_cumulative_node_degrees: Option<bool>,
    ) -> Result<()> {
        let vector_sources = vector_sources.unwrap_or(false);
        let vector_destinations = vector_destinations.unwrap_or(true);
        let vector_cumulative_node_degrees = vector_cumulative_node_degrees.unwrap_or(true);

        if vector_destinations {
            if self.destinations.is_none() {
                self.destinations = Some(self.get_destination_node_ids(true));
            }
        } else {
            self.destinations = None;
        }
        if vector_sources {
            if self.sources.is_none() {
                self.sources = Some(self.get_source_node_ids(true));
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
        Ok(())
    }

    /// Disable all extra perks, reducing memory impact but incresing time requirements.
    pub fn disable_all(&mut self) {
        self.destinations = None;
        self.sources = None;
        self.cumulative_node_degrees = None;
    }
}

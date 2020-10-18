use super::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::iter::once;

impl Graph {
    /// Set the name of the graph.
    /// 
    /// # Arguments
    /// 
    /// * name: String - Name of the graph.
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }
}

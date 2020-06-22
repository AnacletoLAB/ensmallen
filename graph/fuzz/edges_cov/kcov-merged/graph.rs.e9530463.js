var data = {lines:[
{"lineNum":"    1","line":"use derive_getters::Getters;"},
{"lineNum":"    2","line":"use log::info;"},
{"lineNum":"    3","line":"use rayon::prelude::*;"},
{"lineNum":"    4","line":"use std::collections::{HashMap};"},
{"lineNum":"    5","line":"use super::types::*;"},
{"lineNum":"    6","line":"use super::random::sample;"},
{"lineNum":"    7","line":"use arbitrary::Arbitrary;"},
{"lineNum":"    8","line":""},
{"lineNum":"    9","line":""},
{"lineNum":"   10","line":"// TODO FIGURE OUT HOW TO REMOVE PUB FROM ATTRIBUTESuse arbitrary::Arbitrary;"},
{"lineNum":"   11","line":"#[derive(Arbitrary)]","class":"lineNoCov","hits":"0",},
{"lineNum":"   12","line":"#[derive(Debug, Clone, Getters)]","class":"lineNoCov","hits":"0",},
{"lineNum":"   13","line":"pub struct Graph {"},
{"lineNum":"   14","line":"    pub sources: Vec<NodeT>,","class":"lineNoCov","hits":"0",},
{"lineNum":"   15","line":"    pub destinations: Vec<NodeT>,","class":"lineNoCov","hits":"0",},
{"lineNum":"   16","line":"    pub nodes_mapping: HashMap<String, NodeT>,","class":"lineNoCov","hits":"0",},
{"lineNum":"   17","line":"    pub nodes_reverse_mapping: Vec<String>,","class":"lineNoCov","hits":"0",},
{"lineNum":"   18","line":"    pub unique_edges: HashMap<(NodeT, NodeT), EdgeT>,","class":"lineNoCov","hits":"0",},
{"lineNum":"   19","line":"    pub outbounds: Vec<EdgeT>,","class":"lineNoCov","hits":"0",},
{"lineNum":"   20","line":"    pub weights: Option<Vec<WeightT>>,","class":"lineNoCov","hits":"0",},
{"lineNum":"   21","line":"    pub node_types: Option<Vec<NodeTypeT>>,","class":"lineNoCov","hits":"0",},
{"lineNum":"   22","line":"    pub node_types_mapping: Option<HashMap<String, NodeTypeT>>,","class":"lineNoCov","hits":"0",},
{"lineNum":"   23","line":"    pub node_types_reverse_mapping: Option<Vec<String>>,","class":"lineNoCov","hits":"0",},
{"lineNum":"   24","line":"    pub edge_types: Option<Vec<EdgeTypeT>>,","class":"lineNoCov","hits":"0",},
{"lineNum":"   25","line":"    pub edge_types_mapping: Option<HashMap<String, EdgeTypeT>>,","class":"lineNoCov","hits":"0",},
{"lineNum":"   26","line":"    pub edge_types_reverse_mapping: Option<Vec<String>>","class":"lineNoCov","hits":"0",},
{"lineNum":"   27","line":"}"},
{"lineNum":"   28","line":""},
{"lineNum":"   29","line":"impl Graph {"},
{"lineNum":"   30","line":""},
{"lineNum":"   31","line":"    pub fn compute_outbounds(nodes_number: NodeT, sources: &[NodeT]) -> Vec<EdgeT> {","class":"lineCov","hits":"1","order":"157",},
{"lineNum":"   32","line":"        info!(\"Computing outbound edges ranges from each node.\");","class":"lineCov","hits":"1","order":"158",},
{"lineNum":"   33","line":"        let mut last_src: NodeT = 0;","class":"lineCov","hits":"1","order":"138",},
{"lineNum":"   34","line":"        // Instead of fixing the last values after the loop, we set directly"},
{"lineNum":"   35","line":"        // all values to the length of the sources, which is the sum of all"},
{"lineNum":"   36","line":"        // possible neighbors."},
{"lineNum":"   37","line":"        let mut outbounds: Vec<EdgeT> = vec![sources.len(); nodes_number];","class":"lineCov","hits":"1","order":"139",},
{"lineNum":"   38","line":""},
{"lineNum":"   39","line":"        for (i, src) in sources.iter().enumerate() {","class":"lineCov","hits":"1","order":"160",},
{"lineNum":"   40","line":"            if last_src != *src {","class":"lineCov","hits":"1","order":"147",},
{"lineNum":"   41","line":"                // Assigning to range instead of single value, so that traps"},
{"lineNum":"   42","line":"                // have as delta between previous and next node zero."},
{"lineNum":"   43","line":"                for o in &mut outbounds[last_src..*src] {","class":"lineCov","hits":"1","order":"129",},
{"lineNum":"   44","line":"                    *o = i;","class":"lineCov","hits":"1","order":"148",},
{"lineNum":"   45","line":"                }","class":"lineCov","hits":"1","order":"140",},
{"lineNum":"   46","line":"                last_src = *src;","class":"lineCov","hits":"1","order":"130",},
{"lineNum":"   47","line":"            }"},
{"lineNum":"   48","line":"        }","class":"lineCov","hits":"1","order":"143",},
{"lineNum":"   49","line":""},
{"lineNum":"   50","line":"        outbounds"},
{"lineNum":"   51","line":"    }","class":"lineCov","hits":"1","order":"137",},
{"lineNum":"   52","line":""},
{"lineNum":"   53","line":"    pub fn get_node_type_id(&self, node_id:NodeT) -> Result<NodeTypeT, &str> {","class":"lineNoCov","hits":"0",},
{"lineNum":"   54","line":"        if let Some(nt) = &self.node_types{","class":"lineNoCov","hits":"0",},
{"lineNum":"   55","line":"            return Ok(nt[node_id]);","class":"lineNoCov","hits":"0",},
{"lineNum":"   56","line":"        }"},
{"lineNum":"   57","line":"        Err(\"Node types are not defined for current class.\")","class":"lineNoCov","hits":"0",},
{"lineNum":"   58","line":"    }","class":"lineNoCov","hits":"0",},
{"lineNum":"   59","line":""},
{"lineNum":"   60","line":"    pub fn get_edge_type_id(&self, edge_id:EdgeT) -> Result<EdgeTypeT, &str> {","class":"lineNoCov","hits":"0",},
{"lineNum":"   61","line":"        if let Some(et) = &self.edge_types{","class":"lineNoCov","hits":"0",},
{"lineNum":"   62","line":"            return Ok(et[edge_id]);","class":"lineNoCov","hits":"0",},
{"lineNum":"   63","line":"        }"},
{"lineNum":"   64","line":"        Err(\"Edge types are not defined for current class.\")","class":"lineNoCov","hits":"0",},
{"lineNum":"   65","line":"    }","class":"lineNoCov","hits":"0",},
{"lineNum":"   66","line":""},
{"lineNum":"   67","line":"    pub fn get_edge_id(&self, src:NodeT, dst:NodeT) -> EdgeT {","class":"lineNoCov","hits":"0",},
{"lineNum":"   68","line":"        *self.unique_edges.get(&(src, dst)).unwrap()","class":"lineNoCov","hits":"0",},
{"lineNum":"   69","line":"    }","class":"lineNoCov","hits":"0",},
{"lineNum":"   70","line":""},
{"lineNum":"   71","line":"    pub fn get_nodes_number(&self) -> usize {","class":"lineCov","hits":"1","order":"153",},
{"lineNum":"   72","line":"        self.nodes_reverse_mapping.len()","class":"lineCov","hits":"1","order":"155",},
{"lineNum":"   73","line":"    }","class":"lineCov","hits":"1","order":"120",},
{"lineNum":"   74","line":""},
{"lineNum":"   75","line":"    pub fn get_edges_number(&self) -> usize {","class":"lineNoCov","hits":"0",},
{"lineNum":"   76","line":"        self.sources.len()","class":"lineNoCov","hits":"0",},
{"lineNum":"   77","line":"    }","class":"lineNoCov","hits":"0",},
{"lineNum":"   78","line":""},
{"lineNum":"   79","line":"    pub fn get_edge_types_number(&self) -> usize {","class":"lineNoCov","hits":"0",},
{"lineNum":"   80","line":"        if let Some(etm) = &self.edge_types_mapping {","class":"lineNoCov","hits":"0",},
{"lineNum":"   81","line":"            etm.keys().len()","class":"lineNoCov","hits":"0",},
{"lineNum":"   82","line":"        } else {","class":"lineNoCov","hits":"0",},
{"lineNum":"   83","line":"            0","class":"lineNoCov","hits":"0",},
{"lineNum":"   84","line":"        }"},
{"lineNum":"   85","line":"    }","class":"lineNoCov","hits":"0",},
{"lineNum":"   86","line":""},
{"lineNum":"   87","line":"    pub fn get_node_types_number(&self) -> usize {","class":"lineNoCov","hits":"0",},
{"lineNum":"   88","line":"        if let Some(etm) = &self.node_types_mapping {","class":"lineNoCov","hits":"0",},
{"lineNum":"   89","line":"            etm.keys().len()","class":"lineNoCov","hits":"0",},
{"lineNum":"   90","line":"        } else {","class":"lineNoCov","hits":"0",},
{"lineNum":"   91","line":"            0","class":"lineNoCov","hits":"0",},
{"lineNum":"   92","line":"        }"},
{"lineNum":"   93","line":"    }","class":"lineNoCov","hits":"0",},
{"lineNum":"   94","line":""},
{"lineNum":"   95","line":"    fn get_min_max_edge(&self, node: NodeT) -> (EdgeT, EdgeT) {","class":"lineCov","hits":"1","order":"166",},
{"lineNum":"   96","line":"        let min_edge: EdgeT = if node == 0 {","class":"lineCov","hits":"1","order":"168",},
{"lineNum":"   97","line":"            0","class":"lineCov","hits":"1","order":"172",},
{"lineNum":"   98","line":"        } else {"},
{"lineNum":"   99","line":"            self.outbounds[node - 1]","class":"lineCov","hits":"1","order":"170",},
{"lineNum":"  100","line":"        };"},
{"lineNum":"  101","line":"        let max_edge: EdgeT = self.outbounds[node];","class":"lineCov","hits":"1","order":"173",},
{"lineNum":"  102","line":"        (min_edge, max_edge)","class":"lineCov","hits":"1","order":"175",},
{"lineNum":"  103","line":"    }","class":"lineCov","hits":"1","order":"177",},
{"lineNum":"  104","line":""},
{"lineNum":"  105","line":"    fn is_node_trap(&self, node: NodeT) -> bool {","class":"lineCov","hits":"1","order":"178",},
{"lineNum":"  106","line":"        let (_min, _max) = self.get_min_max_edge(node);","class":"lineCov","hits":"1","order":"179",},
{"lineNum":"  107","line":"        _min == _max","class":"lineCov","hits":"1","order":"180",},
{"lineNum":"  108","line":"    }","class":"lineCov","hits":"1","order":"181",},
{"lineNum":"  109","line":""},
{"lineNum":"  110","line":"    fn is_edge_trap(&self, edge: EdgeT) -> bool {","class":"lineCov","hits":"1","order":"182",},
{"lineNum":"  111","line":"        self.is_node_trap(self.destinations[edge])","class":"lineCov","hits":"1","order":"183",},
{"lineNum":"  112","line":"    }","class":"lineCov","hits":"1","order":"184",},
{"lineNum":"  113","line":""},
{"lineNum":"  114","line":"    fn get_node_transition(","class":"lineCov","hits":"1","order":"186",},
{"lineNum":"  115","line":"        &self,"},
{"lineNum":"  116","line":"        node: NodeT,"},
{"lineNum":"  117","line":"        change_node_type_weight: ParamsT,"},
{"lineNum":"  118","line":"    ) -> (Vec<WeightT>, Vec<NodeT>, EdgeT, EdgeT) {"},
{"lineNum":"  119","line":"        // Retrieve edge boundaries."},
{"lineNum":"  120","line":"        let (min_edge, max_edge) = self.get_min_max_edge(node);","class":"lineCov","hits":"1","order":"187",},
{"lineNum":"  121","line":"        // If weights are given"},
{"lineNum":"  122","line":"        let mut transition: Vec<WeightT> = if let Some(w) = &self.weights {","class":"lineCov","hits":"1","order":"189",},
{"lineNum":"  123","line":"            w[min_edge..max_edge].to_vec()","class":"lineCov","hits":"1","order":"190",},
{"lineNum":"  124","line":"        } else {"},
{"lineNum":"  125","line":"            vec![1.0; max_edge - min_edge]","class":"lineNoCov","hits":"0",},
{"lineNum":"  126","line":"        };"},
{"lineNum":"  127","line":""},
{"lineNum":"  128","line":"        let destinations: Vec<NodeT> = self.destinations[min_edge..max_edge].to_vec();","class":"lineCov","hits":"1","order":"192",},
{"lineNum":"  129","line":""},
{"lineNum":"  130","line":"        //############################################################"},
{"lineNum":"  131","line":"        //# Handling of the change node type parameter               #"},
{"lineNum":"  132","line":"        //############################################################"},
{"lineNum":"  133","line":""},
{"lineNum":"  134","line":"        if (change_node_type_weight  - 1.0).abs() > f64::EPSILON {","class":"lineCov","hits":"1","order":"194",},
{"lineNum":"  135","line":"            // If the node types were given:"},
{"lineNum":"  136","line":"            if let Some(nt) = &self.node_types {","class":"lineCov","hits":"1","order":"196",},
{"lineNum":"  137","line":"                // if the destination node type matches the neighbour"},
{"lineNum":"  138","line":"                // destination node type (we are not changing the node type)"},
{"lineNum":"  139","line":"                // we weigth using the provided change_node_type_weight weight."},
{"lineNum":"  140","line":"                let this_type: NodeTypeT = nt[node];","class":"lineNoCov","hits":"0",},
{"lineNum":"  141","line":""},
{"lineNum":"  142","line":"                transition","class":"lineNoCov","hits":"0",},
{"lineNum":"  143","line":"                    .iter_mut()"},
{"lineNum":"  144","line":"                    .zip(destinations.iter().map(|dst| nt[*dst]))","class":"lineNoCov","hits":"0",},
{"lineNum":"  145","line":"                    .filter(|(_, neigh_type)| this_type == *neigh_type)","class":"lineNoCov","hits":"0",},
{"lineNum":"  146","line":"                    .for_each(|(transition_value, _)| *transition_value /= change_node_type_weight);","class":"lineNoCov","hits":"0",},
{"lineNum":"  147","line":"                // credo non serva collect perche\' modifichiamo i valori direttamente"},
{"lineNum":"  148","line":"            }","class":"lineNoCov","hits":"0",},
{"lineNum":"  149","line":"        }"},
{"lineNum":"  150","line":"        (transition, destinations, min_edge, max_edge)","class":"lineCov","hits":"1","order":"198",},
{"lineNum":"  151","line":"    }","class":"lineCov","hits":"1","order":"193",},
{"lineNum":"  152","line":""},
{"lineNum":"  153","line":"    fn get_edge_transition(","class":"lineCov","hits":"1","order":"199",},
{"lineNum":"  154","line":"        &self,"},
{"lineNum":"  155","line":"        edge: EdgeT,"},
{"lineNum":"  156","line":"        return_weight: ParamsT,"},
{"lineNum":"  157","line":"        explore_weight: ParamsT,"},
{"lineNum":"  158","line":"        change_node_type_weight: ParamsT,"},
{"lineNum":"  159","line":"        change_edge_type_weight: ParamsT,"},
{"lineNum":"  160","line":"    ) -> (Vec<WeightT>, Vec<NodeT>, EdgeT, EdgeT) {"},
{"lineNum":"  161","line":"        // Get the source and destination for current edge."},
{"lineNum":"  162","line":"        let (src, dst) = (self.sources[edge], self.destinations[edge]);","class":"lineCov","hits":"1","order":"200",},
{"lineNum":"  163","line":""},
{"lineNum":"  164","line":"        // Compute the transition weights relative to the node weights."},
{"lineNum":"  165","line":"        let (mut transition, destinations, min_edge, max_edge) =","class":"lineCov","hits":"1","order":"201",},
{"lineNum":"  166","line":"            self.get_node_transition(dst, change_node_type_weight);","class":"lineCov","hits":"1","order":"161",},
{"lineNum":"  167","line":""},
{"lineNum":"  168","line":"        //############################################################"},
{"lineNum":"  169","line":"        //# Handling of the change edge type parameter               #"},
{"lineNum":"  170","line":"        //############################################################"},
{"lineNum":"  171","line":""},
{"lineNum":"  172","line":"        // If the edge types were given:"},
{"lineNum":"  173","line":"        if (change_edge_type_weight - 1.0).abs() > f64::EPSILON {","class":"lineCov","hits":"1","order":"125",},
{"lineNum":"  174","line":"            if let Some(et) = &self.edge_types {","class":"lineCov","hits":"1","order":"150",},
{"lineNum":"  175","line":"                //# If the neighbour edge type matches the previous"},
{"lineNum":"  176","line":"                //# edge type (we are not changing the edge type)"},
{"lineNum":"  177","line":"                //# we weigth using the provided change_edge_type_weight weight."},
{"lineNum":"  178","line":"                let this_type: EdgeTypeT = et[edge];","class":"lineNoCov","hits":"0",},
{"lineNum":"  179","line":"                transition","class":"lineNoCov","hits":"0",},
{"lineNum":"  180","line":"                    .iter_mut()"},
{"lineNum":"  181","line":"                    .zip(et[min_edge..max_edge].iter())","class":"lineNoCov","hits":"0",},
{"lineNum":"  182","line":"                    .filter(|(_, &neigh_type)| this_type == neigh_type)","class":"lineNoCov","hits":"0",},
{"lineNum":"  183","line":"                    .for_each(|(transition_value, _)| *transition_value /= change_edge_type_weight);","class":"lineNoCov","hits":"0",},
{"lineNum":"  184","line":"            }","class":"lineNoCov","hits":"0",},
{"lineNum":"  185","line":"        }"},
{"lineNum":"  186","line":""},
{"lineNum":"  187","line":"        //############################################################"},
{"lineNum":"  188","line":"        //# Handling of the P parameter: the return coefficient      #"},
{"lineNum":"  189","line":"        //############################################################"},
{"lineNum":"  190","line":""},
{"lineNum":"  191","line":"        //# If the neigbour matches with the source, hence this is"},
{"lineNum":"  192","line":"        //# a backward loop like the following:"},
{"lineNum":"  193","line":"        //# SRC -> DST"},
{"lineNum":"  194","line":"        //#  ▲     /"},
{"lineNum":"  195","line":"        //#   \\___/"},
{"lineNum":"  196","line":"        //#"},
{"lineNum":"  197","line":"        //# We weight the edge weight with the given return weight."},
{"lineNum":"  198","line":""},
{"lineNum":"  199","line":"        // If the return weight, which is the inverse of p, is not 1, hence"},
{"lineNum":"  200","line":"        // it has some impact, we procced and increase by the given weight"},
{"lineNum":"  201","line":"        // the probability of transitions that go back a previously visited"},
{"lineNum":"  202","line":"        // node."},
{"lineNum":"  203","line":"        if (return_weight  - 1.0).abs() > f64::EPSILON {","class":"lineCov","hits":"1","order":"119",},
{"lineNum":"  204","line":"            transition","class":"lineCov","hits":"1","order":"117",},
{"lineNum":"  205","line":"                .iter_mut()"},
{"lineNum":"  206","line":"                .zip(destinations.iter())","class":"lineCov","hits":"1","order":"116",},
{"lineNum":"  207","line":"                .filter(|&(_, ndst)| src == *ndst || dst == *ndst)","class":"lineCov","hits":"1","order":"114",},
{"lineNum":"  208","line":"                .for_each(|(transition_value, _)| *transition_value *= return_weight);","class":"lineCov","hits":"1","order":"111",},
{"lineNum":"  209","line":"        }"},
{"lineNum":"  210","line":"        //############################################################"},
{"lineNum":"  211","line":"        //# Handling of the Q parameter: the exploration coefficient #"},
{"lineNum":"  212","line":"        //############################################################"},
{"lineNum":"  213","line":""},
{"lineNum":"  214","line":"        if (explore_weight  - 1.0).abs() > f64::EPSILON {","class":"lineCov","hits":"1","order":"144",},
{"lineNum":"  215","line":"            transition","class":"lineCov","hits":"1","order":"109",},
{"lineNum":"  216","line":"                .iter_mut()"},
{"lineNum":"  217","line":"                .zip(destinations.iter())","class":"lineCov","hits":"1","order":"164",},
{"lineNum":"  218","line":"                .filter(|&(_, ndst)| (src != *ndst || dst == *ndst) && !self.unique_edges.contains_key(&(*ndst, src)))","class":"lineCov","hits":"1","order":"133",},
{"lineNum":"  219","line":"                .for_each(|(transition_value, _)| *transition_value *= explore_weight);","class":"lineCov","hits":"1","order":"132",},
{"lineNum":"  220","line":"        }"},
{"lineNum":"  221","line":""},
{"lineNum":"  222","line":"        (transition, destinations, min_edge, max_edge)","class":"lineCov","hits":"1","order":"167",},
{"lineNum":"  223","line":"    }","class":"lineCov","hits":"1","order":"169",},
{"lineNum":"  224","line":""},
{"lineNum":"  225","line":"    fn extract_node(&self, node: NodeT, change_node_type_weight: ParamsT) -> (NodeT, EdgeT) {","class":"lineCov","hits":"1","order":"202",},
{"lineNum":"  226","line":"        let (weights, dsts, min_edge, _) = self.get_node_transition(node, change_node_type_weight);","class":"lineCov","hits":"1","order":"171",},
{"lineNum":"  227","line":"        let index = sample(&weights);","class":"lineCov","hits":"1","order":"203",},
{"lineNum":"  228","line":"        (dsts[index], min_edge + index)","class":"lineCov","hits":"1","order":"174",},
{"lineNum":"  229","line":"    }","class":"lineCov","hits":"1","order":"176",},
{"lineNum":"  230","line":""},
{"lineNum":"  231","line":"    fn extract_edge(","class":"lineCov","hits":"1","order":"204",},
{"lineNum":"  232","line":"        &self,"},
{"lineNum":"  233","line":"        edge: EdgeT,"},
{"lineNum":"  234","line":"        return_weight: ParamsT,"},
{"lineNum":"  235","line":"        explore_weight: ParamsT,"},
{"lineNum":"  236","line":"        change_node_type_weight: ParamsT,"},
{"lineNum":"  237","line":"        change_edge_type_weight: ParamsT,"},
{"lineNum":"  238","line":"    ) -> (NodeT, EdgeT) {"},
{"lineNum":"  239","line":"        let (weights, dsts, min_edge, _) = self.get_edge_transition(","class":"lineCov","hits":"1","order":"185",},
{"lineNum":"  240","line":"            edge,"},
{"lineNum":"  241","line":"            return_weight,"},
{"lineNum":"  242","line":"            explore_weight,"},
{"lineNum":"  243","line":"            change_node_type_weight,"},
{"lineNum":"  244","line":"            change_edge_type_weight,"},
{"lineNum":"  245","line":"        );","class":"lineCov","hits":"1","order":"205",},
{"lineNum":"  246","line":"        let index = sample(&weights);","class":"lineCov","hits":"1","order":"206",},
{"lineNum":"  247","line":"        (dsts[index], min_edge + index)","class":"lineCov","hits":"1","order":"188",},
{"lineNum":"  248","line":"    }","class":"lineCov","hits":"1","order":"207",},
{"lineNum":"  249","line":""},
{"lineNum":"  250","line":"    pub fn walk(","class":"lineCov","hits":"1","order":"191",},
{"lineNum":"  251","line":"        &self,"},
{"lineNum":"  252","line":"        iterations: usize,"},
{"lineNum":"  253","line":"        length: usize,"},
{"lineNum":"  254","line":"        min_length: Option<usize>,"},
{"lineNum":"  255","line":"        return_weight: Option<ParamsT>,"},
{"lineNum":"  256","line":"        explore_weight: Option<ParamsT>,"},
{"lineNum":"  257","line":"        change_node_type_weight: Option<ParamsT>,"},
{"lineNum":"  258","line":"        change_edge_type_weight: Option<ParamsT>,"},
{"lineNum":"  259","line":"    ) -> Result<Vec<Vec<NodeT>>, String> {"},
{"lineNum":"  260","line":"        let _min_length = min_length.unwrap_or(0);","class":"lineCov","hits":"1","order":"208",},
{"lineNum":"  261","line":"        let _return_weight = return_weight.unwrap_or(1.0);","class":"lineCov","hits":"1","order":"195",},
{"lineNum":"  262","line":"        let _explore_weight = explore_weight.unwrap_or(1.0);","class":"lineCov","hits":"1","order":"209",},
{"lineNum":"  263","line":"        let _change_node_type_weight = change_node_type_weight.unwrap_or(1.0);","class":"lineCov","hits":"1","order":"197",},
{"lineNum":"  264","line":"        let _change_edge_type_weight = change_edge_type_weight.unwrap_or(1.0);","class":"lineCov","hits":"1","order":"210",},
{"lineNum":"  265","line":""},
{"lineNum":"  266","line":"        if _return_weight <= 0.0 {","class":"lineCov","hits":"1","order":"211",},
{"lineNum":"  267","line":"            return Err(","class":"lineNoCov","hits":"0",},
{"lineNum":"  268","line":"                String::from(","class":"lineNoCov","hits":"0",},
{"lineNum":"  269","line":"                    \"Given \'return_weight\' is not a strictly positive real number.\""},
{"lineNum":"  270","line":"                )"},
{"lineNum":"  271","line":"            );","class":"lineNoCov","hits":"0",},
{"lineNum":"  272","line":"        }"},
{"lineNum":"  273","line":"        if _explore_weight <= 0.0 {","class":"lineCov","hits":"1","order":"145",},
{"lineNum":"  274","line":"            return Err(","class":"lineNoCov","hits":"0",},
{"lineNum":"  275","line":"                String::from(","class":"lineNoCov","hits":"0",},
{"lineNum":"  276","line":"                    \"Given \'explore_weight\' is not a strictly positive real number.\""},
{"lineNum":"  277","line":"                )"},
{"lineNum":"  278","line":"            );","class":"lineNoCov","hits":"0",},
{"lineNum":"  279","line":"        }"},
{"lineNum":"  280","line":"        if _change_node_type_weight <= 0.0 {","class":"lineCov","hits":"1","order":"108",},
{"lineNum":"  281","line":"            return Err(","class":"lineNoCov","hits":"0",},
{"lineNum":"  282","line":"                String::from(","class":"lineNoCov","hits":"0",},
{"lineNum":"  283","line":"                    \"Given \'change_node_type_weight\' is not a strictly positive real number.\""},
{"lineNum":"  284","line":"                )"},
{"lineNum":"  285","line":"            );","class":"lineNoCov","hits":"0",},
{"lineNum":"  286","line":"        }"},
{"lineNum":"  287","line":"        if _change_edge_type_weight <= 0.0 {","class":"lineCov","hits":"1","order":"107",},
{"lineNum":"  288","line":"            return Err(","class":"lineNoCov","hits":"0",},
{"lineNum":"  289","line":"                String::from(","class":"lineNoCov","hits":"0",},
{"lineNum":"  290","line":"                    \"Given \'change_edge_type_weight\' is not a strictly positive real number.\""},
{"lineNum":"  291","line":"                )"},
{"lineNum":"  292","line":"            );","class":"lineNoCov","hits":"0",},
{"lineNum":"  293","line":"        }"},
{"lineNum":"  294","line":""},
{"lineNum":"  295","line":"        info!(\"Starting random walk.\");","class":"lineCov","hits":"1","order":"106",},
{"lineNum":"  296","line":"        let number_of_results = iterations * self.get_nodes_number();","class":"lineCov","hits":"1","order":"159",},
{"lineNum":"  297","line":""},
{"lineNum":"  298","line":"        Ok(","class":"lineCov","hits":"1","order":"104",},
{"lineNum":"  299","line":"            (0..number_of_results)","class":"lineCov","hits":"1","order":"105",},
{"lineNum":"  300","line":"                .into_par_iter()","class":"lineCov","hits":"1","order":"128",},
{"lineNum":"  301","line":"                .map(|node| {","class":"lineCov","hits":"1","order":"149",},
{"lineNum":"  302","line":"                    self.single_walk(","class":"lineCov","hits":"1","order":"141",},
{"lineNum":"  303","line":"                        length,","class":"lineCov","hits":"1","order":"131",},
{"lineNum":"  304","line":"                        node / iterations,","class":"lineCov","hits":"1","order":"127",},
{"lineNum":"  305","line":"                        _return_weight,","class":"lineCov","hits":"1","order":"142",},
{"lineNum":"  306","line":"                        _explore_weight,","class":"lineCov","hits":"1","order":"146",},
{"lineNum":"  307","line":"                        _change_node_type_weight,","class":"lineCov","hits":"1","order":"124",},
{"lineNum":"  308","line":"                        _change_edge_type_weight","class":"lineCov","hits":"1","order":"136",},
{"lineNum":"  309","line":"                    )"},
{"lineNum":"  310","line":"                })","class":"lineCov","hits":"1","order":"151",},
{"lineNum":"  311","line":"                .filter(|walk| walk.len() >= _min_length)","class":"lineCov","hits":"1","order":"123",},
{"lineNum":"  312","line":"                .collect::<Vec<Vec<NodeT>>>()","class":"lineCov","hits":"1","order":"121",},
{"lineNum":"  313","line":"        )","class":"lineCov","hits":"1","order":"103",},
{"lineNum":"  314","line":"    }","class":"lineCov","hits":"1","order":"122",},
{"lineNum":"  315","line":""},
{"lineNum":"  316","line":"    fn single_walk(","class":"lineCov","hits":"1","order":"102",},
{"lineNum":"  317","line":"        &self,"},
{"lineNum":"  318","line":"        length: usize,"},
{"lineNum":"  319","line":"        node: NodeT,"},
{"lineNum":"  320","line":"        return_weight: ParamsT,"},
{"lineNum":"  321","line":"        explore_weight: ParamsT,"},
{"lineNum":"  322","line":"        change_node_type_weight: ParamsT,"},
{"lineNum":"  323","line":"        change_edge_type_weight: ParamsT,"},
{"lineNum":"  324","line":"    ) -> Vec<NodeT> {"},
{"lineNum":"  325","line":"        let mut walk: Vec<NodeT> = Vec::with_capacity(length);","class":"lineCov","hits":"1","order":"126",},
{"lineNum":"  326","line":"        walk.push(node);","class":"lineCov","hits":"1","order":"135",},
{"lineNum":"  327","line":""},
{"lineNum":"  328","line":"        if self.is_node_trap(node) {","class":"lineCov","hits":"1","order":"152",},
{"lineNum":"  329","line":"            return walk;","class":"lineCov","hits":"1","order":"154",},
{"lineNum":"  330","line":"        }"},
{"lineNum":"  331","line":""},
{"lineNum":"  332","line":"        let (dst, mut edge) = self.extract_node(node, change_node_type_weight);","class":"lineCov","hits":"1","order":"156",},
{"lineNum":"  333","line":"        walk.push(dst);","class":"lineCov","hits":"1","order":"118",},
{"lineNum":"  334","line":""},
{"lineNum":"  335","line":"        for _ in 2..length {","class":"lineCov","hits":"1","order":"101",},
{"lineNum":"  336","line":"            if self.is_edge_trap(edge) {","class":"lineCov","hits":"1","order":"115",},
{"lineNum":"  337","line":"                break;","class":"lineCov","hits":"1","order":"113",},
{"lineNum":"  338","line":"            }"},
{"lineNum":"  339","line":"            let (dst, inner_edge) = self.extract_edge(","class":"lineCov","hits":"1","order":"110",},
{"lineNum":"  340","line":"                edge,","class":"lineCov","hits":"1","order":"112",},
{"lineNum":"  341","line":"                return_weight,"},
{"lineNum":"  342","line":"                explore_weight,"},
{"lineNum":"  343","line":"                change_node_type_weight,"},
{"lineNum":"  344","line":"                change_edge_type_weight,"},
{"lineNum":"  345","line":"            );"},
{"lineNum":"  346","line":"            edge = inner_edge;","class":"lineCov","hits":"1","order":"162",},
{"lineNum":"  347","line":"            walk.push(dst);","class":"lineCov","hits":"1","order":"163",},
{"lineNum":"  348","line":"        }","class":"lineCov","hits":"1","order":"134",},
{"lineNum":"  349","line":"        walk","class":"lineCov","hits":"1","order":"100",},
{"lineNum":"  350","line":"    }","class":"lineCov","hits":"1","order":"165",},
{"lineNum":"  351","line":"}"},
]};
var percent_low = 25;var percent_high = 75;
var header = { "command" : "only_edges", "date" : "2020-06-22 09:48:42", "instrumented" : 180, "covered" : 112,};
var merged_data = [];

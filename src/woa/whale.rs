use core::f64;
use std::f64::consts::PI;

use rand::{Rng, rngs::StdRng, seq::IteratorRandom};

use crate::entity::{graph::Graph, tree::Tree};

#[derive(Debug,Clone)]
pub struct Whale {
    position : Vec<f64>,
    nodes : Vec<(String,bool)>,
    pub tree : Tree,
    pub cost : f64,
    size: usize,
    pub lb : f64,
    pub ub : f64
} 

impl Whale {
    pub fn calculate_value(x : f64) -> f64{
        // T(v) = | (2/PI) * atan( (PI/2) * velocity ) |
        let inner_val = (PI / 2.0) * x;
        let probability = (2.0 / PI) * inner_val.atan();
        probability.abs()
    }

    pub fn new(graph : &Graph, lb : f64 , ub : f64, random : &mut StdRng, k : usize) -> Self {
        let size = graph.get_num_nodes();
        let mut position = vec![lb + random.gen_range(0.0..=1.0) * (ub -lb);size];
        let nodes = graph.get_nodes();
        let mut nodes_tree_ref : Vec<(String,bool)> = nodes.iter().map(|n| (n.clone(), false)).collect();
        let mut nodes_tree: Vec<(String, bool)> = vec![];
        let mut k_element = 1;
        while k_element <= k {
            let i = random.gen_range(0..size);
            if nodes_tree_ref[i].1 {
                continue;
            }

            let limit = random.gen_range(0.0..1.0);
            nodes_tree_ref[i].1 = Whale::calculate_value(position[i]) > limit;
            if nodes_tree_ref[i].1 {
                nodes_tree.push((nodes_tree_ref[i].0.clone(), false));
                k_element +=1;
            } else {
                position[i] = lb + random.gen_range(0.0..=1.0) * (ub -lb)
            }
        }

        let mut tree = graph.generate_tree_by_nodes(k, &mut nodes_tree);
        let cost = tree.get_cost(graph);

        Self {
            position : position,
            nodes : nodes_tree_ref.clone(),
            tree : tree.clone(),
            cost,
            size : size,
            lb,
            ub
        }
    }

    pub fn get_cost(&mut self, graph : &Graph) -> f64 {
        self.tree.get_cost(graph)
    }

    pub fn get_tree(&self) -> Vec<(String, String, f64)> {
        self.tree.get_edges()
    }

    pub fn get_tree_struct(&self) -> Tree {
        self.tree.clone()
    }

    pub fn get_len_position(&self) -> usize {
        self.size
    }

    pub fn get_position(&self, index : usize) -> f64 {
        self.position[index]
    } 


    pub fn get_node(&self, index : usize) -> (String,bool) {
        self.nodes[index].clone()
    }

    pub fn set_position(&mut self, index : usize, value : f64) {
        self.position[index] = value
    }

    pub fn set_node(&mut self, index : usize, in_tree : bool) {
        self.nodes[index].1 = in_tree
    }

    pub fn get_index_node_in_tree(&self, random : &mut StdRng) -> usize {
        let mut index = random.gen_range(0..self.size);
        if self.nodes.iter().all(|(_, in_tree)| !in_tree) { return index; } 
        while !self.nodes[index].1 {
            index = random.gen_range(0..self.size);
            if self.tree.nodes.contains(&self.nodes[index].0) {
                break;
            }
        }
        index
    }

    pub fn get_index_node_in_other_tree(&self, random : &mut StdRng, other_tree : &Tree) -> usize {
        let nodes_other_tree = &other_tree.nodes;
        let nodes_self_tree = &self.tree.nodes;
        let difference_iter = nodes_self_tree.difference(&nodes_other_tree);
        let mut candidates: Vec<String> = difference_iter.cloned().collect();
        candidates.sort(); 

        if candidates.is_empty() {
            return self.get_index_node_in_tree(random)
        };

        let difference_iter = candidates.iter();

        let element: String = match difference_iter.cloned().choose(random) {
            Some(element) => element,
            None => {
                return self.get_index_node_in_tree(random)
            }
        };

        match self.nodes.binary_search_by_key(&element, |(node_name, _)| {
            node_name.to_string()
        }) {
            Ok(index) => {
                index
            },
            Err(_) => {
                self.get_index_node_in_tree(random)
            }
        }
    
    }

    pub fn get_index_node_nin_tree(&self, random : &mut StdRng) -> usize {
        let mut index = random.gen_range(0..self.size);
        if self.nodes.iter().all(|(_, in_tree)| *in_tree) { return index; } 
        while self.nodes[index].1 {
            index = random.gen_range(0..self.size);
            if !self.tree.nodes.contains(&self.nodes[index].0) {
                break;
            }
        }
        index
    }

}
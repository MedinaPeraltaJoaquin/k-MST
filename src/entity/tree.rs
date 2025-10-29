use super::graph::Graph;
use std::collections::{BinaryHeap, HashSet, HashMap};

use ordered_float::OrderedFloat;
//use std::cmp::Reverse;

/// Representa un sub-árbol de `k` nodos dentro del grafo original, utilizado en un algoritmo de búsqueda local.
#[derive(Debug)]
pub struct Tree {
    /// Conjunto de nodos que componen el sub-árbol.
    pub nodes: HashSet<String>,
    /// Las aristas que forman el árbol (k-1 aristas).
    pub edges: Vec<(String, String, f64)>, //k-1 edges
    /// El costo total del árbol. Se cachea. -1.0 si no se ha calculado.
    pub total_cost: f64,
    /// Factor de normalización del costo. Se cachea. -1.0 si no se ha calculado.
    pub normalize : f64,
    /// Caché del vecino, que guarda: (nuevas aristas, costo, nodo_nuevo, nodo_removido).
    pub neighbors: (Vec<(String,String,f64)>, f64, String, String),
    /// El número de nodos del árbol (el parámetro 'k').
    pub k: usize,
}

impl Tree {
    /// Crea una nueva instancia de `Tree`.
    pub fn new(edges: Vec<(String, String, f64)>, nodes : HashSet<String>, k : usize) -> Self {
        Tree { 
            edges, 
            total_cost : -1.0, 
            nodes , 
            normalize: -1.0, 
            neighbors: (vec![], -1.0, String::new(), String::new()), 
            k 
        }
    }

    /// Obtiene una lista de los nombres de los nodos en el árbol.
    pub fn get_nodes(&self) -> Vec<String> {
        self.nodes.clone().into_iter().collect()
    }

    /// Comprueba si un nodo específico está incluido en el árbol.
    pub fn is_node_in_tree(&self, node : String) -> bool{
        self.nodes.contains(&node)
    }

    /// Calcula un árbol vecino intercambiando un nodo existente por un `new_node`.
    ///
    /// La lógica de `retain` en `edges_new_tree` sugiere que solo se mantienen las aristas
    /// conectadas al nodo que se va a remover, lo cual es inusual para Prim. 
    /// **Nota:** Se mantiene la implementación original.
    pub fn get_neighbor(&mut self, 
        graph : &Graph, 
        new_node : &String,
        remove_node : &String
    ) -> Result<
            &(Vec<(String,String,f64)>, f64, String, String),
            ()> {

        // Devuelve el vecino cacheadao si ya se calculó.
        if self.neighbors.1 != -1.0 && 
           self.neighbors.3.eq(remove_node) && 
           self.neighbors.2.eq(new_node){
            return Ok(&self.neighbors);
        }

        if self.nodes.contains(new_node) {
            return Err(());
        }

        let mut nodes_new_tree = self.nodes.clone();
        nodes_new_tree.insert(new_node.clone());
        nodes_new_tree.remove(remove_node);

        // Se mantienen las aristas conectadas al nodo a remover.
        let mut edges_new_tree = self.edges.clone();
        edges_new_tree.retain(|(n1, n2 , _)| 
            *n1 != remove_node.clone() && *n2 != remove_node.clone() );

        // Guardamos nodos que esten en edges_new_tree como (n,1)
        let mut nodes_in_edges : HashMap<String, bool> = HashMap::new();
        for (n1, n2, _) in &edges_new_tree {
            nodes_in_edges.insert(n1.clone(), true);
            nodes_in_edges.insert(n2.clone(), true);
        }

        // Guardamos nodos restantes como (n,0)
        for n in nodes_new_tree {
            if !nodes_in_edges.contains_key(&n) {
                nodes_in_edges.insert(n, false);
            }
        }

        // Prepara la entrada para `graph.prim`.
        let mut nodes_prim_input = nodes_in_edges.into_iter().collect::<Vec<(String, bool)>>();

        // Ejecuta Prim sobre el subconjunto de nodos con las aristas preservadas.
        let new_tree = graph.prim(
            &mut nodes_prim_input, 
            edges_new_tree.clone(), 
            self.k);
        
        let cost = self.get_cost_raw(graph, &new_tree);
        self.neighbors = (new_tree, cost, new_node.clone(), remove_node.clone());

        Ok(&self.neighbors)
    }

    /// Limpia el vecino cacheado.
    pub fn clear_neighbour(&mut self) {
        self.neighbors = (vec![], -1.0, String::new(), String::new());
    }

    /// Acepta el árbol vecino y lo convierte en el árbol actual.
    pub fn recover_solution(&mut self) -> bool {
        if self.neighbors.1 == -1.0 {
            return false;
        }

        // Actualiza el estado del árbol con los datos del vecino.
        self.edges = self.neighbors.0.clone();
        self.total_cost = self.neighbors.1;
        self.nodes.insert(self.neighbors.2.clone()); // Añade el nodo nuevo
        self.nodes.remove(&self.neighbors.3);       // Remueve el nodo viejo
        self.clear_neighbour(); // Limpia la caché.

        true
    }

    /// Obtiene el costo total del árbol, cacheando el resultado.
    pub fn get_cost(&mut self, graph: &Graph) -> f64 {
        if self.total_cost != -1.0 {
            return self.total_cost;
        }

        let edges_clone = self.edges.clone();
        let cost = self.get_cost_raw(graph, &edges_clone);
        self.total_cost = cost;
        cost
    }

    /// Calcula el costo total sin usar la caché. `cost = (suma_pesos) / (factor_normalizacion)`
    pub fn get_cost_raw(&mut self, graph: &Graph, edges: &[(String, String, f64)]) -> f64 {
        let sum_edge : f64 = edges.iter().map(|(_,_,w)| *w).sum();
        let total_cost = sum_edge / self.get_normalize(graph);
        total_cost
    }

    /// Calcula y cachea el factor de normalización.
    pub fn get_normalize(&mut self, graph: &Graph) -> f64 {
        if self.normalize != -1.0 {
            return self.normalize;
        }

        // Usamos un Min-Heap (`Reverse` con `OrderedFloat`) para mantener las aristas más grandes.
        let mut heap: BinaryHeap<OrderedFloat<f64>> = BinaryHeap::new();
        let n = graph.get_num_nodes();
        let k_minus_1 = self.k - 1; // Usar k-1 aristas

        for i in 0 .. n { 
            for j in i + 1 .. n { // Asegura que j > i
                let index = i * n + j; // Calcula el índice plano para M[i][j]
                let edge = graph.get_edge_index(index);
                
                // Solo consideramos aristas originales (marcado con 1)
                let is_original_edge = edge.1 == 1; 
                // La condición edge.0 > 0.0 es implícita para j > i, pero se mantiene por claridad.
                let is_not_self_loop = edge.0 > 0.0; 

                if is_original_edge && is_not_self_loop {
                    // Usamos el peso de la arista M[i][j].
                    heap.push(OrderedFloat(edge.0));
                }
            }
        }   
        
        // Extraemos las k-1 más grandes.
        let mut sum_of_max_edges = 0.0;
        for _ in 0..k_minus_1 {
            if let Some(OrderedFloat(weight)) = heap.pop() {
                sum_of_max_edges += weight;
            } else {
                // Si el grafo tiene menos de k-1 aristas originales, detenemos.
                break;
            }
        }

        self.normalize = sum_of_max_edges;
        self.normalize
    }
}
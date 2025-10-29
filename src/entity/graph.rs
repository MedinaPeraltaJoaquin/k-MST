use std::collections::HashMap;
use std::f64;
use rand::{Rng, rngs::StdRng};
use std::collections::BinaryHeap;


use crate::entity::tree::Tree;
use crate::entity::edge::Edge;

/// Estructura auxiliar para calcular y ajustar los costos de las aristas.
#[derive(Debug)]
struct Cost{
    /// Matriz de distancias más cortas (Floyd-Warshall), almacenada de forma plana.
    floyd_warshall_matrix : Vec<f64>,
}

impl Cost {
    /// Crea una nueva instancia de `Cost`.
    pub fn new() -> Self {
        Cost { floyd_warshall_matrix: vec![] }
    }

    /// Calcula los costos ajustados de las aristas del grafo o ajusta los pesos de la matriz
    /// usando la distancia de Floyd-Warshall y el diámetro del grafo.
    pub fn calculate_edges(
        &mut self, 
        nodes: &HashMap<String, usize>, 
        edges: &mut Vec<(f64, usize)>
    ) -> Vec<(f64, usize)> {
        let floyd_warshall_matrix = self.get_floyd_warshall_matrix(nodes, edges);
        let diameter = self.get_diameter();
        for i in 0..edges.len() {
            // Ajusta las entradas de la matriz que no son aristas originales (marcado con 0).
            if edges[i].1 == 0 {
                if floyd_warshall_matrix[i].is_infinite() {
                    // Si no hay camino, asigna un costo muy alto (diámetro^2).
                    edges[i].0 = diameter * diameter;
                    continue;
                }
                // Si hay camino, ajusta el costo: distancia_mas_corta * diametro.
                edges[i].0 = floyd_warshall_matrix[i] * diameter;
            }
        }

        edges.clone()
    }

    /// Calcula la matriz de distancias más cortas entre todos los pares de nodos
    /// utilizando el algoritmo de Floyd-Warshall.
    fn get_floyd_warshall_matrix(
        &mut self, 
        nodes: &HashMap<String, usize>, 
        edges: &Vec<(f64, usize)>
    ) -> Vec<f64> {
        if !self.floyd_warshall_matrix.is_empty() {
            return self.floyd_warshall_matrix.clone();
        }   
        
        let size = nodes.len();
        // Inicializa la matriz de Floyd-Warshall.
        self.floyd_warshall_matrix = edges.iter().map(|&(cost, _)| cost).collect();

        // Implementación de Floyd-Warshall
        for k in 0..size {
            for i in 0..size {
                for j in 0..size{
                    let ik = self.floyd_warshall_matrix[i * size + k];
                    let kj = self.floyd_warshall_matrix[k * size + j];
                    let ij = self.floyd_warshall_matrix[i * size + j];
                    if ik.is_finite() && kj.is_finite() && ik + kj < ij {
                        self.floyd_warshall_matrix[i * size + j] = ik + kj;
                    }   
                }
            }
        }

        self.floyd_warshall_matrix.clone()
    }

    /// Calcula el diámetro del grafo (la distancia más larga finita en la matriz).
    fn get_diameter(&self) -> f64 {
        let mut diameter = 0.0;
        for &cost in &self.floyd_warshall_matrix {
            if cost.is_finite() && cost > diameter {
                diameter = cost;
            }
        }
        diameter
    }
}

/// Representa un grafo no dirigido con pesos de arista ajustados por distancias más cortas.
#[derive(Debug)]
pub struct Graph {
    /// Mapeo de nombres de nodos (String) a sus índices (usize).
    nodes : HashMap<String, usize>,
    /// Matriz de adyacencia/distancias. Almacenada como un vector plano (aplanado).
    /// Cada elemento es `(peso_o_distancia_ajustada, es_arista_original)`.
    edges : Vec<(f64,usize)>,
}

impl Graph {
    /// Crea una nueva instancia de `Graph` a partir de una lista 
    /// de aristas iniciales.
    pub fn new(edges : Vec<(String,String,f64)>) -> Self {
        let mut nodes : HashMap<String, usize> = HashMap::new();
        let mut num_nodes : usize = 0;

        // 1. Asignar índices a los nodos.
        for (n1,n2, _) in edges.clone() {
            if !nodes.contains_key(&n1) {
                nodes.insert(n1, num_nodes);
                num_nodes += 1;
            }
            if !nodes.contains_key(&n2) {
                nodes.insert(n2, num_nodes);
                num_nodes += 1;
            }
        }

        let size = nodes.len();
        // 2. Inicializar la matriz con INFINITY y 0.0 para auto-bucles.
        let mut weights: Vec<(f64, usize)> = 
                            vec![(f64::INFINITY, 0); size * size];
        for i in 0..size {
            weights[i * size + i] = (0.0, 1);
        }

        // 3. Rellenar con las aristas de entrada.
        for (n1,n2,w) in edges {
            let i = *nodes.get(&n1).unwrap();
            let j = *nodes.get(&n2).unwrap();
            weights[i * size + j] = (w, 1);
            weights[j * size + i] = (w, 1); 
        }

        // 4. Calcular y ajustar los pesos finales.
        let mut cost_calculator = Cost::new();
        weights = cost_calculator.calculate_edges(&nodes, &mut weights);

        Graph { nodes, edges: weights }
    }

    /// Obtiene el número total de nodos.
    pub fn get_num_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Obtiene una lista con los nombres de todos los nodos.
    pub fn get_nodes(&self) -> Vec<String> {
        let mut sorted_nodes : Vec<(String, usize)> = self.nodes
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
            
        sorted_nodes.sort_by_key(|(_,v)| *v);

        sorted_nodes.into_iter().map(|(k, _)| k).collect::<Vec<String>>()
    }

    /// Obtiene el peso/distancia ajustada de la arista entre dos nodos.
    pub fn get_edge(&self, node_a: &String, node_b: &String) -> &(f64, usize) {
        let idx1 = self.nodes.get(node_a).unwrap();
        let idx2 = self.nodes.get(node_b).unwrap();
        &self.edges[idx1 * self.get_num_nodes() + idx2]
    }

    /// Obtiene el peso/distancia ajustada de la arista en un índice plano del vector de aristas.
    pub fn get_edge_index(&self, index: usize) -> &(f64,usize) {
        &self.edges[index]
    }

    /// Genera un sub-árbol inicial de `k` nodos seleccionados aleatoriamente.
    pub fn generate_tree(&self, k: usize, random: &mut StdRng) -> Tree {
        let mut nodes = self.get_nodes();
        let mut nodes_tree = vec![(String::new(), false); k];
        // Selecciona 'k' nodos aleatorios.
        for i in 0..k {
            let index = random.gen_range(0..nodes.len());
            nodes_tree[i] = (nodes.remove(index).clone(), false);
        }

        self.generate_tree_by_nodes(k, &mut nodes_tree)
    }

    /// Genera un sub-árbol de `k` nodos a partir de una lista de nodos preseleccionados.
    pub fn generate_tree_by_nodes(&self, k: usize, nodes_tree : &mut Vec<(String,bool)>) -> Tree {
        let edges_tree = self.prim(nodes_tree, vec![], k);
        let nodes_set = nodes_tree.iter().map(|(n, _)| n.clone()).collect();
        Tree::new(edges_tree, nodes_set, k)
    }

    /// Implementa el algoritmo de Prim para encontrar el Árbol de Expansión Mínima (MST)
    /// sobre un subconjunto de nodos, utilizando los costos ajustados del grafo.
    pub fn prim(
        &self, 
        nodes_tree: &mut Vec<(String,bool)>, 
        edges : Vec<(String,String,f64)>, 
        size : usize
    ) -> Vec<(String,String,f64)> {
        let mut mst_edges = edges.clone();
        let mut edge_heap: BinaryHeap<Edge> = BinaryHeap::new();
        let k_nodes = size;

        // --- INICIO: Lógica de Inicialización Corregida ---
        let mut nodes_to_expand: Vec<usize> = Vec::new();
        let mut any_node_visited = false;
        for i in 0..k_nodes {
            if nodes_tree[i].1 {
                nodes_to_expand.push(i);
                any_node_visited = true;
            }
        }

        if !any_node_visited && k_nodes > 0 {
            nodes_tree[0].1 = true; 
            nodes_to_expand.push(0);
        }

        // Inicializar el heap con todas las aristas salientes de los nodos ya visitados/en el árbol.
        for &i in &nodes_to_expand {
            let from_node_name = &nodes_tree[i].0;
            
            for j in 0..k_nodes {
                if !nodes_tree[j].1 { // Si el nodo 'j' NO está en el árbol
                    let to_node_name = &nodes_tree[j].0;
                    let (weight, _) = self.get_edge(from_node_name, to_node_name);
                    
                    // Edge guarda: weight, índice_desde_en_nodes_tree (i), índice_a_en_nodes_tree (j)
                    edge_heap.push(Edge::new(*weight, i, j));
                }
            }
        }
        // --- FIN: Lógica de Inicialización Corregida ---

        while mst_edges.len() < size - 1 && !edge_heap.is_empty() {
            let edge = match edge_heap.pop() {
                Some(e) => e,
                None => break,
            };

            let from_node_tree_idx = edge.get_from();
            let to_node_tree_idx = edge.get_to();

            // En Prim, solo nos importa si el nodo 'to' ya está en el MST
            if nodes_tree[to_node_tree_idx].1 {
                continue;
            }
            
            // Marcar el nodo 'to' como visitado
            nodes_tree[to_node_tree_idx].1 = true;

            // Añadir la arista al MST
            mst_edges.push(
                (
                    nodes_tree[from_node_tree_idx].0.clone(),
                    nodes_tree[to_node_tree_idx].0.clone(),
                    edge.get_weight(),
                )
            );

            // Expandir: Añadir las nuevas aristas del nodo recién agregado a los no visitados
            let new_node_tree = nodes_tree[to_node_tree_idx].0.clone();
            let new_node_tree_idx = to_node_tree_idx.clone();
            //let new_node_graph_idx = *self.nodes.get(&nodes_tree[new_node_tree_idx].0).unwrap();
            
            for i in 0..nodes_tree.len() {
                if !nodes_tree[i].1 {
                    let (weight, _) = self.get_edge(&new_node_tree, &nodes_tree[i].0);
                    // Los índices de la arista del heap son los índices internos de `nodes_tree`
                    edge_heap.push(Edge::new(*weight, new_node_tree_idx, i));
                }
            }
        }

        mst_edges
    }
}
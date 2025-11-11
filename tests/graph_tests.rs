#[cfg(test)]

mod test {
    // Importa las dependencias necesarias. Asume que 'tu_proyecto' es el nombre de tu crate.
    use k_mst::entity::graph::Graph;
    use k_mst::entity::tree::Tree;
    use k_mst::utils::read_input::ReadInput;
    use std::path::PathBuf;

    use std::{collections::HashMap};
    use rand::{SeedableRng, rngs::StdRng, Rng};

    // Helper para crear un grafo de prueba.
    // Grafo simple de 4 nodos: A, B, C, D, E, F
    // Diámetro: 12.0 (e.g., A->D)
    fn setup_graph() -> Graph {
        let edges = vec![
            ("A".to_string(), "B".to_string(), 3.0),
            ("A".to_string(), "C".to_string(), 5.0),
            ("A".to_string(), "D".to_string(), 1.0),
            ("B".to_string(), "E".to_string(), 9.0),
            ("C".to_string(), "D".to_string(), 7.0),
            ("C".to_string(), "E".to_string(), 7.0),
            ("C".to_string(), "F".to_string(), 1.0),
            ("D".to_string(), "F".to_string(), 4.0),
        ];
        Graph::new(edges, 2)
    }

    fn generate_tree(graph : &Graph, k : usize, random: &mut StdRng) -> Tree {
        let mut nodes = graph.get_nodes();
        let mut nodes_tree = vec![(String::new(), false); k];
        for i in 0..k {
            let size = nodes.len();
            let index = random.gen_range(0..size);
            nodes_tree[i] = (nodes.remove(index).clone(), false);
        }

        graph.generate_tree_by_nodes(k, &mut nodes_tree)
    }

    #[test]
    /// Prueba la creación de la estructura `Graph` y el mapeo de nodos.
    fn test_graph_new_and_node_mapping() {
        let graph = setup_graph();
        
        assert_eq!(graph.get_num_nodes(), 6, "El grafo debe tener 6 nodos.");
        
        let nodes = graph.get_nodes();
        assert!(nodes.contains(&"A".to_string()));
        assert!(nodes.contains(&"D".to_string()));
    }

    #[test]
    /// Prueba el método `get_edge` y la corrección de las distancias ajustadas
    /// (resultado del algoritmo de Floyd-Warshall y el factor de diámetro).
    fn test_graph_edge_retrieval_and_floyd_adjustment() {
        let graph = setup_graph();
        let size = graph.get_num_nodes();
        let nodes_map: HashMap<String, usize> = graph.get_nodes().into_iter().zip(0..size).collect();
        print!("{:?}",nodes_map);
        
        let (distance_a_d, _) = graph.get_edge(&"A".to_string(), &"D".to_string());
        assert_eq!(*distance_a_d, 1.0, "Distancia A-D ajustada debe ser 1.0.");

        let (distance_d_e, _) = graph.get_edge(&"D".to_string(), &"E".to_string());
        assert_eq!(*distance_d_e, 144.0, "Distancia D-E ajustada debe ser 144.0.");
    }


    #[test]
    /// Prueba el algoritmo de Prim para encontrar un MST en un subconjunto de nodos.
    fn test_graph_prim_mst_calculation() {
        let graph = setup_graph();
        
        // Subconjunto de nodos para el árbol: {A, C, D} (k=3)
        let mut nodes_tree = vec![
            ("A".to_string(), false), 
            ("C".to_string(), false), 
            ("D".to_string(), false), 
        ];
        let k = 3;

        let mut mst_edges = graph.prim(&mut nodes_tree, vec![], k);
        
        assert_eq!(mst_edges.len(), 2, "El MST debe tener k-1 aristas (2).");

        let costs: Vec<f64> = mst_edges.iter().map(|(_, _, w)| *w).collect();
        assert!(costs.contains(&5.0) && costs.contains(&1.0), "El MST debe contener los costos ajustados 5.0 y 1.0.");

        nodes_tree = vec![
            ("A".to_string(), true), 
            ("F".to_string(), true), 
            ("E".to_string(), false),
        ];

        mst_edges = graph.prim(&mut nodes_tree, vec![("A".to_string(),"F".to_string(),60.0)], k);
        assert_eq!(mst_edges.len(), 2, "El MST debe tener k-1 aristas (2).");

        let costs: Vec<f64> = mst_edges.iter().map(|(_, _, w)| *w).collect();
        assert!(costs.contains(&60.0) && costs.contains(&96.0), "El MST debe contener los costos ajustados 60.0 y 96.0.");
    }

    #[test]
    /// Prueba la generación de un árbol aleatorio con un generador determinista.
    fn test_graph_generate_tree_deterministic() {
        let graph = setup_graph();
        let k = 2; // Queremos un árbol de 2 nodos.
        let seed = 42 as u64;
        // Usar una semilla fija para la reproducibilidad.
        let mut rng = StdRng::seed_from_u64(seed);

        let tree: Tree = generate_tree(&graph,k, &mut rng);

        assert_eq!(tree.nodes.len(), k);
        assert_eq!(tree.edges.len(), 1);
        
        // La única arista en el MST de {E, D} es E-D con costo ajustado 144.0.
        assert_eq!(tree.edges[0].2, 144.0, "El costo de la arista E-D debe ser el ajustado (144.0).");
    }

    #[test]
    fn test_graph_instance_with_known_data() {
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("tests");
        file_path.push("graph.txt");

        let file_str = file_path.to_str().expect("Fallo al convertir la ruta a string.");
        
        let args = vec![
            "program".to_string(), 
            "-p".to_string(), 
            file_str.to_string(),
            "-s".to_string(), 
            "42".to_string(),
            "-k".to_string(),
            "40".to_string(),
        ];
        
        let mut result = ReadInput::new(args).unwrap();
        let vec = result.read_file().unwrap();
        
        let seed = result.get_seed().unwrap(); // Semilla fija
        let mut rng = StdRng::seed_from_u64(seed[0] as u64);

        let k: usize = result.get_k_nodes().unwrap(); // Obtener k
        let graph = Graph::new(vec, k);
        let diameter = graph.get_diameter();
        let mut tree : Tree = generate_tree(&graph,k, &mut rng);
        let normalize = tree.get_normalize(&graph);

        let expected_normalize = 24681.895;
        let expected_diameter = 904.876;

        println!("Cost {}", tree.get_cost(&graph));
        println!("Is connected {}", tree.is_connected(&graph));

        assert!((normalize - expected_normalize).abs() < 1e-3, 
            "El normalizador no coincide. Esperado: {}, Obtenido: {}", 
            expected_normalize, tree.normalize);
            
        assert!((diameter - expected_diameter).abs() < 1e-3, 
            "El diámetro no coincide. Esperado: {}, Obtenido: {}", 
            expected_diameter, graph.get_diameter());
    }
}
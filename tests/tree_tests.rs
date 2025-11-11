#[cfg(test)]

mod test {
    // tests/tree_tests.rs

    // Importa las dependencias necesarias.
    use k_mst::entity::graph::Graph;
    use k_mst::entity::tree::Tree;

    use std::collections::{HashSet};


    // Helper para crear un grafo de prueba.
    // Grafo simple de 4 nodos: A, B, C, D, E, F
    // Diámetro: 12.0 (e.g., A->D)
    // Normalize: k = 2 -> 32.0
    fn setup_graph_for_tree() -> Graph {
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
        Graph::new(edges,2)
    }

    // Helper para crear un árbol inicial de prueba {A, B, C}.
    // MST en {A, B, C} (usando costos ajustados): A-B(3.0), B-C(5.0). Suma de pesos: 8.0. k=3.
    fn setup_initial_tree(k: usize) -> Tree {
        let nodes: HashSet<String> = vec!["A", "B", "C"].into_iter().map(String::from).collect();
        
        let edges = vec![
            ("A".to_string(), "B".to_string(), 3.0),
            ("B".to_string(), "C".to_string(), 5.0),
        ];

        Tree::new(edges, nodes, k)
    }


    #[test]
    /// Prueba el cálculo del factor de normalización (`get_normalize`) ajustado.
    /// Debe ser la suma de las k-1 (2) aristas originales con mayor costo ajustado.
    fn test_tree_get_normalize_max_k_minus_one_edges() {
        let graph = setup_graph_for_tree(); 
        let mut tree = setup_initial_tree(3); // k=3. Necesita k-1 = 2 aristas.

        // Los 2 pesos más grandes: 9.0 y 7.0.
        // Factor de normalización esperado: 9.0 + 7.0 = 16.0.

        let normalize_factor = tree.get_normalize(&graph);
        
        assert_eq!(normalize_factor, 16.0 * 2.0, "El factor de normalización debe ser la suma de los 2 costos ajustados más grandes (9.0 + 7.0) * 2 = 32.0).");
        assert_eq!(tree.normalize, 16.0 * 2.0, "El factor de normalización debe estar cacheado.");
    }

    #[test]
    /// Prueba el cálculo del costo del árbol (`get_cost`).
    fn test_tree_get_cost() {
        let graph = setup_graph_for_tree();
        let mut tree = setup_initial_tree(3); // Suma de pesos ajustados del MST: 8.0. k=3.
        
        // Normalización: 16.0
        // Costo esperado: (Suma de pesos) / Normalización = 8.0 / 16.0 = 0.2
        let expected_cost = 8.0 / 16.0;
        let cost = tree.get_cost(&graph);

        assert!((cost - expected_cost).abs() < 1e-4, "El costo total del árbol debe ser 0.25.");
        // Asegura que el normalizador se cacheó primero
        assert!((tree.normalize - 32.0).abs() < 1e-4, "El factor de normalización debe estar cacheado en 32.0.");
        assert!((tree.total_cost - expected_cost).abs() < 1e-4, "El costo total debe estar cacheado.");
    }

    #[test]
    /// Prueba la generación de un vecino (`get_neighbor`) y la recuperación (`recover_solution`).
    fn test_tree_neighbor_and_recover_solution() {
        let graph = setup_graph_for_tree(); 
        let mut tree = setup_initial_tree(3); // Árbol inicial: {A, B, C}. Costo ~0.666.
        
        let new_node = "D".to_string(); // Nodo a introducir
        let remove_node = "A".to_string(); // Nodo a remover

        // 1. Generar el vecino: {D, B, C}
        // El método get_neighbor necesita que `tree.get_normalize()` se ejecute para inicializar `tree.normalize`.
        tree.get_normalize(&graph);

        // MST en {D, B, C} tiene aristas C-D(7.0) y B-C(5.0). Suma: 12.0
        // Costo esperado: 12.0 / 16.0 ≈ 0.75
        let neighbor_result = tree.get_neighbor(&graph, &new_node, &remove_node);
        assert!(neighbor_result.is_ok());
        let (_, cost, new_n, rem_n) = neighbor_result.unwrap();

        let expected_cost = 12.0 / 32.0; 
        assert_eq!(*new_n, "D".to_string());
        assert_eq!(*rem_n, "A".to_string());
        assert!((*cost - expected_cost).abs() < 1e-4);

        // 2. Recuperar la solución (aceptar el vecino)
        let recovered = tree.recover_solution();
        
        assert!(recovered, "La recuperación debe ser exitosa.");
        
        // Verificar el nuevo estado del árbol
        assert!(!tree.nodes.contains(&"A".to_string()), "El nodo 'A' debe haber sido removido.");
        assert!(!tree.nodes.contains(&"A".to_string()), "El nodo 'D' debe haber sido añadido.");
        assert!((tree.total_cost - expected_cost).abs() < 1e-4, "El costo total debe actualizarse.");
        assert_eq!(tree.edges.len(), 2, "El nuevo árbol debe tener 2 aristas.");
    }
}
#[cfg(test)]
mod svg_tests {
    // Ajusta la ruta a tus módulos SVG, e.g., use crate::{svg_plot, svg_tree_plot};
    use k_mst::utils::{svg_plot, svg_tree_plot}; 
    use std::fs;
    use tempfile::TempDir;

    #[test]
    /// Prueba la generación del gráfico de convergencia (Costo vs. Iteración).
    /// Verifica que se cree un archivo SVG no vacío en un directorio temporal.
    fn test_plot_convergence_creates_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("convergencia.svg");
        let filename = file_path.to_str().unwrap();

        // Datos de prueba: una curva simple
        let costs = vec![100.0, 50.0, 40.0, 30.0, 30.0];

        // 1. Ejecutar la función
        let result = svg_plot::plot_convergence(&costs, filename);
        
        // 2. Verificar el resultado y el archivo
        assert!(result.is_ok(), "La función plot_convergence debería terminar OK");
        
        let metadata = fs::metadata(&file_path).unwrap();
        assert!(metadata.is_file(), "Se debe crear el archivo SVG.");
        // Un archivo SVG con ejes y etiquetas no puede ser muy pequeño
        assert!(metadata.len() > 500, "El archivo SVG no debe estar vacío.");
    }

    #[test]
    /// Prueba la generación del gráfico de árbol radial.
    /// Verifica que se cree un archivo SVG no vacío en un directorio temporal.
    fn test_plot_tree_creates_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("arbol_test.svg");
        let filename = file_path.to_str().unwrap();

        // Datos de prueba: Grafo simple para layout radial
        // Nodo central 'A' (grado 3), B, C, D en la capa 1, E en la capa 2 (via B)
        let edges = vec![
            ("A".to_string(), "B".to_string(), 1.0),
            ("A".to_string(), "C".to_string(), 1.0),
            ("A".to_string(), "D".to_string(), 1.0),
            ("B".to_string(), "E".to_string(), 2.0),
            ("C".to_string(), "D".to_string(), 3.0),
        ];

        // 1. Ejecutar la función
        let result = svg_tree_plot::plot_tree(edges, filename);
        
        // 2. Verificar el resultado y el archivo
        assert!(result.is_ok(), "La función plot_tree debería terminar OK");
        
        let metadata = fs::metadata(&file_path).unwrap();
        assert!(metadata.is_file(), "Se debe crear el archivo SVG del árbol.");
        assert!(metadata.len() > 500, "El archivo SVG no debe estar vacío.");
    }
}
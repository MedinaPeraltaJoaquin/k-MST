//! Módulo para la visualización de un árbol (o K-MST) usando un layout radial
//! centrado en el nodo de mayor grado.
use std::collections::{HashMap, HashSet, VecDeque};
use svg::node::element::{Circle, Line, Text};
use svg::Document;
use std::f64::consts::PI;

/// Estructura auxiliar para almacenar la posición y propiedades calculadas de cada nodo
struct NodePosition {
    x: f64,
    y: f64,
}

/// Genera un archivo SVG que visualiza un árbol/grafo con un layout radial.
///
/// El nodo de mayor grado se coloca en el centro, y el resto de nodos
/// se organizan en círculos concéntricos basados en su distancia (capa) al centro (BFS).
///
/// # Argumentos
/// * `edges` - Vector de tuplas (Nodo_Desde, Nodo_Hasta, Peso).
/// * `filename` - El nombre del archivo SVG de salida (ej: "arbol.svg").
///
/// # Retorno
/// Retorna `Result<(), std::io::Error>`.
pub fn plot_tree(edges: Vec<(String, String, f64)>, filename: &str) -> Result<(), std::io::Error> {

    // --- 2. Análisis del Grafo y Grados ---
    let mut adj: HashMap<String, Vec<String>> = HashMap::new(); // Lista de adyacencia
    let mut degree: HashMap<String, usize> = HashMap::new(); // Grado de cada nodo
    let mut all_nodes: HashSet<String> = HashSet::new(); // Conjunto de todos los nodos

    for (from, to, _) in &edges {
        all_nodes.insert(from.clone());
        all_nodes.insert(to.clone());

        // Construir la lista de adyacencia
        adj.entry(from.clone()).or_default().push(to.clone());
        adj.entry(to.clone()).or_default().push(from.clone());

        // Contar el grado
        *degree.entry(from.clone()).or_insert(0) += 1;
        *degree.entry(to.clone()).or_insert(0) += 1;
    }

    if all_nodes.is_empty() {
        return Ok(());
    }

    // Encontrar el nodo central (el de mayor grado)
    let center_node = all_nodes.iter()
        .max_by_key(|node| degree.get(*node).unwrap_or(&0))
        .cloned()
        .unwrap_or(all_nodes.into_iter().next().unwrap()); 


    // --- 3. Determinación de Capas (BFS - Breadth-First Search) ---
    // El BFS calcula la distancia más corta (en número de aristas) desde el centro.
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    let mut visited: HashMap<String, usize> = HashMap::new(); // nodo -> capa/profundidad
    let mut layers: HashMap<usize, Vec<String>> = HashMap::new(); // capa -> lista de nodos

    queue.push_back((center_node.clone(), 0));
    visited.insert(center_node.clone(), 0);
    layers.entry(0).or_default().push(center_node.clone());

    let mut max_layer = 0;

    while let Some((node, depth)) = queue.pop_front() {
        if depth > max_layer {
            max_layer = depth;
        }
        
        if let Some(neighbors) = adj.get(&node) {
            for neighbor in neighbors {
                if !visited.contains_key(neighbor) {
                    let next_depth = depth + 1;
                    visited.insert(neighbor.clone(), next_depth);
                    layers.entry(next_depth).or_default().push(neighbor.clone());
                    queue.push_back((neighbor.clone(), next_depth));
                }
            }
        }
    }

    // --- 1. Definición de Parámetros SVG y Layout ---
    let radius_step: f64 = 150.0;  // Separación radial entre capas de nodos
    let width: f64 = max_layer as f64 * radius_step * 2.0 + 400.0;
    let height: f64 = 2000.0;
    let center_x: f64 = width / 2.0;
    let center_y: f64 = height / 2.0;
    let node_r: f64 = 8.0;         // Radio del nodo para el dibujo

    // --- 4. Cálculo de Posiciones SVG ---
    let mut node_positions: HashMap<String, NodePosition> = HashMap::new();

    // 4.1. Posición del nodo central (Capa 0)
    node_positions.insert(center_node.clone(), NodePosition {
        x: center_x,
        y: center_y,
    });

    // 4.2. Posiciones de los nodos en capas (Layout Radial)
    for layer in 1..=max_layer {
        if let Some(nodes_in_layer) = layers.get(&layer) {
            let count = nodes_in_layer.len();
            let radius = layer as f64 * radius_step;

            for (i, node) in nodes_in_layer.iter().enumerate() {
                // Calcular ángulo: Distribuir uniformemente los nodos en el perímetro del círculo
                let angle = (i as f64 / count as f64) * 2.0 * PI;

                // Calcular coordenadas cartesianas (x, y)
                let x = center_x + radius * angle.cos();
                let y = center_y + radius * angle.sin();

                node_positions.insert(node.clone(), NodePosition {
                    x,
                    y,
                });
            }
        }
    }


    // --- 5. Creación del Documento SVG ---
    let mut document = Document::new()
        .set("viewBox", (0, 0, width, height))
        .set("width", width)
        .set("height", height);
        
    // 5.1. Dibujar Aristas (Líneas)
    let mut drawn_edges: HashSet<(String, String)> = HashSet::new(); // Para evitar duplicados

    for (from_name, to_name, _) in edges {
        // Normalizar la tupla para evitar dibujar la misma arista dos veces (ej: A->B y B->A)
        let normalized_edge = if from_name < to_name {
            (from_name.clone(), to_name.clone())
        } else {
            (to_name.clone(), from_name.clone())
        };

        if drawn_edges.contains(&normalized_edge) {
            continue;
        }
        drawn_edges.insert(normalized_edge);

        if let (Some(pos1), Some(pos2)) = (node_positions.get(&from_name), node_positions.get(&to_name)) {
            let line = Line::new()
                .set("x1", pos1.x)
                .set("y1", pos1.y)
                .set("x2", pos2.x)
                .set("y2", pos2.y)
                .set("stroke", "gray")
                .set("stroke-width", 1.0);

            document = document.add(line);
        }
    }

    // 5.2. Dibujar Nodos (Círculos y Etiquetas)
    for (node_name, pos) in &node_positions {
        // Nodo (Círculo)
        let color = if *node_name == center_node { "red" } else { "black" }; // Nodo central en rojo
        let circle = Circle::new()
            .set("cx", pos.x)
            .set("cy", pos.y)
            .set("r", node_r)
            .set("fill", color)
            .set("stroke", "white")
            .set("stroke-width", 1.5);
        
        // Etiqueta (Nombre del nodo)
        let label = Text::new(node_name)
            .set("x", pos.x)
            .set("y", pos.y + node_r * 2.5) // Posicionar debajo del círculo
            .set("font-size", 12)
            .set("fill", "black")
            .set("text-anchor", "middle");

        document = document.add(circle).add(label);
    }

    // --- 6. Guardar SVG ---
    svg::save(filename, &document)?;

    Ok(())
}
//! Módulo para la visualización del proceso de convergencia del algoritmo WOA.
//! Genera un archivo SVG mostrando el Costo (Eje Y) vs. Iteración (Eje X).
use svg::node::element::path::{Data};
use svg::node::element::{Path, Rectangle, Text};
use svg::Document;
use std::{fs::create_dir_all};

/// Genera un archivo SVG que visualiza la convergencia del algoritmo (Costo vs. Iteración).
///
/// La línea de convergencia se dibuja en azul.
///
/// # Argumentos
/// * `costs` - Un slice de f64 representando el mejor costo encontrado en cada iteración.
/// * `filename` - El nombre del archivo SVG de salida (ej: "convergencia.svg").
///
/// # Retorno
/// Retorna `Result<(), std::io::Error>`.
pub fn plot_convergence(costs: &[f64], filename: &str) -> Result<String, std::io::Error> {
    if costs.is_empty() {
        return Ok(filename.to_string());
    }

    // --- 1. Definición de Parámetros de Dibujo ---
    const WIDTH: i64 = 800;
    const HEIGHT: i64 = 500;
    const PADDING: i64 = 50; // Margen para los ejes y etiquetas

    let x_max = costs.len() as f64 - 1.0;
    let y_max = costs[0]; // Costo inicial (típicamente el máximo)
    let y_min = costs.iter().cloned().fold(f64::INFINITY, f64::min); // Costo mínimo

    // Cálculo del rango para escalado.
    let y_range = y_max - y_min;
    
    // Si la convergencia es plana, ajustamos el rango (ej: 1.0) para que el gráfico no colapse.
    let effective_y_range = if y_range < 1e-6 { 1.0 } else { y_range };

    // Factor de escalado Y: Mapea el rango de costo al área de dibujo (alto)
    let y_scale_factor = (HEIGHT - 2 * PADDING) as f64 / effective_y_range; 
    // Factor de escalado X: Mapea el número de iteraciones al área de dibujo (ancho)
    let x_scale_factor = (WIDTH - 2 * PADDING) as f64 / x_max;

    // --- 2. Preparación de la Ruta (Línea Azul) ---
    let mut data = Data::new();
    
    // Función de mapeo (Iteración, Costo) -> (x_pixel, y_pixel)
    let map_to_svg = |i: usize, cost: f64| -> (f64, f64) {
        let x = PADDING as f64 + (i as f64) * x_scale_factor;
        // La coordenada Y del SVG es invertida (0 arriba, MAX abajo).
        let y = HEIGHT as f64 - PADDING as f64 - (cost - y_min) * y_scale_factor;
        (x, y)
    };

    // Mover al primer punto (Iteración 0)
    let (x0, y0) = map_to_svg(0, costs[0]);
    data = data.move_to((x0, y0));

    // Dibujar la línea conectando los puntos restantes
    for (i, &cost) in costs.iter().enumerate().skip(1) {
        let (xi, yi) = map_to_svg(i, cost);
        data = data.line_to((xi, yi));
    }
    
    // Crear el elemento Path (la línea de convergencia)
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "blue")
        .set("stroke-width", 2)
        .set("d", data);


    // --- 3. Creación del Documento SVG y Ejes ---
    let mut document = Document::new()
        .set("viewBox", (0, 0, WIDTH, HEIGHT))
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .add(Rectangle::new() // Fondo blanco
            .set("x", 0).set("y", 0).set("width", WIDTH).set("height", HEIGHT).set("fill", "white")
        )
        .add(path); // Línea de convergencia
        
    // Ejes (Líneas simples)
    let axis_x = Path::new()
        .set("fill", "none").set("stroke", "black")
        .set("d", Data::new().move_to((PADDING, HEIGHT - PADDING)).line_to((WIDTH - PADDING, HEIGHT - PADDING)));
    
    let axis_y = Path::new()
        .set("fill", "none").set("stroke", "black")
        .set("d", Data::new().move_to((PADDING, PADDING)).line_to((PADDING, HEIGHT - PADDING)));
    
    document = document.add(axis_x).add(axis_y);

    
    // --- 4. Etiquetas de los Ejes ---

    // Etiqueta del Eje X: Título
    document = document.add(
        Text::new("Iteración / Generación")
            .set("x", WIDTH / 2)
            .set("y", (HEIGHT - PADDING) as f64 / 4.0)
            .set("font-size", 20)
            .set("text-anchor", "middle")
    );
    
    // Etiqueta del Eje Y: Título (Rotada)
    document = document.add(
        Text::new("Costo / Fitness")
            .set("transform", format!("translate(20, {}) rotate(270)", HEIGHT / 2))
            .set("font-size", 20)
            .set("text-anchor", "middle")
    );
    
    // Etiquetas de valores en el Eje X (Inicio y Fin)
    document = document
        // Inicio (Iteración 0)
        .add(Text::new("0")
            .set("x", PADDING)
            .set("y", HEIGHT - PADDING + 20)
            .set("font-size", 14)
            .set("text-anchor", "middle")
        )
        // Fin (Iteración Máxima)
        .add(Text::new(format!("{}", x_max as usize))
            .set("x", WIDTH - PADDING)
            .set("y", HEIGHT - PADDING + 20)
            .set("font-size", 14)
            .set("text-anchor", "middle")
        );

    // Etiquetas de valores en el Eje Y (Costo Máximo y Mínimo)
    document = document
        // Costo Máximo (Arriba)
        .add(Text::new(format!("{:.3}", y_max))
            .set("x", PADDING - 10)
            .set("y", PADDING + 5)
            .set("font-size", 14)
            .set("text-anchor", "end") 
        )
        // Costo Mínimo (Abajo)
        .add(Text::new(format!("{:.3}", y_min))
            .set("x", PADDING - 10)
            .set("y", HEIGHT - PADDING + 5)
            .set("font-size", 14)
            .set("text-anchor", "end")
        );


    // --- 5. Escritura en el Archivo ---
    create_dir_all("./svg_reports")?;
    let filepath = format!("./svg_reports/{}", filename);
    svg::save(filepath, &document)?;

    Ok(filename.to_string())
}
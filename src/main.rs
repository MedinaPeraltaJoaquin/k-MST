mod utils;
mod entity;
mod woa;

use std::{env, process::exit};

use utils::read_input::ReadInput;
use utils::config::Config;
use utils::svg_plot::plot_convergence;
use utils::svg_tree_plot::plot_tree;
use entity::graph::Graph;
use woa::woa::WOA;


pub fn main(){
    let args : Vec<String> = env::args().collect();
    let mut read_input = match ReadInput::new(args) {
        Ok(read) => read,
        Err(e) => {
            panic!("Error al leer argumentos: {:?}\nUtilice --help o -h", e);
        }
    };   

    if read_input.get_help() {
        read_input.print_help();
        exit(0);
    }

    println!("Iniciando k-MST con Whale Optimization Algorithm (WOA)");
    let verbose_mode = read_input.get_verbose();
    let svg_mode = read_input.get_svg();

    let seeds = match read_input.get_seed() {
        Ok(seeds) => seeds,
        Err(e) => {
            panic!("Error al leer la semilla: {:?}",e);
        }
    };

    let k_nodes = match read_input.get_k_nodes() {
        Ok(k) => k,
        Err(e) => {
            panic!("Error al leer el valor k: {:?}",e);
        }
    };

    println!("Cargando el grafo desde el archivo...");
    let graph_vec = match read_input.read_file() {
        Ok(result) => result,
        Err(e) => {
            panic!("{:?}",e);
        }
    };

    let graph = Graph::new(graph_vec,k_nodes);
    
    let config = Config::from_env();

    let mut best_solution = std::f64::INFINITY;
    let mut best_seed = seeds[0];
    for seed in &seeds {
        println!("Running WOA with seed: {}", seed);

        let mut woa = WOA::new(
            config.size_population,
            config.max_iteration,
            config.lb,
            config.ub,
            *seed as u64,
            k_nodes,
            &graph
        );

        woa.woa(&graph);
        let mut best_whale = woa.get_best_whale();
        let convergence = woa.get_convergence();
        let idx_best_whale = woa.get_idx_best_whale();
        println!("Seed: {}: Best Cost: {}",seed,best_whale.get_cost(&graph));
        if !best_whale.get_tree_struct().is_connected(&graph) {
            eprintln!("Warning: The best whale's tree is not connected or inconsistent!");
        }
        if verbose_mode {
            println!("Best Whale Index in Population: {}", idx_best_whale);
            println!("Best Whale Cost: {}", best_whale.get_cost(&graph));
        }
        if svg_mode {
            let filename_plot = format!("./svg/convergence_seed_{}.svg",seed);
            match plot_convergence(&convergence, &filename_plot) {
                Ok(_) => println!("Gráfica de convergencia guardada en: {}", filename_plot),
                Err(e) => eprintln!("Error al guardar la gráfica de convergencia: {}", e),
            };

            let filename_tree = format!("./svg/tree_seed_{}.svg",seed);
            match plot_tree(best_whale.get_tree(), &filename_tree) {
                Ok(_) => println!("Árbol guardado en: {}", filename_tree),
                Err(e) => eprintln!("Error al guardar el árbol: {}", e),
            };
        }

        if best_whale.get_cost(&graph) < best_solution {
            best_solution = best_whale.get_cost(&graph);
            best_seed = *seed;
        }
    }

    println!("Mejor solución encontrada con semilla {}: Costo = {}", best_seed, best_solution);
    exit(0);
}
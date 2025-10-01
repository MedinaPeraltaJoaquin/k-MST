mod utils;

use std::{env, process::exit};

use utils::read_input::ReadInput;

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

    let mut graph_vec = match read_input.read_file() {
        Ok(result) => result,
        Err(e) => {
            panic!("{:?}",e);
        }
    };

    println!("k: {}",k_nodes);
    println!("graph: {:?}",graph_vec);
    println!("seeds: {:?}",seeds);

    exit(0);
}
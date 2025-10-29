use std::fmt;
use std::fs;

#[derive(Debug)]
pub enum InputError {
    FileNotFound(String),
    InvalidFormat(String),
    InvalidPath(String),
    NoArgs,
    InvalidArgumentSeed,
    InvalidSeed,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            InputError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            InputError::InvalidPath(msg) => write!(f,"Invalid path: {}",msg),
            InputError::NoArgs => write!(f,"Faltan argumentos"),
            InputError::InvalidArgumentSeed => write!(f,"No se pueden usar ambos argumentos"),
            InputError::InvalidSeed => write!(f,"Seed inválida")
        }
    }
}

impl std::error::Error for InputError {}

pub struct ReadInput {
    pub args : Vec<String>,
    pub graph : Vec<(String,String,f64)>,
    pub k_nodes : usize,
    pub seeds : Vec<i32>,
}

impl ReadInput {
    pub fn new(args: Vec<String>) -> Result<Self, InputError> {
        if args.len() == 1 {
            return Err(InputError::NoArgs);
        }
        Ok(ReadInput { args, graph: vec![], k_nodes: 0, seeds: vec![] })
    }

    pub fn read_file(&mut self) -> Result<Vec<(String,String,f64)>, InputError> {
        if !self.graph.is_empty() {
            return Ok(self.graph.clone());
        }

        let position = if let Some(pos) = self.get_position_flag("-p") {
            if pos + 1 >= self.args.len() {
                return Err(InputError::FileNotFound("No se encontro el valor de path".to_string()));
            }
            pos
        } else {
            return Err(InputError::InvalidPath("No se encontro la bandera".to_string()));
        };

        let next_arg = &self.args[position + 1];
        if next_arg.ends_with(".txt") {
            let content = fs::read_to_string(next_arg)
                        .map_err(|_| InputError::InvalidPath("Error al leer el archivo".to_string()))?;

            let lines: Vec<&str> = content.lines().collect();
            for line in lines {
                let parts = line.split(",").collect::<Vec<&str>>();
                if parts.len() != 3 {
                    return Err(InputError::InvalidFormat("Cada línea debe tener 3 partes separadas por comas".to_string()));
                }

                let node1: String = match self.get_node(parts[0]) {
                    Ok(n) => n,
                    Err(e) => return Err(e),
                };
                let node2: String = match self.get_node(parts[1]) {
                    Ok(n) => n,
                    Err(e) => return Err(e),
                };

                let weight: f64 = parts[2].trim().parse()
                            .map_err(|_| InputError::InvalidFormat("Peso inválido".to_string()))?;

                self.graph.push((node1, node2, weight));
            }

            
        } else {
            return Err(InputError::InvalidFormat("Debe de ser un archivo .txt".to_string()));
        };

        return Ok(self.graph.clone());
    }

    pub fn get_k_nodes(&mut self) -> Result<usize, InputError> {
        if self.k_nodes != 0 {
            return Ok(self.k_nodes);
        }

        let position = if let Some(pos) = self.get_position_flag("-k") {
            if pos + 1 > self.args.len() {
                return Err(InputError::InvalidFormat("No se encontro el valor de k".to_string()));
            }
            pos
        } else {
            return Err(InputError::InvalidPath("No se encontro la bandera".to_string()));
        };

        let next_arg = &self.args[position + 1];
        let k: usize = next_arg.parse().map_err(|_| InputError::InvalidFormat("K debe ser un número entero positivo".to_string()))?;
        self.k_nodes = k;
        Ok(self.k_nodes)
    }

    pub fn get_seed(&mut self) -> Result<Vec<i32>, InputError> {
        if !self.seeds.is_empty() {
            return Ok(self.seeds.clone());
        }

        let pos_s = self.get_position_flag("-s");
        let pos_rs = self.get_position_flag("-rs");

        match (pos_s, pos_rs) {
            (Some(_), Some(_)) => {
                return Err(InputError::InvalidArgumentSeed);
            }
            (Some(pos), None) => {
                if self.args.len() < pos + 1 {
                    return Err(InputError::InvalidFormat("No se encontro el valor de seed".to_string()));
                }

                let start: i32 = self.args[pos + 1].parse()
                    .map_err(|_| InputError::InvalidSeed)?;

                if self.args.len() <= pos + 2 || self.args[pos + 2].starts_with('-') {
                    let seeds = vec![start];
                    self.seeds = seeds.clone();
                    return Ok(seeds);
                }

                let end: i32 = self.args[pos + 2].parse()
                    .map_err(|_| InputError::InvalidSeed)?;

                if start > end {
                    return Err(InputError::InvalidSeed);
                }

                let seeds: Vec<i32> = (start..=end).collect();
                self.seeds = seeds.clone();
                return Ok(seeds);
            }
            (None, Some(pos)) => {
                if self.args.len() <= pos + 1 {
                    return Err(InputError::InvalidFormat("No se encontro el valor de seed".to_string()));
                }

                let n: usize = self.args[pos + 1].parse()
                    .map_err(|_| InputError::InvalidSeed)?;

                if n == 0 {
                    return Err(InputError::InvalidSeed);
                }

                use rand::Rng;
                let mut rng = rand::thread_rng();
                let seeds: Vec<i32> = (0..n).map(|_| rng.r#gen()).collect();
                self.seeds = seeds.clone();
                return Ok(seeds);
            }
            (None, None) => {
                Err(InputError::InvalidSeed)
            }
        }
    }

    pub fn get_verbose(&self) -> bool {
        self.get_flag("-v")
    }

    pub fn get_svg(&self) -> bool {
        self.get_flag("-svg")
    }
    
    pub fn get_help(&self) -> bool {
        self.get_flag("-h") || self.get_flag("--help")
    }

    pub fn print_help(&self) {
        println!("Uso: programa [opciones]");
        println!();
        println!("Opciones:");
        println!("  -h, --help         Muestra esta ayuda y termina");
        println!("  -v                 Activa el modo verbose");
        println!("  -p <path>          Ruta explícita del archivo .txt que representa una gráfica");
        println!("  -svg               Activa el modo de salida SVG");
        println!("  -s <I> <F>         Genera semillas en el rango [I, F]");
        println!("  -s <n>             Inicializa con la semilla n");
        println!("  -rs <n>            Genera n semillas aleatorias");
        println!("  -k <n>             Valor para encontrar la k-MST");
    }

    fn get_flag(&self, flag : &'static str) -> bool {
        self.args.iter().any(|arg| arg == flag)
    }

    fn get_position_flag(&self, flag: &str) -> Option<usize> {
        self.args.iter().position(|arg| arg == flag)
    }

    fn get_node(&self, s: &str) -> Result<String, InputError> {
        let trimmed = s.trim();
        Ok(trimmed.to_string())
    }
}
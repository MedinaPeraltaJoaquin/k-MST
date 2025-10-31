//! Módulo para manejar la configuración de la aplicación,
//! cargando los parámetros desde variables de entorno.
use dotenvy::dotenv;
use std::env;

/// Estructura que almacena los parámetros de configuración del algoritmo WOA.
#[derive(Debug)]
pub struct Config {
    /// Tamaño de la población de ballenas.
    pub size_population: usize,
    /// Número máximo de iteraciones.
    pub max_iteration: usize,   
    /// Límite inferior (Lower Bound) para las posiciones de las ballenas.
    pub lb : f64,
    /// Límite superior (Upper Bound) para las posiciones de las ballenas.
    pub ub : f64,
}

impl Config {
    /// Inicializa la configuración leyendo las variables de entorno.
    ///
    /// Busca y parsea las siguientes variables: SIZE_POPULATION, MAX_ITERATION, LB, UB.
    /// Si alguna variable falta o no tiene el formato correcto, el programa entrará en pánico (`panic!`).
    ///
    /// # Retorno
    /// Una nueva instancia de `Config` con los valores leídos.
    pub fn from_env() -> Self {
        // Carga las variables de entorno desde el archivo .env (si existe)
        dotenv().ok();

        let size_population = env::var("SIZE_POPULATION")
            .expect("Falta SIZE_POPULATION en .env")
            .parse::<usize>()
            .expect("SIZE_POPULATION debe ser un número entero");

        let max_iteration = env::var("MAX_ITERATION")
            .expect("Falta MAX_ITERATION en .env")
            .parse::<usize>()
            .expect("MAX_ITERATION debe ser un número entero");

        let lb = env::var("LB")
            .expect("Falta LB en .env")
            .parse::<f64>()
            .expect("LB debe ser un número");

        let ub = env::var("UB")
            .expect("Falta UB en .env")
            .parse::<f64>()
            .expect("UB debe ser un número");

        Config {
            size_population,
            max_iteration,
            lb,
            ub
        }
    }
}
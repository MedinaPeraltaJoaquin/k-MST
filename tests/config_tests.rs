#[cfg(test)]
mod config_tests {
    use k_mst::utils::config::Config; 
    use std::env;
    use serial_test::serial; 

    // --- Helper para limpiar las variables ---
    fn cleanup_env() {
        unsafe {
            env::remove_var("SIZE_POPULATION");
            env::remove_var("MAX_ITERATION");
            env::remove_var("LB");
            env::remove_var("UB");
        }
    }

    #[test]
    #[serial] 
    /// Prueba que la configuración se cargue correctamente de variables de entorno simuladas.
    fn test_config_from_env_success() {
        unsafe {
            env::set_var("SIZE_POPULATION", "10");
            env::set_var("MAX_ITERATION", "100");
            env::set_var("LB", "-5.1");
            env::set_var("UB", "5.1");
        };

        // 2. Ejecutar
        let config = Config::from_env();

        // 3. Assertions
        assert_eq!(config.size_population, 10);
        assert_eq!(config.max_iteration, 100);
        assert_eq!(config.lb, -5.1);
        assert_eq!(config.ub, 5.1);

        // 4. Cleanup: Limpiar variables de entorno
        cleanup_env();
    }

/*     #[test]
    #[serial]
    #[should_panic(expected = "Falta MAX_ITERATION en .env")]
    /// Prueba que `Config::from_env` entra en pánico si falta una variable crucial.
    fn test_config_from_env_missing_var_panic() {
        // 1. Setup: Limpiar primero (por si otro test falló) y establecer las necesarias
        cleanup_env(); 
        
        unsafe {
            env::set_var("SIZE_POPULATION", "10");
            env::set_var("LB", "0.0");
            env::set_var("UB", "1.0");
        }

        // 2. Ejecutar (Debe entrar en pánico)
        let _ = Config::from_env(); 
        
        cleanup_env();
    } */
}
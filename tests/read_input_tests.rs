#[cfg(test)]
mod tests {
    use k_MST::utils::read_input::{InputError,ReadInput};
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_new_no_args() {
        let args = vec!["program".to_string()];
        let result = ReadInput::new(args);
        assert!(matches!(result, Err(InputError::NoArgs)));
    }

    #[test]
    fn test_new_with_args() {
        let args = vec!["program".to_string(), "-p".to_string(), "1".to_string()];
        let result = ReadInput::new(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_path_from_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "1,  2,3.2").unwrap();
        writeln!(file, "2,3 ,6.3").unwrap();
        writeln!(file, "1,  3,0.23").unwrap();

        let args = vec![
            "program".to_string(),
            "-p".to_string(),
            file_path.to_str().unwrap().to_string(),
        ];
        let mut ri = ReadInput::new(args).unwrap();
        let graph = ri.read_file().unwrap();
        assert_eq!(graph, vec![('1','2',3.2),('2','3',6.3),('1','3',0.23)]);
    }

    #[test]
    fn test_get_seed_single_number() {
        let args = vec!["program".to_string(), "-s".to_string(), "5".to_string()];
        let mut ri = ReadInput::new(args).unwrap();
        let seeds = ri.get_seed().unwrap();
        assert_eq!(seeds, vec![5]);
    }

    #[test]
    fn test_get_seed_range() {
        let args = vec![
            "program".to_string(),
            "-s".to_string(),
            "3".to_string(),
            "5".to_string(),
        ];
        let mut ri = ReadInput::new(args).unwrap();
        let seeds = ri.get_seed().unwrap();
        assert_eq!(seeds, vec![3, 4, 5]);
    }

    #[test]
    fn test_get_seed_rs() {
        let args = vec!["program".to_string(), "-rs".to_string(), "10".to_string()];
        let mut ri = ReadInput::new(args).unwrap();
        let seeds = ri.get_seed().unwrap();
        assert_eq!(seeds.len(), 10);
    }

    #[test]
    fn test_get_help() {
        let args = vec!["program".to_string(), "-h".to_string()];
        let ri = ReadInput::new(args).unwrap();
        assert!(ri.get_help());

        let args2 = vec!["program".to_string(), "--help".to_string()];
        let ri2 = ReadInput::new(args2).unwrap();
        assert!(ri2.get_help());
    }

    #[test]
    fn test_invalid_seed() {
        let args = vec!["program".to_string(), "-s".to_string(), "10".to_string(), "5".to_string()];
        let mut ri = ReadInput::new(args).unwrap();
        let res = ri.get_seed();
        assert!(matches!(res, Err(InputError::InvalidSeed)));
    }

    #[test]
    fn test_invalid_path_file() {
        let mut args = vec!["program".to_string(), "-p".to_string(), "nonexistent.txt".to_string()];
        let mut ri = ReadInput::new(args.clone()).unwrap();
        let res = ri.read_file();
        assert!(matches!(res, Err(InputError::InvalidPath(_))));

        args.pop();
        ri = ReadInput::new(args.clone()).unwrap();
        let res = ri.read_file();
        assert!(matches!(res,Err(InputError::FileNotFound(_))));


        args.push("fileNotType.s".to_string());
        ri = ReadInput::new(args.clone()).unwrap();
        let res = ri.read_file();
        assert!(matches!(res, Err(InputError::InvalidFormat(_))));


        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "1,2").unwrap();

        args.pop();
        args.push(file_path.to_str().unwrap().to_string());
        ri = ReadInput::new(args).unwrap();
        let res = ri.read_file();
        assert!(matches!(res, Err(InputError::InvalidFormat(_))));
    }

    #[test]
    fn test_get_verbose() {
        let args = vec!["program".to_string(), "-v".to_string()];
        let ri = ReadInput::new(args).unwrap();
        assert!(ri.get_verbose());
    }

    #[test]
    fn test_get_svg() {
        let args = vec!["program".to_string(), "-svg".to_string()];
        let ri = ReadInput::new(args).unwrap();
        assert!(ri.get_svg());
    }
}
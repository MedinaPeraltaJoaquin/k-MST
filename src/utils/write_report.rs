use std::{fs::{File,create_dir_all}, io::Write};

pub fn save_report(edges : Vec<(String, String, f64)>, seed : i32, timestamp : String) -> Result<String, std::io::Error> {
    create_dir_all("./edges_reports")?;
    let filename = format!("./edges_reports/report_seed_{}_{}.txt", seed, timestamp);

    let edges_report: String = edges
        .iter()
        .map(|(src, dest, weight)| format!("{},{},{}\n", src, dest, weight))
        .collect();

    let mut file = File::create(&filename)?;
    file.write_all(edges_report.as_bytes())?;

    println!("Report saved to {}", filename);
    Ok(filename)
}
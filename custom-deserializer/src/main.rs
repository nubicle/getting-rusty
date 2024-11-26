use std::{fs, io::Read};

mod swagger20;

fn main() {
    let mut file = fs::File::open("swagger.json").unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    let mut data = String::new();
    let _ = file.read_to_string(&mut data);

    let spec: swagger20::Spec = serde_json::from_str(&data).unwrap();
    // println!("{:?}", spec);

    for (path, value) in &spec.paths {
        println!("{:?}", path);
        println!("{:?}", value);
    }
}

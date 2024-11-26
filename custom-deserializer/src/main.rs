use std::fs;

mod swagger20;

fn main() {
    let file = fs::File::open("swagger.json").unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    let spec: swagger20::Spec = serde_json::from_reader(file).unwrap();
    for op in &spec.operations {
        println!("{:?}: \t{}", op.id, op.path);
    }
}

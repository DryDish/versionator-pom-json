use std::env::current_dir;

fn main() {
    let source_name = "package.json";
    let current_dir = current_dir().unwrap();
    println!("Searching {:?} for '{}'",current_dir, source_name);

    


}

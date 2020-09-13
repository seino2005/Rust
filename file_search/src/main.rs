use glob::glob;
use std::env;

fn main() {

    let pwd = env::current_dir().unwrap();

    for entry in glob("**\\*").expect("Failed to read") {
        match entry {
            Ok(path) => println!("{:?}", pwd.join(path).display()),
            Err(e) => println!("{:?}", e)
        }
    }
    
    let files : Vec<_> = glob("*").expect("Failed to read").map(|e| e.unwrap()).collect();
    for file in files {
        println!("File {:?}", file);
    }


}

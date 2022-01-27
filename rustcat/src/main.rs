fn main() {

   // println!("Rust Version of cat command.");                        
    if let Err(e) = rustcat::get_arguments_from_user().and_then(rustcat::run){
    eprintln!("{}", e);
    std::process::exit(1);
    
}

}

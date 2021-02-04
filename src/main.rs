use std::env;
use std::process;

mod lib;
use lib::Params;

fn main() {
    //collect args, remove program name from vec
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    println!("{:?}", args);

    let params = Params::new(args).unwrap_or_else(|err| {
        eprintln!("Error occured: {}", err);
        process::exit(1);
    });
    println!("{:?}", params);

    if let Err(e) = lib::run(params) {
        eprintln!("{}", e);
        process::exit(1);
    }
    //also check if the path is a dir or a file using is_dir, or is_file on the metadata
}

//! # Maxgrep
//! Maxgrep is an extension on the CLI File I/O project from Rust Book's Chapter 12.
//! Taking inspiration from Windows' Findstr, Maxgrep can, in addition to setting search case-sensitivity
//! via command line arguments, print line numbers or even print lines that don't match.

use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Params {
    query: String,
    filename: String,
    case_insensitive: bool, // /c
    print_line_no: bool,    // /n
    print_nonmatch: bool,   // /v
}

impl Params {
    /// Sets the parameters of the program from a given vector of arguments.
    /// Iterates over the vector, finding and removing the first occurence of valid switches from the vector.
    /// If any of these switches are found, the corresponding bool field is set in the struct.
    ///
    /// # Errors
    /// An error is returned if:
    /// 1. The vector supplied has fewer than two elements.
    /// 2. The vector has invalid switches.
    ///
    /// # Examples
    /// ```
    ///     use std::process;
    ///     let mut args: Vec<String> = env::args().collect();
    ///
    ///     println!("{:?}", args);
    ///
    ///     let params = Params::new(args).unwrap_or_else(|err| {
    ///        eprintln!("Error occured: {}", err);
    ///        process::exit(1);
    ///     });
    /// ```

    pub fn new(mut args: Vec<String>) -> Result<Params, &'static str> {
        // closure to find and remove valid switches from vector, so query and filename
        //remain in the same position.
        let mut args_parse = |arg: &str| -> bool {
            match args.iter().position(|x| *x == arg) {
                Some(i) => Some(args.remove(i)).is_some(), //returns true
                None => false,
            }
        };

        let print_line_no = args_parse("/n");
        let print_nonmatch = args_parse("/v");
        let case_insensitive = args_parse("/c");

        //check arg length after popping switches
        if args.len() != 2 {
            return Err("Invalid number of arguments.");
        }

        let query = args[0].clone();
        let filename = args[1].clone();

        Ok(Params {
            query,
            filename,
            case_insensitive,
            print_line_no,
            print_nonmatch,
        })
    }
}

//returning a result to take advantage of ? operator
pub fn run(params: Params) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(&params.filename)?;
    let results = search(&params, &file);

    //sort using vector to get hmap contents in order
    let mut sort_results: Vec<&usize> = results.keys().collect();
    sort_results.sort();

    //print results
    for line_ in sort_results {
        if params.print_line_no {
            print!("{}: ", line_);
        }
        println!("{}", results.get(line_).unwrap());
    }

    Ok(())
}

pub fn search<'a>(params: &Params, file: &'a str) -> HashMap<usize, &'a str> {
    let mut query = params.query.clone();
    let mut results_hmap: HashMap<usize, &str> = HashMap::new();

    //provide for case_insensitive flag
    if params.case_insensitive {
        query = query.to_lowercase();
    }

    //search
    for (i, line) in file.lines().enumerate() {
        let mut line_ = line.to_string();

        //provide for case_insensitive flag
        if params.case_insensitive {
            line_ = line.to_lowercase();
        }

        //check for matches
        match line_.contains(&query) {
            true if !params.print_nonmatch => results_hmap.insert(i + 1, line),
            false if params.print_nonmatch => results_hmap.insert(i + 1, line),
            _ => None,
        };
    }

    results_hmap
}

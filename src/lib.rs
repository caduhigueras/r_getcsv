#![cfg_attr(windows, feature(abi_vectorcall))]

use csv::ReaderBuilder;
use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

fn get_csv_standard(path: &str, delimiter: &str) -> Result<ZBox<ZendHashTable>, Box<dyn Error>> {
    let mut final_vec = ZendHashTable::new();
    let delim = delimiter.as_bytes()[0];

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(delim)
        .flexible(true)
        .from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        final_vec.push(
            record
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        )?;
    }

    Ok(final_vec)
}

fn get_from_file_read(path: &str, delimiter: &str) -> Result<ZBox<ZendHashTable>, Box<dyn Error>> {
    let mut final_vec = ZendHashTable::new();

    if let Ok(lines) = read_lines(path) {
        let mut first_line = true;
        let mut line_count = 0;

        for line in lines.map_while(Result::ok) {
            //---------- Convert string into vector
            let mut record: Vec<String> = line
                .split(delimiter)
                .map(|s| s.trim_matches('"').to_string())
                .collect();

            //---------- Infer number of columns from header
            if first_line {
                line_count = record.len();
                first_line = false;
            }

            //---------- If columns are missing, fill with empty strings
            if record.len() < line_count {
                let missing = line_count - record.len();
                for _ in 0..missing {
                    record.push(String::new());
                }
            } else if record.len() > line_count {
                record.truncate(line_count);
            }

            final_vec.push(record)?;
        }
    }

    Ok(final_vec)
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[php_function]
fn r_getcsv(path: String, delimiter: Option<String>) -> ZBox<ZendHashTable> {
    let delim = delimiter.unwrap_or(String::from(","));

    let result = if delim.len() == 1 {
        get_csv_standard(&path, &delim)
    } else {
        get_from_file_read(&path, &delim)
    };

    match result {
        Ok(data) => data,
        Err(err) => {
            println!("error running example: {}", err);
            process::exit(1);
        }
    }
}

// Required to register the extension with PHP.
#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

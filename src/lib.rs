#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
use std::error::Error;
use std::process;
use csv::Reader;

fn example(path: &str) -> Result<ZBox<ZendHashTable>, Box<dyn Error>> {
    let mut final_vec = ZendHashTable::new();
    // let mut final_vec: Vec<Vec<String>> = Vec::new();

    // let path = "/home/ubuntu/app/tools/testing_csv/src/.data/test.csv";
    let mut rdr = Reader::from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        final_vec.push(record.iter().map(|s| s.to_string()).collect::<Vec<String>>()).expect("Failed to parse vec");
    }
    // println!("{:?}", final_vec);
    Ok(final_vec)
}

#[php_function]
fn r_getcsv(path: String) -> ZBox<ZendHashTable> {
    let result = example(&path);
    
    match result {
        Ok(data) => {
            data
        } Err(err) => {
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

//add on readme
//sudo apt-get install -y libc6-dev gcc build-essential
//sudo apt install llvm-dev libclang-dev clang
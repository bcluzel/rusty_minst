use std::{error::Error, process};

const IMAGE_SIZE: usize = 784;
extern crate nalgebra as na;

#[derive(Debug, serde::Deserialize)]
struct MinstRecord {
    label: i32,
    values: Vec<i32>,
}

fn parse_mnist_record(path: &str) -> Vec<MinstRecord> {
    let mut rdr = csv::ReaderBuilder::new()
    .has_headers(false)
    .from_path(path).unwrap();

    let iter = rdr.deserialize().skip(1);
    iter.map(|x| x.unwrap()).collect()
}

fn main_training() -> Result<(), Box<dyn Error>> {
    let training_data = parse_mnist_record("data/mnist_train.csv");
    println!("Training data len {}", training_data.len());
    Ok(())
}

fn main() {
    if let Err(err) = main_training() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

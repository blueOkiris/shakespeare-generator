/*
 * Author: Dylan Turner
 * Description: Interface to Shakespeare Generating tool
 */

mod data;

use std::{
    str::from_utf8,
    time::Instant
};
use clap::{
    Arg, Command, crate_version, ArgMatches
};
use scratch_genetic::genetic::{
    gen_pop, test_and_sort, reproduce, load_and_predict, export_model
};
use crate::data::{
    TableEntry, NUM_INPUTS, NUM_OUTPUTS, collection_from_file, from_table_entry
};

// Neuron connection settings
pub const NEURON_ACTIVATION_THRESH: f64 = 0.60;
pub const TRAIT_SWAP_CHANCE: f64 = 0.80;
pub const WEIGHT_MUTATE_CHANCE: f64 = 0.65;
pub const WEIGHT_MUTATE_AMOUNT: f64 = 0.5;
pub const OFFSET_MUTATE_CHANCE: f64 = 0.25;
pub const OFFSET_MUTATE_AMOUNT: f64 = 0.05;

// Neural network settings
pub const LAYER_SIZES: [usize; 4] = [ 8, 32, 32, 16 ];

// Algortithm settings
const POP_SIZE: usize = 2000;

const DATA_FILE_NAME: &'static str = "sonnets.csv";
const MODEL_FILE_NAME: &'static str = "model.sg";
const NUM_GENS: usize = 100;

// Entry point
#[tokio::main]
async fn main() {
    let args = get_args();

    if !args.is_present("predict") {
        train().await;
    } else {
        predict(args.value_of("predict").unwrap()).await;
    }
}

// Train on march madness legacy data
pub async fn train() {
    println!("Training new Shakespeare Generator Model");

    println!("Loading training data from {}", DATA_FILE_NAME);
    let games = collection_from_file(DATA_FILE_NAME);

    println!("Generating randomized population");
    let now = Instant::now();
    let mut pop = gen_pop(
        POP_SIZE,
        LAYER_SIZES.to_vec(), NUM_INPUTS, NUM_OUTPUTS,
        NEURON_ACTIVATION_THRESH, TRAIT_SWAP_CHANCE,
        WEIGHT_MUTATE_CHANCE, WEIGHT_MUTATE_AMOUNT,
        OFFSET_MUTATE_CHANCE, OFFSET_MUTATE_AMOUNT
    ).await;
    let elapsed = now.elapsed();
    println!("Generation took {}s", elapsed.as_secs_f64());

    println!("Starting training");
    for i in 0..NUM_GENS {
        println!("Generation {} / {}", i, NUM_GENS);
        test_and_sort(&mut pop, &games).await;
        reproduce(&mut pop).await;
    }

    // Save algorithm
    println!("Saving model to {}", MODEL_FILE_NAME);
    export_model(MODEL_FILE_NAME, &pop[0]).await;
}

// Load in a model and make a prediction
pub async fn predict(line: &str) {
    println!("Converting input into data...");
    let entry = TableEntry {
        initial_line: String::from(line),
        following_line: String::from("")
    };
    let game = from_table_entry(&entry);

    println!("Predicting!");
    let mut result_invalid = load_and_predict(MODEL_FILE_NAME, &game.0).await;

    // Convert to ASCII
    for byte in result_invalid.iter_mut() {
        *byte &= 0x7F;
    }

    let result = result_invalid.as_slice();

    println!("'{}'", from_utf8(result).expect("Invalid result!"));
}

// Note that data in Game prediction should be alphabetical team name
fn get_args() -> ArgMatches {
    Command::new("mmp")
        .version(crate_version!())
        .author("Dylan Turner <dylantdmt@gmail.com>")
        .about("Shakespeare Generator")
        .arg(
            Arg::new("predict")
                .short('p')
                .long("predict")
                .takes_value(true)
                .help("Switches application to prediction mode")
        ).get_matches()
}

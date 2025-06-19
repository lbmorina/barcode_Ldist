extern crate levenshtein;
use itertools::Itertools;
use levenshtein::levenshtein;
use std::{collections::HashMap, env, fs, time::Instant};

// run like: cargo run -- {barcode.file.txt}

/*
 can use ./data/toy_example.txt for testing
 or ~/code/programs/test{100/1k/10k}_bcs.txt
 or ~/code/programs/3M-february-2018.txt
 or /mithril/Data/Nanopore/projects/visium-v1.txt
*/

// we can make this function faster by inserting value i+j Ls (diagnonal)
/// Calculates the L distance of barcode list and returns hashmap of
fn calc_lev(barcodes: Vec<&str>) -> HashMap<usize, usize> {
    let mut result: HashMap<usize, usize> = HashMap::new();
    let mut start: usize = 1_usize;
    for bc in barcodes.iter() {
        for i in start..barcodes.len() {
            //println!("{},{},{},{},{}",i,idx,bc,&barcodes[start as usize],huh);
            let val = result.entry(levenshtein(bc, barcodes[i])).or_insert(0);
            *val += 1;
        }
        start += 1;
    }
    result
}

// new has nested hashmap
fn calc_lev_hashmap(barcodes: Vec<&str>) -> HashMap<&str, HashMap<&str, usize>> {
    let mut result: HashMap<&str, HashMap<&str, usize>> = HashMap::new();

    for bc in barcodes.iter() {
        let mut bc_map: HashMap<&str, usize> = HashMap::new();
        for bc2 in barcodes.iter() {
            bc_map.insert(bc2, levenshtein(bc, bc2));
        }
        result.insert(bc, bc_map);
    }
    result
}

fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let bcs = fs::read_to_string(&args[1]).expect("Error reading in the file");
    let mut bcs: Vec<&str> = bcs.split("\n").collect();
    if bcs.last().unwrap().chars().count() == 0 {
        bcs.pop();
    } // removes the last entry of vector if 0 length
    let num_bcs = &bcs.len(); // need to establish here because it we lose `bcs` on next line
    let bcs2 = bcs.clone();
    let hashy = calc_lev(bcs); // hashmap of Len:Count

    // println!("Barcode file: {}", args[1]);
    // println!("Barcodes found: {:?}", num_bcs);
    // println!("Dist\tCount");
    // for (k, v) in hashy.iter().sorted_by_key(|x| x.0) {
    //     println!("{}\t{}", k, v)
    // }
    //dbg!(&result);

    let stop = now.elapsed();
    // println!("Time Elapsed: {:.2?}", stop);
    // Now let's write to a file?
    // println!();

    // println!("Testing out dictionary thing");
    let yeet = calc_lev_hashmap(bcs2);

    print!("Barcode");
    for (bc, _) in yeet.iter().sorted_by_key(|x| x.0) {
        print!("\t{}", bc);
    }
    println!();

    for (bc1, bc2_map) in yeet.iter().sorted_by_key(|x| x.0) {
        print!("{}", bc1);
        for (_, l_dist) in bc2_map.iter().sorted_by_key(|x| x.0) {
            print!("\t{}", l_dist);
        }
        println!();
    }
}

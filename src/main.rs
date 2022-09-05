extern crate levenshtein;
use levenshtein::levenshtein;
use itertools::Itertools;
use std::{time::Instant,fs,env,collections::HashMap};


// can use ./hmm.txt for testing 
// or ~/code/programs/test{100/1k/10k}_bcs.txt 
// or ~/code/programs/3M-february-2018.txt 
// or /mithril/Data/Nanopore/projects/visium-v1.txt

fn calc_lev(s:&str, vecy:&Vec<&str>)->Vec<usize>{
    let mut l_vec:Vec<usize> = vec![0;vecy.len()];
    for (i,bc) in vecy.iter().enumerate(){
        //println!("{}\t{}",i,&bc);
        l_vec[i]=levenshtein(&s,&bc);
    }
    l_vec
}

fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let bcs = fs::read_to_string(&args[1]).expect("Error reading in the file");
    let mut bcs:Vec<&str>=bcs.split("\n").collect();
    bcs.pop();
    let mut l_vecy = Vec::new();
    
    //dbg!(&bcs[]);

    for bc in bcs.iter(){
        l_vecy.push(calc_lev(&bc,&bcs).to_vec());
    }

    //println!("l_vecy is: {:?}",&l_vecy);

    let mut result: HashMap<usize,usize> = HashMap::new();
    for row in l_vecy.iter(){
        for key in row {
            let val = result.entry(*key).or_insert(0);
            *val += 1;
        }
    }
    println!("{:?} Barcodes found",result.get(&0).unwrap());
    println!("Dist\tCount");
    for (k,v) in result.iter().sorted_by_key(|x| x.0){
        println!("{}\t{}",k,v)
    }
    //dbg!(&result);

    let stop = now.elapsed();
    println!("Time Elapsed: {:.2?}",stop);
    // Now let's write to a file
    
}
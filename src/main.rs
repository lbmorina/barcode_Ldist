extern crate levenshtein;
use levenshtein::levenshtein;
use itertools::Itertools;
use std::{time::Instant,fs,env,collections::HashMap};


/* 
 can use ./hmm.txt for testing 
 or ~/code/programs/test{100/1k/10k}_bcs.txt 
 or ~/code/programs/3M-february-2018.txt 
 or /mithril/Data/Nanopore/projects/visium-v1.txt
*/

// we can make this function faster by inserting value i+j Ls (diagnonal)

fn calc_lev(barcodes:Vec<&str>)->HashMap<usize,usize>{
    let mut result:HashMap<usize,usize> = HashMap::new();

    let mut start:i32 = 1;
    let end:i32 = (barcodes.len()) as i32;
    for bc in barcodes.iter(){
        for _ in start..end{
            //println!("{},{},{},{},{}",i,idx,bc,&barcodes[start as usize],huh);
            let val = result.entry(levenshtein(bc,&barcodes[start as usize])).or_insert(0);
                *val += 1; 
        }
        start +=1;
    }
    result
}

fn main(){
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let bcs = fs::read_to_string(&args[1]).expect("Error reading in the file");
    let mut bcs:Vec<&str>=bcs.split("\n").collect();
    if bcs.last().unwrap().chars().count() == 0 {bcs.pop();}
    let num_bcs = &bcs.len();
    let hashy = calc_lev(bcs); // first argument is identity matrix cuz we only need to fill upper tri
   
    println!("{:?} Barcodes found",num_bcs);
    println!("Dist\tCount");
    for (k,v) in hashy.iter().sorted_by_key(|x| x.0){
        println!("{}\t{}",k,v)
    }
    //dbg!(&result);

    let stop = now.elapsed();
    println!("Time Elapsed: {:.2?}",stop);
    // Now let's write to a file?
    println!()

}
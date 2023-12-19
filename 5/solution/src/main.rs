use std::fs::File;
use std::io::{prelude::*, BufRead};
use std::iter;
use std::path::Iter;


#[derive(Debug)] //allow print
struct ValuetoValue{
    from:Vec<u32>,
    into:Vec<u32>,
}

fn find_seeds(line:String) -> Vec<u32>{
    let index = line.find(':').unwrap();
    let line = &line[index+2..];
    
    let mut rtn:Vec<u32> = vec![];
    for seed in line.split(' ').into_iter(){
        let i:u32 = seed.parse::<u32>().unwrap();
        rtn.push(i);
    }
    rtn
}

fn pull_values(mut input:std::str::Lines<'_>) -> ValuetoValue{
    let mut from:Vec<u32>= vec![];
    let mut into:Vec<u32> = vec![];
    //skip first line
    input.next();
    
    loop {
        match input.next() {
            None => return ValuetoValue { from: from, into: into},
            Some("") => return ValuetoValue { from: from, into: into},
            Some(line) => {  
                let mut split = line.split_ascii_whitespace();
                let first = split.next().unwrap().parse::<u32>().unwrap();
                let second = split.next().unwrap().parse::<u32>().unwrap();
                let increment = split.next().unwrap().parse::<u32>().unwrap();
                
                for x in 0..=increment{
                    from.push(first+x);
                    into.push(second+x);
                }
            }
        }
    }    
}

fn main() {
    //Pull file:
    let dir = "O:\\Code\\Rust\\AdventOfCode\\5\\example.txt".to_string();
    let mut file = File::open(dir).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();    
    let mut iter: std::str::Lines<'_> = contents.lines();

    let seeds = find_seeds(iter.next().unwrap().to_string());
    println!("Seeds: {:#?}",seeds);
    iter.next();//skip line  

    let seedtosoil = pull_values(iter);
    iter.next();//skip line  
    let soiltofert = pull_values(iter);
    println!("Seed -> Soil {:#?}",seedtosoil);
    println!("Soil -> Fert {:#?}",soiltofert);

    /* 
    seed-to-soil map
    soil-to-fertilizer map
    fertilizer-to-water map:
    water-to-light map:
    light-to-temperature map:
    temperature-to-humidity map:
    humidity-to-location map:
    */
}

use std::fs::File;
use std::io::prelude::*;

fn vert_sym_check_line(line:&str) -> Vec<u32>{
    let mut symetries:Vec<u32> = vec![];
    //check left == right 
    let len = line.len();
    for x in 1..len-1{            
        let left = &line[0..x];
        let left:String = left.chars().rev().collect();
        let right = &line[x..len-1];
        let check_len = std::cmp::min(right.len(),left.len());

        if right[0..check_len] == left[0..check_len] && check_len !=0{
            println!("{:#?} == {:#?} found vert sym @ {}", left[0..check_len].to_string(), right[0..check_len].to_string(), x);
            symetries.push(x as u32);
        }
    }
    return symetries;

}

fn vertical_sym_check(input:String){
    let mut solutions:Vec<u32> = vec![];

    for line in input.split('\n'){
        let line_check = vert_sym_check_line(line);
        if solutions.is_empty(){
            solutions = line_check;
        }
        else {
            solutions.(other)
            for (i,x) in solutions.iter().enumerate(){
                if !line_check.contains(x){
                    solutions.remove(i);
                }
            }
        }

        
        println!("End Line" );
    }
}

fn main() {
    //Pull file:
    let dir = "O:\\Code\\Rust\\AdventOfCode\\13\\example.txt".to_string();
    let mut file = File::open(dir).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    vertical_sym_check(contents);
    
    //for maze in contents.split("\n\n"){
     //   println!("{}",maze);
    //}
}

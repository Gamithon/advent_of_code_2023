use std::fs::File;
use std::io::prelude::*;

fn expand_number(line:Vec<char>, col:usize) -> u32{
    let mut sum:u32 = 0;
    for (i,c) in line.iter().enumerate(){
        if i>col && !c.is_numeric() {
            return sum;
        }
        if c==&'.'{
            sum = 0;
        }
        else if c.is_numeric(){
            sum*=10;
            sum+= *c as u32-0x30;
        }
    }
    return sum;//In a more official codebase this would be an error
}

//search +/- 1 of the value for numbers
fn find_numbers(input:Vec<Vec<char>>, row:usize, col:usize) ->u32{
    //possible issue if 2 numbers are on a symbol
    let mut sum:u32 = 0;
    let mut prev:u32 = 0;

    for x in row-1..=row+1{
        for y in col-1..=col+1{
            if input[x][y].is_numeric(){
                println!("found #{}  @{},{}",input[x][y],x,y);
                //set col to prevent double count
                let full_num = expand_number(input[x].clone(),y);
                //hacked double count fix....
                if full_num != prev{                
                    sum+=full_num;
                    println!("Added #: {}",full_num);
                }
                prev = full_num
            }
        }
    }
    return sum;
}

fn part1(input:Vec<Vec<char>>){
    //turn text into 2d string array
    //Search each line for a number
    //Search perimiter for special symbols

    let mut sum:u32 =0;
    for (row,line) in input.iter().enumerate(){
        for (col,c) in line.iter().enumerate(){
            if  c.is_numeric(){
                //find special characters
                let (start, end) = find_number_range()
                sum += find_numbers(input.clone(),row,col);
            }
        }
    }
    //217247
    //503376
    println!("SUM: {}",sum);
}

fn main() {
    //Pull file:
    let dir = "O:\\Code\\Rust\\AdventOfCode\\3\\key.txt".to_string();
    let mut file = File::open(dir).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut input:Vec<Vec<char>> = vec![];
    for line in contents.lines(){
        let vec:Vec<char> = line.chars().collect();
        input.push(vec);
    }
    //let input = input;

    println!("PART 1");
    part1(input.clone());
 }

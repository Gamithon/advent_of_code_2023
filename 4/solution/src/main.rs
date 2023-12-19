use std::fs::File;
use std::io::prelude::*;

fn game_score(game:String ) -> u32{
    //remove Game #:
    let index = game.find(':').unwrap();
    let game = &game.to_string()[index+1..];
    println!("{}",game);

    
    let index = game.find('|').unwrap();
    let s = game.to_string();
    let mut winning_nums = s[..index].split_whitespace();
    let ticket_nums = s[index+1..].split_whitespace();

    let mut sum = 0;
    for ticket_num in ticket_nums{
        match winning_nums.clone().into_iter().find( |&s| s ==ticket_num){
            Some(_) => sum+=1,
            None => (),
        }
    }
    if sum == 0{
        return 0;
    }
    let base:u32 = 2;
    return base.pow(sum-1)
}

fn main() {
    //Pull file:
    let dir = "O:\\Code\\Rust\\AdventOfCode\\4\\key.txt".to_string();
    let mut file = File::open(dir).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    //PART1
    let mut sum:u32 = 0;
    for line in contents.lines(){
        sum+= game_score(line.to_string());
    }
    println!("{}",sum);

    //PART2
    
}

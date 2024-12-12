use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();
    for l in reader.lines() {
        match l {
            Ok(line) => {
                let mut split = line.split("   ");
                let num1s = split.next().unwrap();
                let num1 = num1s.parse::<i32>().unwrap();
                vec1.push(num1);
                let num2s = split.next().unwrap();
                let num2 = num2s.parse::<i32>().unwrap();
                vec2.push(num2);
            }
            Err(_) => ()
        }
    }
    vec1.sort();
    vec2.sort();
    let mut sum = 0;
    //parte A
    for i in 0..vec1.len() {
        let diff = (vec1[i] - vec2[i]).abs();
        sum = sum + diff;
    }
    //parte B
    let mut sim = 0;
    for val in vec1 {
        //buscar el valor en el otro vector, aunque sea el mismo
        let count = vec2.iter().filter(|x| **x == val).count();
        sim = sim + (val * count as i32);
    }
    println!("Distancia: {}", sum);
    println!("Similaridad: {}", sim);
    Ok(())
}

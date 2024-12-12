use core::num;
use std::fs::{read, File};
use std::io::{BufRead, BufReader};

pub fn read_line(line: &String) -> Vec<i32> {
    //llega una linea de numeros, devuelve el vector
    let numbers = line.split(" ");
    let mut vec_numbers: Vec<i32> = Vec::new();
    for number in numbers {
        vec_numbers.push(number.parse::<i32>().unwrap());
    }
    vec_numbers
}
//determina si una linea es buena o no
pub fn is_ok(vec: &Vec<i32>) -> bool {
    let mut asd = true; //suponemos que es ascendente
    if vec[0] > vec[1] {
        asd = false; //vamos abajo
        if vec[0] - vec[1] > 3 {
            return false 
        }
    }
    if vec[0] < vec[1] {
        asd = true;
        if vec[1] - vec[0] > 3 { //vamos arriba
            return false 
        }
    }
    if vec[0] == vec[1] {
        return false
    }
    for i in 2..vec.len() {
        if vec[i-1] < vec[i] && !asd {
            return false
        }
        else if vec[i-1] > vec[i] && asd {
            return false
        }
        else if vec[i-1] == vec[i] {
            return false
        }
        else if (vec[i-1] - vec[i]).abs() > 3 {
            return false
        }
        
    }
    true
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;

    let lines = BufReader::new(file).lines();
    let mut valid = 0;
    for line in lines {
        let vec = read_line(&line.unwrap());
        for num in 0..vec.len() {
            let mut vec_copy = vec.clone();
            vec_copy.remove(num);
            if is_ok(&vec_copy) {
                valid = valid + 1;
                break; //deja de probar, suma y sigue
            }
        }

    }
    println!("{}", valid);
    Ok(())
}

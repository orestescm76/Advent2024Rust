use std::fs::read_to_string;
use std::io::Result;

fn find_guard(map:&Vec<String>) -> (usize, usize) {
    let mut i = 0;
    let mut j = 0;
    for line in map {
        j = 0;
        for char in line.chars() {
            if char == '^' {
                return (i,j);
            }
            j+=1;
        }
        i+=1;
    }
    i = usize::MAX;
    j = usize::MAX;
    (i,j)
}

fn has_left(map: &mut Vec<Vec<char>>, coords: (usize, usize)) -> Option<&mut char> {
    match map.get_mut(coords.0) {
        Some(line) => line.get_mut(coords.1),
        None => None,
    }
}

fn main() -> Result<()>{
    let lines = read_to_string("input.txt")?;
    let file : Vec<String> = lines.lines().map(String::from).collect();
    let mut coords = find_guard(&file);
    let mut map : Vec<Vec<char>> = Vec::new();

    for line in file  {
        map.push(line.chars().collect());
    }
    let mut direction = 0; //0 arriba, 1 derecha, 2 abajo, 3 izq
    let mut ans = 1;
    //[Y][X]
    loop {
        if coords.0 == 0 || coords.1 == 0 {
            break;
        }
        match direction {
            0 => coords.0 -= 1,
            1 => coords.1 += 1,
            2 => coords.0 += 1,
            3 => coords.1 -= 1,
            _ => {}
        }
        match has_left(&mut map, coords) {
            Some(map_char) => {
                match map_char {
                    '#' => {
                        match direction {
                            0 => coords.0 += 1,
                            1 => coords.1 -= 1,
                            2 => coords.0 -= 1,
                            3 => coords.1 += 1,
                            _ => {}
                        }
                        direction = (direction + 1) % 4;
                    }
                    '.' => {
                        ans += 1;
                        *map_char = 'X';
                    }
                    _ => {}
                }
            }
            None => break,
        }
    }
    println!("ans = {}", ans);
    Ok(())
}

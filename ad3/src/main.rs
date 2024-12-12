use std::fs::read_to_string;
use std::io;

fn mul(a:i32, b:i32) -> i32 {
    a * b
}

fn main() -> io::Result<()>{
    let input = read_to_string("input.txt")?;
    //let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let mut total = 0;
    let dos_string = input.split("do()");
    for dos in dos_string {
        let mut donts_string = dos.split("don't()");
        let dont_izq = donts_string.next().unwrap();
        let muls = dont_izq.split("mul(");
        for coms in muls {
            let fin = coms.split(")");
            for numbers in fin {
                //now filter
                let numbers_separated = numbers.split(",");
                let mut vec:Vec<i32> = Vec::new();
                for n in numbers_separated {
                    let a = n.parse::<i32>();
                    match a {
                        Ok(num) => vec.push(num),
                        Err(_parse_int_error) => {},
                    }
                }
                if vec.len() == 2 {
                    total = total + mul(vec[0],vec[1]);
                }
            }
        }

    }
    println!("{}", total);
    Ok(())
}

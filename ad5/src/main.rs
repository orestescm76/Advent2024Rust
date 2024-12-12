use std::fs::read_to_string;
use std::io::Result;

fn read_rule(rule:&str) -> (i32, i32) {
    let mut split = rule.split("|");
    let n1 = split.next().unwrap().parse::<i32>().unwrap(); //no deberia fallar
    let n2 = split.next().unwrap().parse::<i32>().unwrap(); 
    (n1,n2)
}
fn check_page(rules: &Vec<(i32,i32)>, rule: &str) -> (bool, i32) {
    let split = rule.split(",");
    let mut pages = Vec::new();
    for num in split {
        pages.push(num.parse::<i32>().unwrap());
    }
    //check rules
    
    let mut valid = true;

    //mirar cuales fallan
    for rule in rules {
        let mut iter_pages = pages.iter();
        let op_i1 = iter_pages.position(|x| x == &rule.0);
        let mut iter_pages = pages.iter(); 
        let op_i2 = iter_pages.position(|y| y == &rule.1);
        if let Some(i1) = op_i1 {
            if let Some(i2) = op_i2 {
                if i1 > i2 {
                    valid = false;
                    //fix it
                    //pages.swap(i1, i2);
                    //println!("{:?}", pages);

                }
            }
        }
    }

    if !valid {
        fix_page(rules, &mut pages);
        return (true, pages[pages.len()/2])
    }
    
    (false, 0)
    // if valid {
    //     return (true, pages[pages.len()/2])
    // }
    // else {
       

    // }
}
fn fix_page(rules:&Vec<(i32,i32)>, page: &mut Vec<i32>) {
    for rule in rules {
        let mut iter_pages = page.iter();
        let op_i1 = iter_pages.position(|x| x == &rule.0);
        let mut iter_pages = page.iter(); 
        let op_i2 = iter_pages.position(|y| y == &rule.1);
        if let Some(i1) = op_i1 {
            if let Some(i2) = op_i2 {
                if i1 > i2 {
                    page.swap(i1, i2);
                    fix_page(rules, page);
                    //fix it
                    //println!("{:?}", pages);

                }
            }
        }
    }
}
fn main() -> Result<()> {
    let file = read_to_string("input.txt")?;
    let mut rules : Vec<(i32,i32)> = Vec::new();
    let mut reading_rules = true;
    let mut ans = 0;
    for line in file.lines() {
        if line.is_empty() {
            reading_rules = false;
            continue;
        }
        //leer reglas
        if reading_rules {
            rules.push(read_rule(line));
        }
        else {
            //check if the pages are OK
            let res = check_page(&rules, line); 
            if res.0 {
                ans = ans+res.1;
            }
        }

    }
    println!("{}",ans);
    Ok(())
}

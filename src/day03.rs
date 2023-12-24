use std::fs::read_to_string;

#[test]
fn part1a() -> Result<(), String>{
    assert_eq!(run_part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..")?,4361);
    Ok(())
}
#[test]
fn tests() -> Result<(), String>{
    let tests = [
        ("*1", 1),
        ("...\n.1.\n...", 0),
        ("*..\n..1\n...", 0),
        ("...*\n10..\n....", 0),
        ("*...\n..10\n....", 0),        
        ("....\n10..\n...*", 0),
        ("....\n..10\n*...", 0),
    ];
    for (s,count) in tests{
        assert_eq!(run_part1(s)?, count);
    }
    Ok(())
}
#[test]
fn part1b() -> Result<(), String>{
    let str = read_to_string("inputs/day03.txt").map_err(|x|x.to_string())?;
    let result = run_part1(&str)?;
    assert_eq!(result, 543867);
    Ok(())
}

fn run_part1(input: &str) -> Result<i32, String>{   
    let mut symbol_lines : Vec<Vec<Symbol>> = Vec::new();
    let mut index_lines : Vec<Vec<usize>> = Vec::new();
    for line in input.lines(){
        let symbols = parse_symbols(line);
        let mut indexes = Vec::new();
        for sym in &symbols{
            match sym.ty{
                Type::Symbol => {
                    indexes.push(sym.start);
                },
                _ => {},
            }
        }
        index_lines.push(indexes);
        symbol_lines.push(symbols);
    } 

    let mut sum = 0;
    for index in 0..symbol_lines.len(){
        let symbol_line = &symbol_lines[index];
        
        let current = &index_lines[index];
        
        let mut prev = None;
        if index > 0{
            prev = Some(&index_lines[index-1]);
        }

        let mut next = None;
        if index < index_lines.len() -1 {
            next = Some(&index_lines[index+1]);
        }

        for sym in 0..symbol_line.len(){
            let symbol = &symbol_line[sym];
            match symbol.ty{
                Type::Number => {},
                _ => {continue;}
            } 

            let lower_bound = if symbol.start == 0 { 0 } else { symbol.start -1};
            let upper_bound = symbol.capture.len() + symbol.start ;
            let mut is_adjacent = false;
            
            // if there is a previous line, process it
            if prev.is_some(){
                for p in prev.unwrap(){
                    if *p > upper_bound{
                        break;
                    }
                    if *p >= lower_bound && *p <= upper_bound{
                        is_adjacent = true;
                        break;
                    }
                }
            }

            if !is_adjacent{
                // find if any symbols are in the range of this item in the current
                for c in current{
                    if *c > upper_bound{
                        break;
                    }
                    if *c >= lower_bound && *c <= upper_bound{
                        is_adjacent = true;
                        break;
                    }
                }
            }

            // if there is a next line, process it
            if next.is_some() && !is_adjacent{
                for n in next.unwrap(){
                    if *n > upper_bound{
                        break;
                    }
                    if *n >= lower_bound && *n <= upper_bound{
                        is_adjacent = true;
                        break;
                    }
                }
            }

            // if adjacent, add to sum
            if is_adjacent{
                match symbol.capture.parse::<i32>(){
                    Ok(n) => sum += n,
                    Err(e) => { 
                        return Err(e.to_string())
                    }
                }
            }
        }
    }
    Ok(sum)
}

enum Type {
    Symbol,
    Number,
    Dots,
}
struct Symbol {
    capture: String,
    start: usize,
    ty: Type,
}

fn parse_symbols(line: &str) -> Vec<Symbol>{
    let mut symbols = Vec::new();
    let mut iter = line.char_indices().peekable();
    while let Some((start, ch)) = iter.clone().peek(){
        match ch{
            '.' =>{
                let mut end = 0;
                while let Some((p,_)) = iter.next_if(|(_,c)|*c == '.'){
                    end = p;
                }
                let capture = &line[*start..=end];
                symbols.push(Symbol{ start: *start, capture: capture.to_string(), ty: Type::Dots});
            }
            ch if ch.is_numeric() =>{
                let mut end = 0;
                while let Some((p, _)) = iter.next_if(|(_,c)|c.is_numeric()){
                    end = p;
                }
                let capture = &line[*start..=end];
                symbols.push(Symbol{start: *start, capture: capture.to_string(), ty: Type::Number});
            }
            _ => {
                _ = iter.next();
                symbols.push(Symbol{start: *start, capture: ch.to_string(), ty: Type::Symbol});
            }
        }
    }
    symbols
}

#[test]
fn part2a(){}

#[test]
fn part2b(){}

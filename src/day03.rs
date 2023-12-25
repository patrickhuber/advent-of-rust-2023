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
fn part1_tests() -> Result<(), String>{
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
    let mut segment_lines : Vec<Vec<Segment>> = Vec::new();
    let mut index_lines : Vec<Vec<usize>> = Vec::new();
    for line in input.lines(){
        let segments = parse_segments(line);
        let mut indexes = Vec::new();
        for seg in &segments{
            match seg.ty{
                Type::Symbol => {
                    indexes.push(seg.start);
                },
                _ => {},
            }
        }
        index_lines.push(indexes);
        segment_lines.push(segments);
    } 

    let mut sum = 0;
    for index in 0..segment_lines.len(){
        let segment_line = &segment_lines[index];
        
        let current = &index_lines[index];
        
        let mut prev = None;
        if index > 0{
            prev = Some(&index_lines[index-1]);
        }

        let mut next = None;
        if index < index_lines.len() -1 {
            next = Some(&index_lines[index+1]);
        }

        for seg in 0..segment_line.len(){
            let symbol = &segment_line[seg];
            match symbol.ty{
                Type::Number => {},
                _ => {continue;}
            } 

            let lower_bound = lower_bound(symbol);
            let upper_bound = upper_bound(symbol);
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


#[test]
fn part2a() -> Result<(), String>{
    assert_eq!(run_part2("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..")?, 467835);
    Ok(())
}

#[test]
fn part2_tests() -> Result<(), String>{
    let tests = [
        ("1*1", 1),
        ("...\n.1.\n...", 0),
        ("*..\n..1\n...", 0),
        ("10..\n...*\n10..", 0),
        ("10..\n*10.\n10..",0),
        ("....\n2*2.\n....",4),
        ("10..\n..*.\n10..",100),
    ];
    for (s,count) in tests{
        assert_eq!(run_part2(s)?, count);
    }
    Ok(())
}

#[test]
fn part2b() -> Result<(), String>{
    let str = read_to_string("inputs/day03.txt").map_err(|x|x.to_string())?;
    let result = run_part2(&str)?;
    assert_eq!(result, 79613331);
    Ok(())
}

fn run_part2(input: &str) -> Result<usize, String>{
    let mut star_lines = Vec::new();
    let mut number_lines = Vec::new();

    for line in input.lines(){
        let segments = parse_segments(line);
        let mut stars = Vec::new();
        let mut numbers = Vec::new();
        for seg in segments{
            match seg.ty{
                Type::Number=> numbers.push(seg),
                Type::Symbol if seg.capture == "*"=>{ stars.push(seg);},
                _ => {}
            }
        }
        star_lines.push(stars);
        number_lines.push(numbers);
    }

    let mut sum = 0;
    for line_no in 0..star_lines.len(){
        let mut prev = None;
        let mut next = None;
        if line_no > 0 { 
            prev = Some(&number_lines[line_no-1]);
        }
        if line_no < number_lines.len() -1 { 
            next = Some(&number_lines[line_no+1]);
        }
        let stars = &star_lines[line_no];
        let current = &number_lines[line_no];

        for star in stars{
            let mut numbers: Vec<usize> = Vec::new();
            if prev.is_some(){                
                for num in prev.unwrap(){
                    let lower_bound = lower_bound(num);
                    let upper_bound = upper_bound(num);
                    if lower_bound <= star.start && star.start <= upper_bound{
                        numbers.push(num.capture.parse::<usize>().unwrap());
                    }
                }
            }

            for num in current{
                let lower_bound = lower_bound(num);
                let upper_bound = upper_bound(num);
                if lower_bound <= star.start && star.start <= upper_bound{
                    numbers.push(num.capture.parse::<usize>().unwrap());
                }
            }
            
            if next.is_some(){
                for num in next.unwrap(){
                    let lower_bound = lower_bound(num);
                    let upper_bound = upper_bound(num);
                    if lower_bound <= star.start && star.start <= upper_bound{                        
                        numbers.push(num.capture.parse::<usize>().unwrap());
                    }
                }
            }

            if numbers.len() == 2{
                sum += numbers[0] * numbers[1];
            }
        }
        
    }
    Ok(sum)
}

fn upper_bound(seg: &Segment) -> usize{    
    seg.capture.len() + seg.start
}

fn lower_bound(seg: &Segment) -> usize{
    if seg.start == 0 { 
        0 
    } else { 
        seg.start -1
    }
}

enum Type {
    Symbol,
    Number,
    Dots,
}
struct Segment {
    capture: String,
    start: usize,
    ty: Type,
}

fn parse_segments(line: &str) -> Vec<Segment>{
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
                symbols.push(Segment{ start: *start, capture: capture.to_string(), ty: Type::Dots});
            }
            ch if ch.is_numeric() =>{
                let mut end = 0;
                while let Some((p, _)) = iter.next_if(|(_,c)|c.is_numeric()){
                    end = p;
                }
                let capture = &line[*start..=end];
                symbols.push(Segment{start: *start, capture: capture.to_string(), ty: Type::Number});
            }
            _ => {
                _ = iter.next();
                symbols.push(Segment{start: *start, capture: ch.to_string(), ty: Type::Symbol});
            }
        }
    }
    symbols
}

use std::fs::read_to_string;

#[test]
fn part1a() -> Result<(), String>{
    assert_eq!(run_part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")?, 8);
    Ok(())
}

#[test]
fn part1b() -> Result<(), String>{
    let str = read_to_string("inputs/day02.txt").map_err(|x|x.to_string())?;
    let result = run_part1(&str)?;
    assert_eq!(result, 2776);
    Ok(())
}

fn run_part1(input : &str) -> Result<i32, String>{    
    let mut sum = 0;
    for line in input.lines(){
        let game = parse_game(line.trim().to_string())?;
        let mut valid = true;
        for subset in &game.subsets{
            valid = subset.red <=12 && subset.blue <=14 && subset.green <=13;
            if !valid{
                break;
            }
        }
        if !valid{
            continue;
        }        
        sum += game.id;
    }
    Ok(sum)
}

#[test]
fn part2a() -> Result<(), String>{
    assert_eq!(run_part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")?, 2286);
    Ok(())
}

#[test]
fn part2b() -> Result<(), String>{
    let str = read_to_string("inputs/day02.txt").map_err(|x|x.to_string())?;
    let result = run_part2(&str)?;
    assert_eq!(result, 68638);
    Ok(())
}

fn run_part2(input : &str) -> Result<i32, String>{
    let mut sum = 0;
    for line in input.lines(){
        let game = parse_game(line.trim().to_string())?;
        let mut max = Subset{ red:0,green:0,blue:0}; 
        for sub in game.subsets{
            if sub.red > max.red {
                max.red = sub.red;
            }
            if sub.green > max.green{
                max.green = sub.green;
            }
            if sub.blue > max.blue{
                max.blue = sub.blue;
            }
        }
        let pow = max.red * max.green * max.blue;
        sum += pow;
    }
    Ok(sum)
}

struct Game {
    id: i32,
    subsets: Vec<Subset>
}

struct Subset{
    red: i32,
    green: i32,
    blue:i32
}

struct ColorCount{
    color : String,
    count:i32,
}

fn parse_game(line: String) -> Result<Game, String>{
    let splits = line.split(':').collect::<Vec<_>>();
    if splits.len() != 2{
        return Err(String::from("expected 1 semicolon"))
    }    
    let id = parse_id(splits[0].to_string())?;
    let subsets = parse_subsets(splits[1].to_string())?;
    return Ok(Game{ id, subsets})
}

fn parse_id(seg: String) -> Result<i32, String>{
    let splits = seg.split(|x:char|x.is_whitespace()).collect::<Vec<_>>();
    if splits.len() != 2{
        return Err(String::from("expected format 'Game <id>'"))
    }
    if splits[0] != "Game"{
        return Err(String::from("expected 'Game'"))
    }
    let id = splits[1].trim().parse::<i32>();
    match id{
        Ok(i) => return Ok(i),
        Err(_)=> return Err(String::from("unable to parse id"))
    }
}

fn parse_subsets(seg: String) -> Result<Vec<Subset>,String>{
    let splits = seg.split(';');
    let mut subsets = Vec::new();
    for split in splits{
        let subset = parse_subset(split.to_string())?;
        subsets.push(subset);
    }
    Ok(subsets)
}

fn parse_subset(seg: String) -> Result<Subset, String>{
    let splits = seg.split(',');
    let mut subset = Subset{
        blue:0,
        red:0,
        green:0,
    };

    for split in splits{
        let color_count = parse_color_count(split.to_string())?;
        let count = color_count.count;
        match color_count.color.as_str(){
            "red" => subset.red += count,
            "blue" => subset.blue += count,
            "green" => subset.green += count,
            _=>{},
        }
    }
    Ok(subset)
}

fn parse_color_count(seg: String) -> Result<ColorCount, String>{
    let splits = seg.trim().split(|x:char| x.is_whitespace()).collect::<Vec<_>>();
    return Ok(ColorCount{
        color: splits[1].to_string(),
        count: splits[0].parse::<i32>().map_err(|x|x.to_string())?
    })
}
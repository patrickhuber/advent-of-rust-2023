use std::{collections::HashMap, str::Lines, fs::read_to_string};

const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[test]
fn part1a() -> Result<(), String>{
    assert_eq!(run_part1(INPUT)?, 35);
    Ok(())
}

#[test]
fn part1b()-> Result<(), String>{
    let str = read_to_string("inputs/day05.txt").map_err(|x|x.to_string())?;
    let result = run_part1(&str)?;
    assert_eq!(result, 806029445);
    Ok(())
}

fn run_part1(input: &str) -> Result<u64, String>{
    let almanac = parse_almanac(input)?;
    let mut results = almanac.seeds.clone();

    let map_names = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location"];

    for map_name in map_names{
        let lines = almanac.maps.get(map_name).ok_or("missing map")?;
        for index in 0..results.len(){
            let item = results[index];
            for line in lines{
                // if the item is in the range
                if line.source <= item && item <= line.source + line.length{
                    // get the diff between the item and source start
                    // apply the diff to the destination start
                    results[index] = item - line.source + line.destination;
                    break;
                }
            }
        }
    }
    Ok(*results.iter().min().unwrap())
}

#[test]
fn part2a(){}

#[test]
fn part2b(){}

struct Almanac{
    seeds: Vec<u64>,
    maps: HashMap<String, Vec<Line>>,
}
struct Line {
    source: u64,
    destination: u64,
    length: u64,
}

fn parse_almanac(input: &str) -> Result<Almanac, String> {
    // parse seeds
    let mut lines = input.lines();
    let seeds = parse_seeds(&mut lines)?;
    if lines.next().ok_or("missing empty line".to_string())?.len() > 0{
        return Err("line is not empty".to_string());
    }
    let maps = parse_maps(&mut lines)?;
    Ok(Almanac{
        seeds: seeds,
        maps: maps,
    })
}

fn parse_seeds(iter: &mut Lines<'_>) -> Result<Vec<u64>, String>{
    let next = iter.next();
    if next.is_none(){
        return Err("iterator empty".to_string())
    }
    let line = next.unwrap();
    let splits = line.split(':').collect::<Vec<_>>();
    let numbers = splits[1]
        .split(|x:char|x.is_whitespace())
        .filter_map(|x|x.parse::<u64>().ok())
        .collect::<Vec<_>>();

    Ok(numbers)
}

fn parse_maps(iter: &mut Lines<'_>) -> Result<HashMap<String, Vec<Line>>, String>{    
    let mut maps = HashMap::new();
    while let Some(_) = iter.clone().peekable().peek(){
        let (key,numbers) = parse_map(iter)?;
        maps.insert(key, numbers);
    }
    Ok(maps)
}

fn parse_map(iter: &mut Lines<'_>) -> Result<(String, Vec<Line>), String>{
    // parse the name
    let line = iter.next().ok_or("map line required".to_string())?;
    let splits = line.split(' ').collect::<Vec<_>>();
    let name = splits[0];

    // parse each line
    let mut lines = Vec::new();
    while let Some(line) = iter.next(){
        if line.trim().len() == 0{
            break;
        }
        lines.push(parse_line(line)?);
    }
    Ok((name.to_string(), lines))
}

fn parse_line(line: &str) -> Result<Line, String>{
    let splits = line
        .split(|x:char|x.is_whitespace())
        .collect::<Vec<_>>();
    
    Ok(Line{
        destination:parse_int(splits[0])?,
        source: parse_int(splits[1])?,
        length:parse_int(splits[2])?}
    )
}

fn parse_int(s: &str)-> Result<u64, String>{
    s.parse::<u64>().map_err(|x|x.to_string())   
}
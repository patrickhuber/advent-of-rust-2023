use std::{collections::{HashSet, HashMap}, fs::read_to_string};

#[test]
fn part1a() -> Result<(), String>{
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let sum = run_part1(input)?;
    assert_eq!(sum, 13);
    Ok(())
}

#[test]
fn part1b() -> Result<(), String>{
    let str = read_to_string("inputs/day04.txt").map_err(|x|x.to_string())?;
    let result = run_part1(&str)?;
    assert_eq!(result, 23673);
    Ok(())
}

fn run_part1(input: &str) -> Result<u32, String>{
    let mut sum = 0;
    for line in input.lines(){

        // parse the card
        let card = parse_card(line)?;
                
        // hash the winners
        let winners:HashSet<u32> = HashSet::from_iter(card.winners);
        let matches = card.numbers.iter()
            .filter(|x|winners.contains(x))
            .collect::<Vec<_>>();
        
        sum += if matches.len() == 0 { 
            0 
        } else { 
            2_u32.pow((matches.len() -1) as u32) 
        } ;
    }
    Ok(sum)
}

#[test]
fn part2a() -> Result<(), String>{
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let sum = run_part2(input)?;
    assert_eq!(sum, 30);
    Ok(())
}

#[test]
fn part2b() -> Result<(), String>{
    let str = read_to_string("inputs/day04.txt").map_err(|x|x.to_string())?;
    let result = run_part2(&str)?;
    assert_eq!(result, 12263631);
    Ok(())
}

fn run_part2(input: &str) -> Result<i32, String>{
    let mut cards = Vec::new();
   
    for line in input.lines(){
        let card = parse_card(line)?;
        cards.push(card);  
    }
    let mut copies: HashMap<_,_>= cards.iter().map(|c|(c.id, 1)).collect();
    let mut sum = 0;

    for card in cards{
        let winners:HashSet<u32> = HashSet::from_iter(card.winners);
        let matches = card.numbers.iter()
            .filter(|x|winners.contains(x))
            .count() as i32;
        
        let current_count = *copies.get(&card.id).unwrap_or(&1);

        // add counts for subsequent cards
        for i in 1..=matches{
            let key = i as u32 + card.id;
            match copies.get(&key){
                Some(count) => {
                    copies.insert(key, (*count) + current_count);
                },
                _ => {}
            }            
        }
        sum += current_count;
    }
    Ok(sum)
}

struct Card{
    id: u32,
    numbers: Vec<u32>,
    winners: Vec<u32>,
}
fn parse_card(line: &str) -> Result<Card, String>{
    let splits = line.split(&[':','|'][..]).collect::<Vec<_>>();
    if splits.len() != 3{
        return Err("Invalid split count when parsing".to_string());
    }
    Ok(Card { 
        winners: splits[2]
            .trim()
            .split(|x:char|x.is_whitespace())
            .filter_map(|s| s.parse::<u32>().ok())
            .collect(),
        numbers: splits[1]
            .trim()
            .split(|x:char|x.is_whitespace())
            .filter_map(|s|s.parse::<u32>().ok())
            .collect(),
        id: match splits[0].trim().split(|x:char|x.is_whitespace()).last() {
            Some(id) => { 
                id.parse::<u32>().map_err(|x|x.to_string())?
            },
            None => { return Err("unable to match id".to_string())}
        }
    })
}
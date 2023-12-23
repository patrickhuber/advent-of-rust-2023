use std::fs::read_to_string;


fn run_part1(str: &str) -> i32{
    let lines = str.lines();
    let mut sum = 0;
    for line in lines  {
        let mut rev = line.chars().rev();
        let mut chars = line.chars();
        
        let last = rev.find(|x|x.is_numeric()).unwrap_or('\0');
        let first = chars.find(|x|x.is_numeric()).unwrap_or('\0');

        // combine into a single number
        let mut num = first.to_string();
        num.push(last);
        let res = num.parse::<i32>().unwrap();
        sum += res;
    }
    sum
}

#[test]
fn part1a() -> Result<(), std::io::Error>{
    assert_eq!(run_part1("1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"), 142);
    Ok(())
}

#[test]
fn part1b() -> Result<(), std::io::Error>{
    let str = read_to_string("inputs/day01.txt")?;
    let result = run_part1(&str);
    assert_eq!(result, 55488);
    Ok(())
}

fn find(source: &str, items: &[String]) -> Option<String>{
    let mut index = usize::MAX;
    let mut result = "";

    for item in items.into_iter(){        
        match source.find(item){
            Some(i) => {
                if i > index{
                    continue;
                }
                index = i;
                result = item;
            }
            None => continue
        }
    }
    Some(result.to_string())
}

fn lookup(str: &str) -> String{
    match str {
        "one" => "1".into(),
        "two" => "2".into(),
        "three" => "3".into(),
        "four" => "4".into(),
        "five" => "5".into(),
        "six" => "6".into(),
        "seven"=>"7".into(),
        "eight"=>"8".into(),
        "nine"=>"9".into(),
        _ => str.to_string()
    }
}

fn run_part2(str: &str) -> i32{
    let words = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].map(String::from).to_vec();
    let rev_words= words
            .iter()
            .map(|x|x.chars().rev().collect())
            .collect::<Vec<_>>();
    let mut sum = 0;
    for line in str.lines(){
        let rev_line = line.chars().rev().collect::<String>();
        let first = find(line, &words)
            .unwrap();
        let last= find(rev_line.as_str(), &rev_words)
            .unwrap().chars().rev().collect::<String>();

        let mut num = lookup(&first);
        num.push_str(&lookup(&last));

        let res = num.parse::<i32>().unwrap();
        sum += res;
    }
    sum
}

#[test]
fn part2a(){
    assert_eq!(run_part2("two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"),281)
}

#[test]
fn part2b() -> Result<(), std::io::Error>{
    let str = read_to_string("inputs/day01.txt")?;
    let result = run_part2(&str);
    assert_eq!(result, 55614);
    Ok(())
}
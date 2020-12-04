const INPUT: &str = include_str!("../../inputs/Day3.txt");

fn main() {
    let map = get_map();

    let part1_input = (3, 1);
    let part1 = tree_count(&map, part1_input);

    println!("Part 1: {} trees passed ", part1);

    let part2_inputs = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];

    let part2 = part2_inputs.iter()
        .map(|x| tree_count(&map, *x))
        .fold(1, |acc, x| acc * x);

    println!("Part 2: {}", part2)
}

fn tree_count(map: &Vec<Vec<bool>>, (right, down): (usize, usize)) -> u32 {
    let mut pos = 0;
    let mut tree_count = 0;
    for row in map.iter().step_by(down) {
        if row[pos] { tree_count += 1; }
        pos = (pos + right) % row.len();
    }
    tree_count
}

fn get_map() -> Vec<Vec<bool>> {
    let mut map = Vec::new();
    for line in INPUT.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(char == '#');
        }
        map.push(row);
    }
    map
}
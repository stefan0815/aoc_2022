use std::fs;

pub fn solver() {
    let input = fs::read_to_string("./src/day1/input.txt")
    .expect("Should have been able to read the file");
    let elves = input.split("\r\n\r\n");
    let mut elves_carry_list : Vec<i32> = Vec::new();
    for elf in elves {
        let elf_carries_as_string_list= elf.split("\r\n");
        let mut elf_carries_as_number_list : Vec<i32> = Vec::new();
        for elf_carry in elf_carries_as_string_list {
            // println!("{elf_carry}");
            let elf_carry_trimmed = elf_carry.trim();
            elf_carries_as_number_list.push(elf_carry_trimmed.parse::<i32>().unwrap());
        }
        let sum : i32 = elf_carries_as_number_list.clone().iter().sum();
        elves_carry_list.push(sum);
    }
    elves_carry_list.sort();
    elves_carry_list.reverse();
    let max_carry_weight0 = elves_carry_list[0];
    let max_carry_weight1 = elves_carry_list[1];
    let max_carry_weight2 = elves_carry_list[2];
    println!("Day1:");
    println!("Elf with most calories:  {max_carry_weight0}");
    println!("Elf with second most calories:  {max_carry_weight1}");
    println!("Elf with third most calories:  {max_carry_weight2}");
    let sum_of_elves = elves_carry_list[0] + elves_carry_list[1] + elves_carry_list[2];
    println!("Calories of three most carrying elves: {sum_of_elves}");
}
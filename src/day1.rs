use crate::get_input;
pub fn part1() {

    let i = get_input("inputs/day1.txt");

    let mut max = 0;

    let mut temp_sum = 0;

    let mut sums = Vec::new();

    for v in i {
        if let Some(v) = v {
            temp_sum += v;
        }
        else {
            sums.push(temp_sum);
            temp_sum = 0;
        }

    }

    sums.sort();
    sums.reverse();

    println!("Top 3 sum: {}", sums.iter().take(3).sum::<u32>());

    
    println!("Top 3 sums: {} {} {}", sums[0], sums[1], sums[2]);
    dbg!(max);

}

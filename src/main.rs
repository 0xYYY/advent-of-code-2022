mod day01;
mod day02;

// TODO: remove duplications

fn main() {
    day01::solve1("input/day01.txt").unwrap();
    day01::solve2("input/day01.txt").unwrap();
    day02::solve1("input/day02.txt").unwrap();
    day02::solve2("input/day02.txt").unwrap();
}

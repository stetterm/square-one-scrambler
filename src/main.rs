
use sq1_scrambler::cube::SqOne;

fn main() {
    let mut cube = SqOne::new();
    let scramble = cube.scramble();
    println!("{}", scramble);
}

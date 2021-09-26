mod spirals_diagonal_numbers;
use spirals_diagonal_numbers::SpiralsDiagonalNumbers;

fn main() {
    println!("Calculation started");
    let sdn = SpiralsDiagonalNumbers::new(1001);

    let answer_p28: u128 = sdn.sum();

    println!(
        "The answer to problem 28 of project Euler is {}.",
        answer_p28
    );
}

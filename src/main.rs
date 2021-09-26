mod spiral_diagonal_numbers;
use spiral_diagonal_numbers::SpiralsDiagonalNumbers;

fn main() {
    let sdn = SpiralsDiagonalNumbers::new(5);
    for dn in sdn {
        println!(" {} ", dn);
    }
}

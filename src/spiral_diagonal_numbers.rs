pub struct SpiralsDiagonalNumbers {
    max_square_width_height: u16,
    current_diagonal_num: u128,
    current_square_size: u16,
    index_of_diagonal_number_in_square: u8,
}

impl SpiralsDiagonalNumbers {
    pub fn new(max_square_width_height: u16) -> Self {
        Self {
            max_square_width_height,
            current_diagonal_num: 0,
            current_square_size: 0,
            index_of_diagonal_number_in_square: 0,
        }
    }

    fn last_number_reached(&self) -> bool {
        self.current_square_size > self.max_square_width_height
    }

    fn update_diagonal_number(&mut self) {
        if self.current_diagonal_num != 0 {
            if self.index_of_diagonal_number_in_square == 4 {
                self.index_of_diagonal_number_in_square = 0;
                self.current_square_size += 2;
            }
            self.index_of_diagonal_number_in_square += 1;
            self.current_diagonal_num += self.current_square_size as u128 - 1;
        } else {
            self.current_diagonal_num = 1;
            self.current_square_size = 1;
            self.index_of_diagonal_number_in_square = 4;
        }
    }
}

impl Iterator for SpiralsDiagonalNumbers {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        self.update_diagonal_number();
        if self.last_number_reached() == false {
            Some(self.current_diagonal_num)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SpiralsDiagonalNumbers;
    #[test]
    fn sum_of_diagonals_spiral_size_5() {
        let sdn = SpiralsDiagonalNumbers::new(5);
        assert_eq!(sdn.sum::<u128>(), 101);
    }
}

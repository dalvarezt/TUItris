use rand::seq::SliceRandom;
use rand::thread_rng;

use super::tetromino::TetrominoType;

pub struct BagRandomizer {
    bag: Vec<TetrominoType>,
}

impl BagRandomizer {
    pub fn new() -> Self {
        let mut randomizer = Self {
            bag: Vec::with_capacity(7),
        };
        randomizer.refill();
        randomizer
    }

    fn refill(&mut self) {
        self.bag.extend(TetrominoType::ALL);
        self.bag.shuffle(&mut thread_rng());
    }
}

impl Default for BagRandomizer {
    fn default() -> Self {
        Self::new()
    }
}
impl Iterator for BagRandomizer {
        type Item = TetrominoType;

        fn next(&mut self) -> Option<Self::Item> {
            if self.bag.is_empty() {
                self.refill();
            }
            self.bag.pop()
        }
 
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn each_bag_contains_all_seven_pieces() {
        let pieces: Vec<_> = BagRandomizer::new().take(70).collect();
        for chunk in pieces.chunks(7) {
            let unique: HashSet<_> = chunk.iter().copied().collect();
            assert_eq!(unique.len(), 7);
        }
    }

    #[test]
    fn no_drought_longer_than_twelve_pieces() {
        let pieces: Vec<_> = BagRandomizer::new().take(700).collect();
        for kind in TetrominoType::ALL {
            let positions: Vec<usize> = pieces
                .iter()
                .enumerate()
                .filter(|&(_, &p)| p == kind)
                .map(|(i, _)| i)
                .collect();
            for pair in positions.windows(2) {
                // at most 12 pieces *between* two occurrences => index gap <= 13
                assert!(pair[1] - pair[0] <=13, "{kind:?}")
            }
        }
    }

      #[test]
      fn distribution_is_even() {
          let pieces: Vec<_> = BagRandomizer::new().take(700).collect();
          for kind in TetrominoType::ALL {
              assert_eq!(pieces.iter().filter(|&&p| p == kind).count(), 100);
          }
       }
}

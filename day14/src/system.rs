use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct DockSystem {
    mask: String,
    memory: HashMap<u64, u64>,
}

impl DockSystem {
    /// returns a new `DockSystem` with an empty bitstring mask and a 36-bit
    /// memory space initialized to 0
    pub fn new() -> DockSystem {
        DockSystem {
            mask: "".to_string(),
            memory: HashMap::new(),
        }
    }

    pub fn update_mask(&mut self, mask: &str) {
        self.mask = mask.to_string();
    }

    pub fn update_memory(&mut self, address: u64, value: u64) {
        let (and_mask, or_mask, _) = DockSystem::get_masks(&self.mask);

        let mut res = value;
        res &= and_mask;
        res |= or_mask;

        let val = self.memory.entry(address).or_insert(0);
        *val = res;
    }

    pub fn update_memory2(&mut self, address: u64, value: u64) {
        let (and_mask, or_mask, floating_mask) = DockSystem::get_masks(&self.mask);

        let addresses = DockSystem::get_addresses(address, and_mask, or_mask, floating_mask);
    }

    pub fn get_sum(&self) -> u64 {
        self.memory.values().sum()
    }

    fn get_masks(mask: &str) -> (u64, u64, u64) {
        let mut and_mask: u64 = 0;
        let mut or_mask: u64 = 0;
        let mut floating_mask: u64 = 0;

        for (i, ch) in mask.chars().rev().enumerate() {
            match ch {
                '0' => and_mask += 1 << i,
                '1' => or_mask += 1 << i,
                'X' => floating_mask += 1 << i,
                _ => {}
            }
        }

        (!and_mask, or_mask, floating_mask)
    }

    fn get_addresses(address: u64, and_mask: u64, or_mask: u64, floating_mask: u64) -> Vec<u64> {
        let mut res = Vec::new();

        // apply the 1's and 0's mask
        let mut modified_address = address;
        modified_address &= and_mask;
        modified_address |= or_mask;

        // apply the floating mask to get all the permutations of the address

        println!("{:?}", res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_masks() {
        let tests = vec![
            ("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", (64, !2)),
            ("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX111", (7, !0)),
            ("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX0XXX000", (0, !71)),
            ("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX0111100", (60, !67)),
        ];

        for (inp, (or_mask, and_mask)) in tests {
            let res = DockSystem::get_masks(inp);

            assert_eq!(res.0, and_mask);
            assert_eq!(res.1, or_mask);
        }
    }

    #[test]
    fn test_get_addresses() {
        let tests: Vec<(u64, u64, u64, u64)> = vec![
            (42, 0b1100, 0b10010, 0b100001),
            (26, 0b1110100, 0b000000, 0b1011),
        ];

        let answers: Vec<HashSet<u64>> = vec![
            HashSet::from_iter(vec![26, 27, 58, 59]),
            HashSet::from_iter(vec![16, 17, 18, 19, 24, 25, 26, 27]),
        ];

        for (i, (address, and_mask, or_mask, float_mask)) in tests.iter().enumerate() {
            let answer = &answers[i];
            let res = DockSystem::get_addresses(*address, *and_mask, *or_mask, *float_mask);
            assert_eq!(res.len(), answer.len());

            for num in answer {
                assert_eq!(answer.contains(num), true);
            }
        }
    }
}

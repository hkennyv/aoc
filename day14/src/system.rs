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
        let addresses = DockSystem::get_addresses(address, &self.mask);

        for address in addresses {
            self.memory.insert(address, value);
        }
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

    fn get_addresses(address: u64, mask: &str) -> Vec<u64> {
        let address_string: String = format!("{:036b}", address)
            .chars()
            .zip(mask.chars())
            .map(|(ac, mc)| match mc {
                '0' => ac,
                '1' => '1',
                'X' => 'X',
                _ => mc,
            })
            .collect();

        let combinations = DockSystem::combinations(&address_string);

        combinations
            .iter()
            .map(|s| u64::from_str_radix(s, 2).unwrap())
            .collect()
    }

    fn combinations(address: &str) -> Vec<String> {
        if address.contains('X') {
            vec![
                DockSystem::combinations(&address.replacen("X", "0", 1)),
                DockSystem::combinations(&address.replacen("X", "1", 1)),
            ]
            .into_iter()
            .flatten()
            .collect()
        } else {
            vec![address.to_owned()]
        }
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
}

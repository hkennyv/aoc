use std::collections::HashMap;

pub struct DockSystem {
    mask: String,
    memory: HashMap<u64, u64>
}

impl DockSystem {
    /// returns a new `DockSystem` with an empty bitstring mask and a 36-bit
    /// memory space initialized to 0
    pub fn new() -> DockSystem {
        DockSystem {
            mask: "".to_string(),
            memory: HashMap::new()
        }
    }

    pub fn update_mask(&mut self, mask: &str) {
        self.mask = mask.to_string();
    }

    pub fn update_memory(&mut self, address: u64, value: u64) {
        let (and_mask, or_mask) = DockSystem::get_masks(&self.mask);

        let mut res = value;
        res &= and_mask;
        res |= or_mask;

        let val = self.memory.entry(address).or_insert(0);
        *val = res;
    }

    pub fn get_sum(&self) -> u64 {
        self.memory.values().sum()
    }

    fn get_masks(mask: &str) -> (u64, u64) {
        let mut and_mask: u64 = 0;
        let mut or_mask: u64 = 0;

        for (i, ch) in mask.chars().rev().enumerate() {
            match ch {
                '0' => and_mask += 1 << i,
                '1' => or_mask += 1 << i,
                _ => {}
            }
        }

        (!and_mask, or_mask)
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

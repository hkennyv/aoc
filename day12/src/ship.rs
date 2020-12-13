/// the `Ship` model. the orientation is in +degrees and the location is
/// based on a cartesian coordinate system where E is (1, 0) and N is (0, 1)
pub struct Ship {
    orientation: i32,
    location: (i32, i32),
    waypoint: (i32, i32),
}

impl Ship {
    /// returns a new instance of `Ship` initialized with orientation 0 and
    /// location (0, 0)
    pub fn new() -> Ship {
        Ship {
            orientation: 0,
            location: (0, 0),
            waypoint: (10, 1),
        }
    }

    /// updates the ship's orientation and location based on a single action
    /// valid actions are: N, S, E, W, L, R, F (based on p1)
    pub fn handle_action_p1(&mut self, action: char, value: i32) {
        match action {
            'N' => {
                self.location.1 += value;
            }
            'S' => {
                self.location.1 -= value;
            }
            'E' => {
                self.location.0 += value;
            }
            'W' => {
                self.location.0 -= value;
            }
            'L' => {
                self.orientation = (self.orientation + value) % 360;
                if self.orientation < 0 {
                    self.orientation += 360;
                }
            }
            'R' => {
                self.orientation = (self.orientation - value) % 360;
                if self.orientation < 0 {
                    self.orientation += 360;
                }
            }
            'F' => {
                let x = value * (self.orientation as f32).to_radians().cos() as i32;
                let y = value * (self.orientation as f32).to_radians().sin() as i32;

                self.location.0 += x;
                self.location.1 += y;
            }
            _ => {}
        }
    }

    /// updates the ship's location and the waypoints location based on a
    /// single action valid actions are: N, S, E, W, L, R, F (based on p2)
    pub fn handle_action_p2(&mut self, action: char, value: i32) {
        match action {
            'N' => {
                self.waypoint.1 += value;
            }
            'S' => {
                self.waypoint.1 -= value;
            }
            'E' => {
                self.waypoint.0 += value;
            }
            'W' => {
                self.waypoint.0 -= value;
            }
            'L' => {
                let mag = ((self.waypoint.0.pow(2) + self.waypoint.1.pow(2)) as f32).sqrt();
                let angle = (self.waypoint.1 as f32).atan2(self.waypoint.0 as f32)
                    + (value as f32).to_radians();

                let x = (mag * angle.cos()).round();
                let y = (mag * angle.sin()).round();

                self.waypoint.0 = x as i32;
                self.waypoint.1 = y as i32;
            }
            'R' => {
                let mag = ((self.waypoint.0.pow(2) + self.waypoint.1.pow(2)) as f32).sqrt();
                let angle = (self.waypoint.1 as f32).atan2(self.waypoint.0 as f32)
                    - (value as f32).to_radians();

                let x = (mag * angle.cos()).round();
                let y = (mag * angle.sin()).round();

                self.waypoint.0 = x as i32;
                self.waypoint.1 = y as i32;
            }
            'F' => {
                self.location.0 += value * self.waypoint.0;
                self.location.1 += value * self.waypoint.1;
            }
            _ => {}
        }
    }

    /// returns the manhattan distance - the absolute values of its east/west
    /// position and its north/south position
    pub fn get_manhattan_distance(&self) -> i32 {
        self.location.0.abs() + self.location.1.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_action_p1() {
        let mut ship = Ship::new();

        let actions: Vec<(char, i32)> = vec![
            ('N', 2),
            ('S', 1),
            ('E', 1),
            ('W', 3),
            ('L', 90),
            ('R', 180),
            ('F', 3),
        ];

        let answers: Vec<(i32, (i32, i32))> = vec![
            (0, (0, 2)),
            (0, (0, 1)),
            (0, (1, 1)),
            (0, (-2, 1)),
            (90, (-2, 1)),
            (270, (-2, 1)),
            (270, (-2, -2)),
        ];

        // should be initialized to (0, 0)
        assert_eq!(ship.location, (0, 0));

        for (i, (action, value)) in actions.iter().enumerate() {
            let answer = answers[i];

            ship.handle_action_p1(*action, *value);
            assert_eq!(ship.orientation, answer.0);
            assert_eq!(ship.location, answer.1);
        }
    }

    #[test]
    fn test_handle_action_p2() {
        let mut ship = Ship::new();

        let actions: Vec<(char, i32)> = vec![
            ('N', 2),
            ('S', 1),
            ('E', 1),
            ('W', 3),
            ('L', 90),
            ('R', 180),
            ('F', 3),
        ];

        let answers: Vec<((i32, i32), (i32, i32))> = vec![
            ((0, 0), (10, 3)),
            ((0, 0), (10, 2)),
            ((0, 0), (11, 2)),
            ((0, 0), (8, 2)),
            ((0, 0), (-2, 8)),
            ((0, 0), (2, -8)),
            ((6, -24), (2, -8)),
        ];

        // ship should be initialized to (0, 0)
        // and waypoint should be (10, 1)
        assert_eq!(ship.location, (0, 0));
        assert_eq!(ship.waypoint, (10, 1));

        for (i, (action, value)) in actions.iter().enumerate() {
            let answer = answers[i];

            ship.handle_action_p2(*action, *value);
            assert_eq!(ship.location, answer.0);
            assert_eq!(ship.waypoint, answer.1);
        }
    }

    #[test]
    fn test_get_manhattan_distance() {
        let mut ship = Ship::new();

        assert_eq!(ship.get_manhattan_distance(), 0);

        ship.location = (17, 8);
        assert_eq!(ship.get_manhattan_distance(), 25);
    }
}

use super::*;
use std::f32::consts::*;

#[derive(Debug)]
pub struct Eye {
    pub fov_range: f32,
    pub fov_angle: f32,
    pub cells: usize,
}

impl Eye {
    // Range of field of view.
    const FOV_RANGE: f32 = 0.25;

    // Angle of field of view.
    const FOV_ANGLE: f32 = PI + FRAC_PI_4;

    // Photoreceptors in a single eye.
    const CELLS: usize = 9;

    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0. && fov_angle > 0. && cells > 0);

        Self { fov_range, fov_angle, cells }
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            let vec = food.position - position;
            let distance = vec.norm();

            // Skips foods outside the fov_range.
            if distance >= self.fov_range {
                continue;
            }

            let angle =
                na::Rotation2::rotation_between(&na::Vector2::x(), &vec)
                    .angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);

            // Skips foods outside the fov_angle
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            // [ -fov_angle/2, fov_angle/2 ] to [ 0, fov_angle ]
            let angle = angle + self.fov_angle / 2.0;
            // [ 0, fov_angle ] to [ 0, 1 ]
            let cell = angle / self.fov_angle;
            // get index of the cell
            let cell = cell * (self.cells as f32);
            // `min` here is to avoid index into `cells.len()`
            let cell = (cell as usize).min(cells.len() - 1);

            // Energy is between [0., 1.] and the higher the energy is, the closer the food will be.
            let energy = (self.fov_range - distance) / self.fov_range;

            cells[cell] += energy;
        }

        cells
    }

    pub fn cells(&self) -> usize {
        self.cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(Self::FOV_RANGE, Self::FOV_ANGLE, Self::CELLS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_TEST_EYE_CELLS: usize = 13;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(
                self.fov_range,
                self.fov_angle,
                DEFAULT_TEST_EYE_CELLS,
            );

            let actual_vision = eye.process_vision(
                na::Point2::new(self.x, self.y),
                na::Rotation2::new(self.rot),
                &self.foods,
            );
            let actual_vision: Vec<_> = actual_vision
                .into_iter()
                .map(|cell| {
                    if cell >= 0.7 {
                        // food is fairly close
                        "#"
                    } else if cell >= 0.3 {
                        // food is somewhat further
                        "+"
                    } else if cell > 0.0 {
                        // food is pretty far away
                        "."
                    } else {
                        // no food in sight
                        " "
                    }
                })
                .collect();
            // join them with empty string.
            let actual_vision = actual_vision.join("");

            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    // helper function to create food easily.
    fn food(x: f32, y: f32) -> Food {
        Food { position: na::Point2::new(x, y) }
    }

    mod different_fov_ranges {
        use super::*;
        use test_case::test_case;

        #[test_case(1.0, "      +      ")] // Food is inside the FOV
        #[test_case(0.9, "      +      ")] // ditto
        #[test_case(0.8, "      +      ")] // ditto
        #[test_case(0.7, "      .      ")] // Food slowly disappears
        #[test_case(0.6, "      .      ")] // ditto
        #[test_case(0.5, "             ")] // Food disappeared!
        #[test_case(0.4, "             ")]
        #[test_case(0.3, "             ")]
        #[test_case(0.2, "             ")]
        #[test_case(0.1, "             ")]
        fn test(fov_range: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.5)],
                fov_angle: FRAC_PI_2,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_range,
                expected_vision,
            }
            .run()
        }
    }

    mod different_rotations {
        use super::*;
        use test_case::test_case;

        #[test_case(0.00 * PI, "         +   ")] // Food is to our right
        #[test_case(0.25 * PI, "        +    ")]
        #[test_case(0.50 * PI, "      +      ")]
        #[test_case(0.75 * PI, "    +        ")]
        #[test_case(1.00 * PI, "   +         ")] // Food is behind us
        #[test_case(1.25 * PI, " +           ")] // (we continue to see it
        #[test_case(1.50 * PI, "            +")] // due to 360° fov_angle.)
        #[test_case(1.75 * PI, "           + ")]
        #[test_case(2.00 * PI, "         +   ")] // Here we've done 360°
        #[test_case(2.25 * PI, "        +    ")] // (and a bit more, to
        #[test_case(2.50 * PI, "      +      ")] // prove the numbers wrap.)
        fn test(rot: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(0.5, 1.0)],
                fov_range: 1.0,
                fov_angle: 2.0 * PI,
                x: 0.5,
                y: 0.5,
                rot,
                expected_vision,
            }
            .run()
        }
    }
}

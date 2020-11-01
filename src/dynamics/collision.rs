use crate::utils::intersection::circle_intersection;

use crate::{
    dynamics::{acceleration::Acceleration, velocity::Velocity},
    shapes::circle::Circle,
    utils::intersection::Intersection,
};

#[derive(Debug, PartialEq)]
pub struct Collision<B> {
    body1: B,
    body2: B,
}

impl<B> Collision<B> {
    pub fn new(body1: B, body2: B) -> Collision<B> {
        Collision { body1, body2 }
    }
}

impl Collision<Circle> {
    pub fn get_contacts(&self) -> Intersection {
        circle_intersection(self.body1, self.body2)
    }

    pub fn get_velocities(&self) -> (Velocity, Velocity) {
        (self.body1.get_velocity(), self.body2.get_velocity())
    }

    pub fn get_accelerations(&self) -> (Acceleration, Acceleration) {
        (self.body1.get_acceleration(), self.body2.get_acceleration())
    }
}

#[cfg(test)]
mod tests {
    use super::{Acceleration, Circle, Collision, Velocity};
    use crate::utils::{intersection::IntersectionResult, point::Point};
    #[test]
    fn it_finds_circle_poc() {
        let v1 = Velocity::new(5.0, 5.0, 0.0);
        let v2 = Velocity::new(-5.0, 5.0, 0.0);

        let a1 = Acceleration::new(0.0, 0.0, 0.0);
        let a2 = Acceleration::new(0.0, 0.0, 0.0);

        let c1 = Circle::new(10.0, 10.0, 2.5, None, v1, a1);
        let c2 = Circle::new(5.0, 5.0, 2.5, None, v2, a2);

        let collision: Collision<Circle> = Collision::new(c1, c2);
        let contacts = collision.get_contacts();
        let is_intersection = contacts.result_type == IntersectionResult::Intersection;
        let result = contacts.result;
        if is_intersection {
            assert_eq!(
                result.unwrap(),
                (Point::new(7.5, 7.5), Point::new(7.5, 7.5))
            )
        }
    }

    #[test]
    fn it_circle_velocities() {
        let v1 = Velocity::new(5.0, 5.0, 0.0);
        let v2 = Velocity::new(-5.0, 5.0, 0.0);

        let a1 = Acceleration::new(0.0, 0.0, 0.0);
        let a2 = Acceleration::new(0.0, 0.0, 0.0);

        let c1 = Circle::new(10.0, 10.0, 2.5, None, v1, a1);
        let c2 = Circle::new(5.0, 5.0, 2.5, None, v2, a2);

        let collision: Collision<Circle> = Collision::new(c1, c2);

        assert_eq!(collision.get_velocities(), (v1, v2));
    }

    #[test]
    fn it_circle_accelerations() {
        let v1 = Velocity::new(5.0, 5.0, 0.0);
        let v2 = Velocity::new(-5.0, 5.0, 0.0);

        let a1 = Acceleration::new(1.0, 1.0, 1.0);
        let a2 = Acceleration::new(1.0, 1.0, 1.0);

        let c1 = Circle::new(10.0, 10.0, 2.5, None, v1, a1);
        let c2 = Circle::new(5.0, 5.0, 2.5, None, v2, a2);

        let collision: Collision<Circle> = Collision::new(c1, c2);

        assert_eq!(collision.get_accelerations(), (a1, a2));
    }
}

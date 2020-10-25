use crate::{
    dynamics::{acceleration::Acceleration, velocity::Velocity},
    shapes::circle::Circle,
    utils::point::Point,
};

#[derive(Debug, PartialEq)]
pub struct Collision<B> {
    body1: B,
    body2: B,
}

impl<T> Collision<T> {
    pub fn new(body1: T, body2: T) -> Collision<T> {
        Collision { body1, body2 }
    }
}

impl Collision<Circle> {
    // pub fn get_contacts(&self) -> Point {
    //     let (x1, y1) = self.body1.get_position();
    //     let (x2, y2) = self.body2.get_position();
    //     let r1 = self.body1.get_radius();
    //     // Rigid circles can only have one contact point
    //     Point::new((x2 - x1) - r1, y2 - y1 - r1)
    // }

    pub fn get_velocities(&self) -> (Velocity, Velocity) {
        (self.body1.get_velocity(), self.body2.get_velocity())
    }

    pub fn get_accelerations(&self) -> (Acceleration, Acceleration) {
        (self.body1.get_acceleration(), self.body2.get_acceleration())
    }
}

#[cfg(test)]
mod tests {
    use super::{Acceleration, Circle, Collision, Point, Velocity};
    // #[test]
    // fn it_finds_circle_poc() {
    //     let v1 = Velocity::new(5.0, 5.0, 0.0);
    //     let v2 = Velocity::new(-5.0, 5.0, 0.0);

    //     let a1 = Acceleration::new(0.0, 0.0, 0.0);
    //     let a2 = Acceleration::new(0.0, 0.0, 0.0);

    //     let c1 = Circle::new(10.0, 10.0, 2.5, None, v1, a1);
    //     let c2 = Circle::new(5.0, 5.0, 2.5, None, v2, a2);

    //     let collision: Collision<Circle> = Collision::new(c1, c2);

    //     assert_eq!(collision.get_contacts(), Point::new(7.5, 7.5))
    // }

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

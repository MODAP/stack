use nalgebra as na;
use std::time::{SystemTime, SystemTimeError};

// A quick word here about locale:
// X is X, Y is Y, Z is Z. w.r.t. MPU6050

/// Represents the locale of the current user
#[derive(Clone)]
pub struct Locale {
    // Acceleration: directly set
    pub accel: na::SVector<f32,3>,
    // Velocity: 1st int. of accel
    pub velocity: na::SVector<f32,3>,
    // Position: 2nd int. of accer
    pub position:  na::SVector<f32,3>,
    // Timestamp of the previous update
    timestamp: SystemTime
}

// Implements the locale struct with facilities
// for providing integration. This is **bad**
// and needs, eventually, error correction facilities
impl Locale {

    /// Updates the acceleration value and runs an iteration
    /// TODO: in the future, also run Kalman state update
    ///
    /// # Arguments
    /// - `accel:(f32,f32,f32)`: the new acceleration
    ///
    /// # Returns
    /// `Result<Self, SystemTimeError>`: a Clone of the object with values updated
    ///
    /// # Examples
    ///
    /// ```
    /// let mut loc = brain::Locale::new((0.0,0.0,0.0));
    /// loc = loc.update((0.0, 0.3, 0.4)).unwrap(); // DANGEROUS we unwrap b/c it returns a Result
    ///                                             // but it could error with SystemTimeError
    ///                                             // when Rust cannot access the hardware clock
    ///                                             // and that case should be handled with `match`
    /// println!("{}", loc.position); 
    /// ```
    pub fn update(&mut self, accel:(f32,f32,f32)) -> Result<Self, SystemTimeError>{
        let elapsed:f32 = self.timestamp.elapsed()?.as_secs_f32();

        self.accel = na::Vector3::<f32>::new(accel.0, accel.1, accel.2);
        self.velocity += self.accel.scale(elapsed);
        self.position += self.velocity.scale(elapsed);

        self.timestamp = SystemTime::now();

        Ok(self.clone())
    }

    /// Initialize the Locale with an initial position
    ///
    /// # Examples
    ///
    /// ```
    /// let loc = brain::Locale::new((0.0,0.0,0.0));
    /// println!("{}", loc.accel);
    /// ```
    pub fn new(initial_pos:(f32,f32,f32)) -> Self {
        return Locale {
            accel: na::Vector3::<f32>::zeros(),
            velocity: na::Vector3::<f32>::zeros(),
            position: na::Vector3::<f32>::new(initial_pos.0, initial_pos.1, initial_pos.2),
            timestamp: SystemTime::now()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale() {
        let mut loc = Locale::new((0.0,40.0,0.0));

        loc = loc.update((0.0, -209.8, 0.0)).unwrap();

        loc = loc.update((0.0, 100.8, 0.0)).unwrap();

        loc = loc.update((0.0, -9.8, 0.0)).unwrap();

        let _loss = loc.position.magnitude();
    }
}

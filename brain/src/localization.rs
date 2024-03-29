use nalgebra as na;
use std::time::{SystemTime, SystemTimeError};

// A quick word here about locale:
// X is X, Y is Y, Z is Z. w.r.t. MPU6050
// Angle is angle in radians

/// Represents the locale of the current user
/// This should be relative to ground (i.e. gyro)

#[derive(Clone)]
pub struct Locale {
    // Acceleration: directly set
    pub accel: na::SVector<f32, 3>,
    // Velocity: 1st int. of accel
    pub velocity: na::SVector<f32, 3>,
    // Position: 2nd int. of accer
    pub position: na::SVector<f32, 3>,
    // Angle: the first intergral of gyro
    pub angle: na::SVector<f32, 3>,
    // Timestamp of the previous update
    timestamp: SystemTime,
    // Aprogee detection armed
    armed: bool,
    // Acceleration Cache
    accel_cache: Vec<na::SVector<f32, 3>>,
    // Gyro Angle Cache
    gyro_cache: Vec<na::SVector<f32, 3>>,
    // Fidelity (ms)
    fidelity: u16,
}

// Implements the locale struct with facilities
// for providing integration. This is **bad**
// and needs, eventually, error correction facilities
impl Locale {
    /// Updates the acceleration value and runs an iteration
    /// TODO: in the future, also run Kalman state update
    ///
    /// # Arguments
    /// - `accel:(f32,f32,f32)`: the new acceleration (m/s^2)
    /// - `gyro:(f32,f32,f32)`: the new gyroscope value (radians/s)
    ///
    /// # Returns
    /// `Result<bool, SystemTimeError>`: whether or not we have reached apogee
    ///
    /// # Examples
    ///
    /// ```
    /// let mut loc = brain::Locale::new((0.0,0.0,0.0), 500);
    /// loc.update((0.0, 0.3, 0.4),
    ///            (0.0, 0.3, 0.5)).unwrap(); // DANGEROUS we unwrap b/c it returns a Result
    ///                                       // but it could error with SystemTimeError
    ///                                       // when Rust cannot access the hardware clock
    ///                                       // and that case should be handled with `match`
    /// println!("{}", loc.position);
    /// ```
    pub fn update(
        &mut self,
        accel: (f32, f32, f32),
        gyro: (f32, f32, f32),
    ) -> Result<bool, SystemTimeError> {
        // Push the current stamp into cache
        self.accel_cache
            .push(na::Vector3::<f32>::new(accel.0, accel.1, accel.2));
        self.gyro_cache
            .push(na::Vector3::<f32>::new(gyro.0, gyro.1, gyro.2));

        // Calculate local acceleration
        let accel = self
            .accel_cache
            .iter()
            .sum::<na::SVector<f32, 3>>()
            .scale(1.0 / (self.accel_cache.len() as f32));

        // Calculate the mean gyro angles (rad)
        self.angle = self
            .gyro_cache
            .iter()
            .sum::<na::SVector<f32, 3>>()
            .scale(1.0 / (self.gyro_cache.len() as f32));

        // And therefore, produce a unit vector in the direction in those angles
        let accel_normalized = accel
            .iter()
            .zip(self.angle.iter())
            .map(|(a, g)| g.cos() * a)
            .collect::<Vec<f32>>();

        // Setting the acceleration
        self.accel = na::Vector3::<f32>::new(
            accel_normalized[0],
            accel_normalized[1],
            accel_normalized[2],
        );

        // Calculate elapsed time since last stamp
        let elapsed: u16 = (self.timestamp.elapsed()?.as_millis()) as u16;

        // If the temporal fidelity is reached, we integrate
        if elapsed > self.fidelity {
            // Reset the caches and timestamps
            self.timestamp = SystemTime::now();
            self.accel_cache = vec![];
            self.gyro_cache = vec![];

            // Push the velocity and position values
            self.velocity += self.accel.scale(elapsed as f32 / 1000.0);
            self.position += self.velocity.scale(elapsed as f32 / 1000.0);
        }

        // Arm the apogee detection of velocity becomes > 5m/s
        if self.velocity.magnitude() > 5.0 {
            self.armed = true;
        }

        // Check if velocity is smaller than 0.1, then in which case
        // increment
        return Ok(if self.armed && self.velocity.magnitude() < 0.1 {
            true
        } else {
            false
        });
    }

    /// Initialize the Locale with an initial position and a fidelity
    ///
    /// # Aurgements
    /// - `initial_pos:(f32,f32,f32)`: the initial position of the object
    /// - `fidelity:u16`: the fidelity in ms to update, larger the # larger the cache
    ///
    /// # Examples
    ///
    /// ```
    /// let loc = brain::Locale::new((0.0,0.0,0.0), 500);
    /// println!("{}", loc.accel);
    /// ```
    pub fn new(initial_pos: (f32, f32, f32), fidelity: u16) -> Self {
        return Locale {
            accel: na::Vector3::<f32>::zeros(),
            velocity: na::Vector3::<f32>::zeros(),
            position: na::Vector3::<f32>::new(initial_pos.0, initial_pos.1, initial_pos.2),
            angle: na::Vector3::<f32>::zeros(),
            timestamp: SystemTime::now(),
            armed: false,
            accel_cache: vec![],
            gyro_cache: vec![],
            fidelity,
        };
    }
}

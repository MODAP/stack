# MODAP Stack
Welcome to the MODAP stack.

# Engineering Guide
## Common Interfaces
- Images =ndarray::Array3=: (1600 * 1200 * 3) (width * height * rgb)
- Acceleration =(f32,f32,f32)=: range ±36*9.81 m/s^2
- Rotation =(f32,f32,f32)=: degrees, x, y, z

## Data Rate
Camera: 15fps, Accel: 1kHz, Gyro: 8kHz. "Brain" code has buffer. "Hand" code calls dumping facilities in Brain code to dump data to cache buffer.

## Mainloop Design
Boom! Ascend. We are in the air.
1. During ascend, Hand code runs mainloop that samples data continuously from handware. Each sample gets passed to "Brain" code, which runs with the detection on buffered data. On each point returns whether or not — given this new sample —  apogee is achieved

"Aproogee function" in Brain is actually secretly a kalman filter that also returns position, velocity, and errors in those. So Akshar Time. We need to give all of these back to Hand so that it could figure out where it is and correct for errors to get back to the ground in the sameish place.

2. Once apogee is reached, Hand code snaps a photo and call the processing code in Brain. Simultaneously, Hand code sends the processed image.
3. Then, Hand code work on guiding it back down.

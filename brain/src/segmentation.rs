use nalgebra as na;
use std::collections::VecDeque;

const THRESHOLD: f32 = 10.0;

// Returns a color-based floodfill mask with the same dimensions as the input.
//
// # Arguments
// * `image` - A 1920x1080 matrix of Vector3<f32>, where each vector is RGB values from 0-255.
// # Example
// ```
// use nalgebra as na;
// let image = na::Smatrix::repeat(na::Vector3::new_random()) // Just a demo - not actually RGB
// let mask = floodfill(image)
// ```
fn floodfill(image: na::SMatrix<na::Vector3<f32>, 1920, 1080>, pos: na::Vector2<usize>) {
    let mut q = VecDeque::new();
    q.push_front(image[(pos.x, pos.y)]);
    while !q.is_empty() {
        let mut n = q.pop_front().unwrap();
        let east = image[(pos.x + 1, pos.y)];
        if (n - east).norm() < THRESHOLD {
            q.push_front(east);
        }
    }
}

use nalgebra as na;
use std::collections::VecDeque;

const THRESHOLD: u32 = 10;

fn floodfill(image: na::SMatrix<na::Vector3<u32>, 1920, 1080>, pos: na::Vector2<usize>) {
    let mut q = VecDeque::new();
    q.push_front(image[(pos.x, pos.y)]);
    while !q.is_empty() {
	let mut n = q.pop_front().unwrap();
	let east = image[(pos.x+1, pos.y)];
	if (n - east).norm() < THRESHOLD {
	    q.push_front(east);
	}
    }
}

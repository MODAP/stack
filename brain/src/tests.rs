// import pylon API to seed pylon for camea
use pylon_cxx;
use anyhow::Result;

// import everything from our end
use super::*;

//// Unit Tests ////

#[test]
/// A test for locale creation
fn locale_create() {
	let mut _loc = Locale::new((0.0,4000.0,0.0), 500);
}

#[test]
/// A test for locale updating
fn locale_update() {
	let mut loc = Locale::new((0.0,4000.0,0.0), 500);

	// DANGEROUS we unwrap b/c it returns a Result
	// but it could error with SystemTimeError
	// when Rust cannot access the hardware clock
	// and that case should be handled with `match`
	// instead of just wrapping

	loc.update((1.0, -9.8, 0.0),
			   (0.0, 0.3, 0.5)).unwrap();
}

#[test]
fn camera_initalization() {
	// create new instance
	let pylon = pylon_cxx::Pylon::new();
	// And then create camera
	let cam = Camera::new(&pylon);
	// cam.start();
	cam.debug();
}

// #[test]
// fn camera_grab() -> anyhow::Result<()> {
// 	let pylon = pylon_cxx::Pylon::new();
// 	let mut camera = Camera::new(&pylon);
// 	camera.start_limited(5).unwrap();
// 	for i in 0..5 {
// 		let frame = camera.grab_frame().unwrap();
// 		image::save_buffer(
// 			format!("./test{}.png", i),
// 			&frame,
// 			1024,
// 			1040,
// 			image::ColorType::L8
// 		).unwrap();
// 	}
// 	Ok(())
// }

#[tokio::test]
async fn camera_stream() -> anyhow::Result<()> {
	use tokio_stream::StreamExt;
	let pylon = pylon_cxx::Pylon::new();
	let mut camera = Camera::new(&pylon);
	camera.start_limited(5).unwrap();
	let inner = camera.camera;
	// let frame = inner.take(5).collect::<Vec<pylon_cxx::GrabResult>>().await;
	// for i in frame.iter() {
	// 	image::save_buffer(
	// 		"./test.png",
	// 		i.buffer().unwrap(),
	// 		i.width().unwrap(),
	// 		i.height().unwrap(),
	// 		image::ColorType::L8
	// 	).unwrap();
	// }
	// for i in 0..5 {
	// 	let res = inner.poll_next()?;
	// }
	tokio::pin!(inner);
	let mut n = 0;
	while let Some(grab_result) = inner.next().await {
		n += 1;
		image::save_buffer(
			format!("./test{}.png", n),
			grab_result.buffer().unwrap(),
			grab_result.width().unwrap(),
			grab_result.height().unwrap(),
			image::ColorType::L8
		).unwrap();
		std::mem::forget(grab_result);
		println!("{n}");
	} 
	panic!("AAA")
}

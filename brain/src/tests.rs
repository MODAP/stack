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
//	let pylon = pylon_cxx::Pylon::new();
//	let mut camera = Camera::new(&pylon);
//	camera.start_limited(5).unwrap();
//	for i in 0..5 {
//		let frame = camera.grab_frame().unwrap();
//		image::save_buffer(
//			format!("./test{}.png", i),
//			&frame,
//			1024,
//			1040,
//			image::ColorType::L8
//		).unwrap();
//	}
//	Ok(())
// }

#[tokio::test]
async fn camera_stream() -> anyhow::Result<()> {
	use tokio_stream::StreamExt;
	use tokio::io::AsyncWriteExt;
	use image::ImageEncoder;
	use image::codecs::png::PngEncoder;

	let pylon = pylon_cxx::Pylon::new();
	let mut camera = Camera::new(&pylon);
	camera.start().unwrap();
	let inner = camera.camera;

	tokio::pin!(inner);
	let mut n = 0;	
	while let Some(grab_result) = inner.next().await {
		let start = std::time::Instant::now();
		n += 1;
		let buf = grab_result.buffer().unwrap().iter().copied().collect::<Vec<u8>>();
		let width = grab_result.width().unwrap();
		let height = grab_result.height().unwrap();
		tokio::spawn(async move {
			let mut file = tokio::fs::File::create("foo.png").await.expect("Reason");
			let mut out = Vec::new();
			let encoder = PngEncoder::new(&mut out);			
			encoder.write_image(
				&buf,
				width,
				height,
				image::ColorType::L8,
			).unwrap();
			file.write_all(&out).await
		});
		println!("{n}");
		dbg!(start.elapsed());
	}
	panic!("AAA")
}

use anyhow::Result;
use opencv::{
    prelude::*,
    videoio,
    highgui
}; // Note, the namespace of OpenCV is changed and no longer one enormous container.

pub struct CameraReader {}

fn readFrame() -> Result<(Mat)> { // Note, this is anyhow::Result
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default(); // This array will store the web-cam data
    // Read the camera
    // and display in the window
    cam.read(&mut frame)?;
    Ok(frame)
}

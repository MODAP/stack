// Import pylon
use pylon_cxx::*;

use anyhow::Result;

// Streaming utilites
#[allow(unused_imports)]
use tokio_stream::StreamExt;

/// Represents an camera instance
/// This would abstract away Pylon APIs
pub struct Camera<'a> {
    // and the camera
    pub camera: pylon_cxx::InstantCamera<'a>
}

// We will provide initalization and
// grab facilities
impl<'a> Camera<'a> {

    /// Initialize camera, try getting a camera, and seeding it
    pub fn new(pylon:&'a pylon_cxx::Pylon) -> Self {
        // and create new camera object
        let cam = pylon_cxx::TlFactory::instance(pylon).create_first_device();

        // Return the camera
        Camera {
            camera: match cam {
                Ok(c) => c,
                Err(_) => panic!("We need a camera.")
            }
        }
    }

    //// SYNCRONOUS OPTS ////
    // fn camera_info() -> (design and return serialized struct)


    fn debug(self) -> Result<()> {
	let ver = pylon_version();
	let info = self.camera.device_info();
	println!("Pylon version {}.{}.{}, build {}.", ver.major, ver.minor, ver.subminor, ver.build);
	println!("{}", info.model_name()?);
	Ok(())
    }

    //// ASYNC OPTS ////
    // fn camera_stream() -> Result<tokio_stream::Stream<ndarray::Array3>> # streaming buffer
    // Example: https://github.com/strawlab/pylon-cxx/blob/main/examples/async-grab.rs

    // REMINDER TO WRITE TESTS IN tests.rs
}


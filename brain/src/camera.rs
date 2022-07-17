// Import pylon
use pylon_cxx::*;

use anyhow::Result;

// Streaming utilites
#[allow(unused_imports)]
use tokio_stream::StreamExt;
use anyhow::{Result, anyhow};

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

    //// SYNCRONOUS OPTS ///

    fn grab_frame(self) -> Result<Vec<u8>> {
        let mut grab_result = pylon_cxx::GrabResult::new()?;
        // Wait for an image and then retrieve it. A timeout of 0 ms is used.
        self.camera.retrieve_result(
            0,
            &mut grab_result,
            pylon_cxx::TimeoutHandling::ThrowException,
        )?;

        // Checking if the image is grabbed succesfully
        if grab_result.grab_succeeded()? {
            // Access the image data.
            if cfg!(debug_assertions) {
                println!("Frame Size_X: {}", grab_result.width()?);
                println!("Frame Size_Y: {}", grab_result.height()?);
            }

            let image_buffer = grab_result.buffer()?;
            return Ok(image_buffer.to_vec())
        } else {
            return Err(anyhow!("YOOOOO Code: {} {}", grab_result.error_code()?, grab_result.error_description()?))
        }
    }
    
    fn grab_frame_buffer(self, num_frames: usize) -> Result<Vec<Vec<u8>>> {
        let frame_buffer: Vec<Vec<u8>>;

        for i in 0..num_frames {
            frame_buffer.push(self.grab_frame()?);
        }
        return Ok(frame_buffer)
    }
    
    //// ASYNCRONOUS OPTS ///

    /// Dumps the Pylon build version and camera model name to terminal
    ///
    /// # Returns
    /// `Result<()>`: Pylon can error when trying to fetch the model name.
    ///
    /// # Examples
    ///
    /// ```
    /// let pylon = pylon_cxx::Pylon::new();
    /// let cam = brain::Camera::new(&pylon);
    /// cam.debug();
    /// ```
    pub fn debug(self) -> Result<()> {
        let ver = pylon_version();
        let info = self.camera.device_info();
        println!("Pylon version {}.{}.{}, build {}.", ver.major, ver.minor, ver.subminor, ver.build);
        println!("Camera model name: {}", info.model_name()?);
        Ok(())
    }

    //// ASYNC OPTS ////
    // fn camera_stream() -> Result<tokio_stream::Stream<ndarray::Array3>> {
//    }
    // Example: https://github.com/strawlab/pylon-cxx/blob/main/examples/async-grab.rs

    // REMINDER TO WRITE TESTS IN tests.rs
}


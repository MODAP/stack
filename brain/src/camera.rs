// Import pylon
use pylon_cxx::*;

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

    pub fn start(&mut self) -> Result<()> {
        self.camera.open()?;
        self.camera.start_grabbing(&pylon_cxx::GrabOptions::default());
        Ok(())
    }

    pub fn start_limited(&mut self, num: u32) -> Result<()> {
        self.camera.open()?;
        self.camera.start_grabbing(&pylon_cxx::GrabOptions::default().count(num));
        Ok(())
    }
    
    //// SYNCRONOUS OPTS ///
    /// FIXME Hi yes this will gather slices and slowly fill up memory
    fn grab_frame_buffer(&self, mut result: pylon_cxx::GrabResult) -> Result<Vec<u8>> {
        self.camera.retrieve_result(
            0,
            &mut result,
            pylon_cxx::TimeoutHandling::ThrowException,
        )?;

        // Checking if the image is grabbed succesfully
        if result.grab_succeeded()? {
            // Access the image data.
            if cfg!(debug_assertions) {
                println!("Frame Size_X: {}", result.width()?);
                println!("Frame Size_Y: {}", result.height()?);
            }

            let image_buffer = result.buffer()?;
            return Ok(image_buffer.to_vec())
        } else {
            return Err(anyhow!("YOOOOO Code: {} {}", result.error_code()?, result.error_description()?))
        }
    }

    fn grab_frame(&self) -> Result<Vec<u8>> {
        let grab_result = pylon_cxx::GrabResult::new()?;
        return Ok(self.grab_frame_buffer(grab_result)?)
    }

    #[allow(dead_code)]    
    fn grab_frame_vec(&self, num_frames: usize) -> Result<Vec<Vec<u8>>> {
        let mut frame_buffer: Vec<Vec<u8>> = Vec::new();

        for _i in 0..num_frames {
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

}


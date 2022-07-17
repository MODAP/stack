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


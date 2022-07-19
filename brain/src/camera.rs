// Import pylon
use pylon_cxx::*;

// Streaming utilites
use anyhow::{anyhow, Result};
#[allow(unused_imports)]
use tokio_stream::StreamExt;

/// Represents an camera instance
/// This would abstract away Pylon APIs
pub struct Camera<'a> {
    // and the camera
    pub camera: pylon_cxx::InstantCamera<'a>,
    started: bool,
}

// We will provide initalization and
// grab facilities
impl<'a> Camera<'a> {
    /// Initialize camera, try getting a camera, and seeding it
    pub fn new(pylon: &'a pylon_cxx::Pylon) -> Self {
        // and create new camera object
        let cam = pylon_cxx::TlFactory::instance(pylon).create_first_device();

        // Return the camera
        Camera {
            camera: match cam {
                Ok(c) => c,
                Err(_) => panic!("We need a camera."),
            },
            started: false,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.camera.open()?;
        self.camera.start_grabbing(&pylon_cxx::GrabOptions::default())?;
        self.started = true;
        Ok(())
    }

    pub fn start_limited(&mut self, num: u32) -> Result<()> {
        self.camera.open()?;
        self.camera.start_grabbing(&pylon_cxx::GrabOptions::default().count(num))?;
        self.started = true;
        Ok(())
    }

    pub fn grab_frame(&self) -> Result<Vec<u8>> {
        let mut result = pylon_cxx::GrabResult::new()?;
        self.camera.retrieve_result(
            400,
            &mut result,
            pylon_cxx::TimeoutHandling::ThrowException,
        )?;
        if result.grab_succeeded()? {
            log::trace!("Frame Size_X: {}", result.width()?);
            log::trace!("Frame Size_Y: {}", result.height()?);

            let image_buffer = result.buffer()?;
            return Ok(image_buffer.to_vec());
        } else {
            return Err(anyhow!(
                "Pylon GrabResult error: {} {}",
                result.error_code()?,
                result.error_description()?
            ));
        }
    }

    #[allow(dead_code)]
    pub fn grab_frame_vec(&self, num_frames: usize) -> Result<Vec<Vec<u8>>> {
        let mut frame_buffer: Vec<Vec<u8>> = Vec::new();

        for _i in 0..num_frames {
            frame_buffer.push(self.grab_frame()?);
        }
        return Ok(frame_buffer);
    }

    /// Dumps various information concerning the camera dn Pylon
    ///
    /// Always dumps the Pylon build version and camera model name
    /// If the camera has been started, will list the pixel format.
    ///
    /// # Returns
    /// `Result<()>`: Pylon can error when trying to fetch the model name.
    ///
    /// # Examples
    ///
    /// ```
    /// let pylon = pylon_cxx::Pylon::new();
    /// let cam = brain::Camera::new(&pylon);
    /// cam.debug().unwrap();
    /// ```
    pub fn debug(self) -> Result<()> {
        let ver = pylon_version();
        let info = self.camera.device_info();
        log::info!(
            "Pylon version {}.{}.{}, build {}.",
            ver.major,
            ver.minor,
            ver.subminor,
            ver.build
        );
        log::info!("Camera model name: {}", info.model_name()?);
        if self.started {
            log::info!(
                "Camera is started. Color profile is {}",
                self.camera.node_map().enum_node("PixelFormat")?.value()?
            );
        } else {
            log::info!("Camera not started.")
        }
        Ok(())
    }
}

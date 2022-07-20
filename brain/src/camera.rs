use pylon_cxx::*;
use anyhow::{anyhow, Result};

/// Wrapper struct around a Pylon camera instance.
///
/// Provides convienience methods for high level access.
pub struct Camera<'a> {
    // Internal Pylon camera object
    pub camera: pylon_cxx::InstantCamera<'a>,
    // If the camera has started grabbing yet
    started: bool,
}

impl<'a> Camera<'a> {
    /// TODO @jemoka could you expand the docs here
    /// Initialize camera, try getting a camera, and seeding it
    pub fn new(pylon: &'a pylon_cxx::Pylon) -> Self {
        let cam = pylon_cxx::TlFactory::instance(pylon).create_first_device();

        Camera {
            camera: match cam {
                Ok(c) => c,
                Err(_) => panic!("We need a camera."),
            },
            started: false,
        }
    }

    /// Starts grabbing frames from the camera continously.
    ///
    /// Pylon cameras need to be `.open()`d and told how to grab frames
    /// before actually taking any images. This function also sets `started`
    /// to true so that other methods know they can safely grab images/info.
    ///
    /// # Returns
    /// `Result<()>`: Pylon can error when trying to start the camera.
    ///
    /// # Examples
    ///
    /// ```
    /// let pylon = pylon_cxx::Pylon::new();
    /// let cam = brain::Camera::new(&pylon);
    /// cam.start().unwrap();
    /// ```
    pub fn start(&mut self) -> Result<()> {
        self.camera.open()?;
        self.camera.start_grabbing(&pylon_cxx::GrabOptions::default())?;
        self.started = true;
        Ok(())
    }

    /// Starts grabbing the specified number of frames from the camera.
    ///
    /// See `start()` for context. This is essentially the same thing
    /// but instead of continually grabbing frames the camera will just grab
    /// a specified amount. Useful for writing tests.
    ///
    /// # Arguments
    /// - `num`: the number of frames to grab befor stopping.
    ///
    /// # Returns
    /// `Result<()>`: Pylon can error when trying to start the camera.    
    pub fn start_limited(&mut self, num: u32) -> Result<()> {
        self.camera.open()?;
        self.camera.start_grabbing(&pylon_cxx::GrabOptions::default().count(num))?;
        self.started = true;
        Ok(())
    }

    /// Returns the latest frame from the camera.
    ///
    /// Will return a frame that the camera has grabbed (and thus relies on
    /// the camera having been started).
    ///
    /// TODO Confusing terminology: camera grabbing frame vs user grabbing
    /// TODO Make sure we know this is the _latest_ frame
    ///
    /// # Returns
    /// A vector of bytes representing the image. Use `get_pixel_fmt()` to
    /// get the format of the bytes returned.
    pub fn grab_frame(&self) -> Result<Vec<u8>> {
        let mut result = pylon_cxx::GrabResult::new()?;
        self.camera.retrieve_result(
            400, // TODO Entirely arbitrary timeout - experimentation needed.
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

    /// Returns the pixel format of the images the camera is taking as a string.
    pub fn get_pixel_fmt(&self) -> Result<String> {
        if self.started {
            Ok(self.camera.node_map().enum_node("PixelFormat")?.value()?)
        } else {
            Err(anyhow!("Camera is not started."))
        }
    }

    /// Grab multiple frames from the camera
    ///
    /// See `grab_frame()` for context - this is just a batched version.
    ///
    /// # Arguments
    /// - `num_frames`: number of frames to grab
    ///
    /// # Returns
    /// A vector of `num_frames` frames.
    pub fn grab_frame_vec(&self, num_frames: usize) -> Result<Vec<Vec<u8>>> {
        let mut frame_buffer: Vec<Vec<u8>> = Vec::new();

        for _i in 0..num_frames {
            frame_buffer.push(self.grab_frame()?);
        }
        return Ok(frame_buffer);
    }

    /// Dumps various information concerning the camera in Pylon
    ///
    /// Always dumps the Pylon build version and camera model name
    /// If the camera has been started, will dump the pixel format.
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

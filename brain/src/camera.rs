// Import pylon
use pylon_cxx;

// Streaming utilites
#[allow(unused_imports)]
use tokio_stream::StreamExt;
use anyhow::Result;

/// Represents an camera instance
/// This would abstract away Pylon APIs
pub struct Camera<'a> {
    // and the camera
    pub camera: Option<pylon_cxx::InstantCamera<'a>>
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
                Ok(c) => Some(c),
                Err(_) => None
            }
        }
    }

    //// SYNCRONOUS OPTS ////
    // fn camera_info() -> (design and return serialized struct)
    // fn camera_grab(n) -> Result<ndarray::Array4> # grab n frames
    fn grab_frame(self) -> Result<Vec<u8>, (u32, String)> {
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
            return Err((grab_result.error_code()?, grab_result.error_description()?))
        }
    }
    
    fn grab_frame_buffer(self, num_frames: usize) -> Result<Vec<Vec<u8>>> {
        let frame_buffer: Vec<Vec<u8>>;

        for i in 0..num_frames {
            frame_buffer.push(self.grab_frame()?);
        }
        return Ok(frame_buffer)
    }
    // Example: https://github.com/strawlab/pylon-cxx/blob/09ce34be4a84dd63a7a2a0f588ab546412b3bc83/examples/grab.rs#L34-L54

    //// ASYNC OPTS ////
    // fn camera_stream() -> Result<tokio_stream::Stream<ndarray::Array3>> # streaming buffer
    // Example: https://github.com/strawlab/pylon-cxx/blob/main/examples/async-grab.rs

    // REMINDER TO WRITE TESTS IN tests.rs
}


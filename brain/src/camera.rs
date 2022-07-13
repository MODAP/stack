// Import pylon
use pylon_cxx;

// Streaming utilites
#[allow(unused_imports)]
use tokio_stream::StreamExt;

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
    // Example: https://github.com/strawlab/pylon-cxx/blob/09ce34be4a84dd63a7a2a0f588ab546412b3bc83/examples/grab.rs#L34-L54

    //// ASYNC OPTS ////
    // fn camera_stream() -> Result<tokio_stream::Stream<ndarray::Array3>> # streaming buffer
    // Example: https://github.com/strawlab/pylon-cxx/blob/main/examples/async-grab.rs

    // REMINDER TO WRITE TESTS IN tests.rs
}


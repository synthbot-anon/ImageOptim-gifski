use rgb::RGBA8;
use std::ffi::CString;
use std::mem;
use pyo3::{PyResult, exceptions};
use pyo3::prelude::*;
use crate::c_api::*;

/// Gifski(width, height, /, quality=90, fast=False, repeat=0)
///
/// Example usage for creating a gif:
///     frame_duration = 1 / 24 # 24 frames per second
///     g = Gifski(width, height)
///     g.set_file_output("output/path.gif")
///
///     timestamp = 0
///     for frame in imgs:
///         pixels = frame.convert('RGBA').tobytes()
///         g.add_frame_rgba(pixels, timestamp)
///         timestamp += frame_duration
///
///     g.finish()
/// 
/// Parameters
/// ----------
/// width : int
///     positive integer, pixel width
/// height : int
///     positive integer, pixel height
/// quality : int
///     integer from 1 (best compression) to 100 (best quality)
/// fast : bool
///     faster encoder, lower quality
/// repeat : int
///     -1 for no looping, 0 for infinite looping, or n for looping n times
#[pyclass]
#[pyo3(name="Gifski")]
struct PyGifski {
    _handle: usize,
    width: u32,
    height: u32,
    frame_count: u32,
}

#[pymethods]
impl PyGifski {

    #[new]
    #[args(quality=90, fast=false, repeat=0)]
    unsafe fn new(width: u32, height: u32, quality: u8, fast: bool, repeat: i16) -> PyResult<Self> {
        if width == 0 || height == 0 {
            return Err(exceptions::PyValueError::new_err("width and height must be greater than 0"));
        }
        if quality < 1 || quality > 100 {
            return Err(exceptions::PyValueError::new_err("quality must be between 1 and 1000"));
        }
        if repeat < -1 {
            return Err(exceptions::PyValueError::new_err("repeat must be -1, 0, or positive"));
        }

        let settings = GifskiSettings {
            width, height, quality, fast, repeat,
        };

        Ok(PyGifski {
            _handle: gifski_new(&settings) as usize,
            width,
            height,
            frame_count: 0,
        })
    }

    /// Set the gif output destination to the given file path.
    ///
    /// This method should only be called once on a Gifski object.
    ///
    /// For a complete list of errors, see the GifskiError enum here:
    ///     https://github.com/synthbot-anon/ImageOptim-gifski/blob/main/gifski.h
    ///
    /// Common errors:
    ///     INVALID_STATE: the output might have already been set for this object.
    ///     NOT_FOUND: the target directory doesn't exist.
    ///     PERMISSION_DENIED: the target file is not writable.
    ///     ALREADY_EXISTS: the target file already exists.
    ///
    /// Parameters
    /// ----------
    /// destination : str
    ///     File path for writing the output gif.
    #[pyo3(text_signature = "(self, destination, /)")]
    unsafe fn set_file_output(&self, destination: String) -> PyResult<()> {
        let handle = self._handle as *const GifskiHandle;

        // make this mutable so we can get the pointer without deallocating
        let mut c_str = CString::new(destination);
        let ptr = c_str.as_mut().unwrap().as_ptr();


        let success = gifski_set_file_output(handle, ptr);
        if success as u8 == 0 {
            return Ok(());
        }

        Err(exceptions::PyException::new_err(success.to_string()))
    }

    /// Specify a new gif frame using a pixel buffer.
    ///
    /// Example for getting a pixel buffer:
    ///     from PIL import Image
    ///     image = Image.open(image_path, mode='r')
    ///     pixels = image.convert('RGBA').tobytes()
    ///
    /// Parameters
    /// ----------
    /// pixels : bytes
    ///     RGBA pixel data, 4 bytes per pixel. The number of pixels must match the
    ///     width and height provided when creating the Gifski object.
    #[pyo3(text_signature = "(self, pixels, timestamp, /)")]
    unsafe fn add_frame_rgba(&mut self, pixels: &[u8], timestamp: f64) -> PyResult<()> {
        let handle = self._handle as *const GifskiHandle;
        if pixels.len() % 4 != 0 {
            return Err(exceptions::PyValueError::new_err("pixels must be in RGBA format, 4 bytes per pixel"));
        }
        if self.width * self.height * 4 != pixels.len() as u32 {
            return Err(exceptions::PyValueError::new_err("pixel width*height doesn't match the width*height used during construction"));
        }
        if self.frame_count > 0 && timestamp == 0.0 {
            return Err(exceptions::PyValueError::new_err("only the first frame's timestamp is allowed to be 0"));
        }

        let buffer = mem::transmute::<*const u8, *const RGBA8>(pixels.as_ptr());
        let success = gifski_add_frame_rgba(handle, self.frame_count, self.width, self.height, buffer, timestamp);
        if success as u8 != 0 {
            return Err(exceptions::PyException::new_err(success.to_string()));
        }

        self.frame_count += 1;
        return Ok(());
    }

    /// Finalize the gif and write the output.
    ///
    /// No further methods should be called on this object after calling finish().
    #[pyo3(text_signature = "(self, /)")]
    unsafe fn finish(&self) -> PyResult<()> {
        let handle = self._handle as *const GifskiHandle;
        let success = gifski_finish(handle);
        if success as u8 == 0 {
            return Ok(());
        }

        return Err(exceptions::PyException::new_err(success.to_string()));
    }
}

#[pymodule]
fn gifski(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGifski>()?;
    Ok(())
}

use rgb::RGBA8;
use std::ffi::CString;
use std::mem;
use pyo3::{PyResult, exceptions};
use pyo3::prelude::*;
use crate::c_api::*;

#[pyclass]
#[pyo3(name="Gifski")]
#[pyo3(text_signature = "(width, height, /, quality=90, fast=False, repeat=-1)")]
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

    #[pyo3(text_signature = "(self, pixels, timestamp, /)")]
    unsafe fn add_frame_rgba(&mut self, pixels: &[u8], timestamp: f64) -> PyResult<()> {
        let handle = self._handle as *const GifskiHandle;
        if pixels.len() % 4 != 0 {
            return Err(exceptions::PyValueError::new_err("pixels must be in RGBA format, 4 bytes per pixel"));
        }
        if self.width * self.height != pixels.len() as u32 {
            return Err(exceptions::PyValueError::new_err("pixel width*height doesn't match the width*height used during construction"));
        }

        let buffer = mem::transmute::<*const u8, *const RGBA8>(pixels.as_ptr());
        let success = gifski_add_frame_rgba(handle, self.frame_count, self.width, self.height, buffer, timestamp);
        if success as u8 != 0 {
            return Err(exceptions::PyException::new_err(success.to_string()));
        }

        self.frame_count += 1;
        return Ok(());
    }

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

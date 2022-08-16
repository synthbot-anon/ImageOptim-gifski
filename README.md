A fork of [gifski, from ImageOptim](https://github.com/ImageOptim/gifski) to include bindings for Python.


# Installing from source
Dependencies:
- git (https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
- Rust (https://rustup.rs/)
- virtualenv (`pip install virtualenv`)

After installing Rust, you will need to restart your shell/terminal for the environment variable changes to take effect.

After installing the dependencies and restarting your shell/terminal:
```
# compile the wheel
python3 -m virtualenv .venv
.venv/bin/pip wheel 'gifski@git+https://github.com/synthbot-anon/ImageOptim-gifski.git'

# install the compiled wheel
pip install gifski-*.whl

# clean up
rm -r .venv
rm gifski-*.whl
```

# Usage
```
Help on package gifski:

NAME
    gifski

PACKAGE CONTENTS
    gifski

CLASSES
    builtins.object
        builtins.Gifski
    
    class Gifski(object)
     |  Gifski(width, height, /, quality=90, fast=False, repeat=0)
     |  
     |  Example usage for creating a gif:
     |      frame_duration = 1 / 24 # 24 frames per second
     |      g = Gifski(width, height)
     |      g.set_file_output("output/path.gif")
     |  
     |      timestamp = 0
     |      for frame in imgs:
     |          pixels = frame.convert('RGBA').tobytes()
     |          g.add_frame_rgba(pixels, timestamp)
     |          timestamp += frame_duration
     |  
     |      g.finish()
     |  
     |  Parameters
     |  ----------
     |  width : int
     |      positive integer, pixel width
     |  height : int
     |      positive integer, pixel height
     |  quality : int
     |      integer from 1 (best compression) to 100 (best quality)
     |  fast : bool
     |      faster encoder, lower quality
     |  repeat : int
     |      -1 for no looping, 0 for infinite looping, or n for looping n times
     |  
     |  Methods defined here:
     |  
     |  add_frame_rgba(self, pixels, timestamp, /)
     |      Specify a new gif frame using a pixel buffer.
     |      
     |      Example for getting a pixel buffer:
     |          from PIL import Image
     |          image = Image.open(image_path, mode='r')
     |          pixels = image.convert('RGBA').tobytes()
     |      
     |      Parameters
     |      ----------
     |      pixels : bytes
     |          RGBA pixel data, 4 bytes per pixel. The number of pixels must match the
     |          width and height provided when creating the Gifski object.
     |  
     |  finish(self, /)
     |      Finalize the gif and write the output.
     |      
     |      No further methods should be called on this object after calling finish().
     |  
     |  set_file_output(self, destination, /)
     |      Set the gif output destination to the given file path.
     |      
     |      This method should only be called once on a Gifski object.
     |      
     |      For a complete list of errors, see the GifskiError enum here:
     |          https://github.com/synthbot-anon/ImageOptim-gifski/blob/main/gifski.h
     |      
     |      Common errors:
     |          INVALID_STATE: the output might have already been set for this object.
     |          NOT_FOUND: the target directory doesn't exist.
     |          PERMISSION_DENIED: the target file is not writable.
     |          ALREADY_EXISTS: the target file already exists.
     |      
     |      Parameters
     |      ----------
     |      destination : str
     |          File path for writing the output gif.
     |  
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |  
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.

DATA
    __all__ = ['Gifski']

```
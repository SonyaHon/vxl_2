pub struct Texture {
    data: Vec<u8>,
    width: u32,
    height: u32,
    id: gl::types::GLuint,
}

impl Texture {
    pub fn new(image: image::DynamicImage) -> Texture {
        let img = match image {
            image::DynamicImage::ImageRgba8(img) => img,
            img => img.to_rgba(),
        };

        let dimensions = img.dimensions();
        let img_data = img.into_raw();

        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                dimensions.0 as i32,
                dimensions.1 as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                &img_data[0] as *const u8 as *const std::ffi::c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture {
            data: img_data,
            width: dimensions.0,
            height: dimensions.1,
            id,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

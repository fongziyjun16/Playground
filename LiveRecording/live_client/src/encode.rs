use openh264::{encoder::Encoder, formats::YUVSource};

#[derive(Clone, Debug, Default)]
pub struct CapturedYUV {
    y: Vec<u8>,
    u: Vec<u8>,
    v: Vec<u8>,
    width: usize,
    height: usize,
}

impl CapturedYUV {
    fn from_frame_buffer(width: usize, height: usize, buffer: &[u8]) -> CapturedYUV {
        let mut y_channel = vec![0u8; width * height];
        let mut u_channel = vec![0u8; (width / 2) * (height / 2)];
        let mut v_channel = vec![0u8; (width / 2) * (height / 2)];

        for j in 0..height {
            for i in 0..width {
                let index = 4 * (j * width + i);
                let r = buffer[index] as f32;
                let g = buffer[index + 1] as f32;
                let b = buffer[index + 2] as f32;

                let y_val = (0.299 * r + 0.587 * g + 0.114 * b).round() as u8;
                let u_val = (-0.168736 * r - 0.331264 * g + 0.5 * b + 128.0).round() as u8;
                let v_val = (0.5 * r - 0.418688 * g - 0.081312 * b + 128.0).round() as u8;

                y_channel[j * width + i] = y_val;

                if j % 2 == 0 && i % 2 == 0 {
                    u_channel[(j / 2) * (width / 2) + (i / 2)] = u_val;
                    v_channel[(j / 2) * (width / 2) + (i / 2)] = v_val;
                }
            }
        }

        CapturedYUV {
            // y,
            // u,
            // v,
            y: y_channel,
            u: u_channel,
            v: v_channel,
            width,
            height,
        }
    }
}

impl YUVSource for CapturedYUV {
    fn y(&self) -> &[u8] {
        &self.y
    }

    fn u(&self) -> &[u8] {
        &self.u
    }

    fn v(&self) -> &[u8] {
        &self.v
    }

    fn strides(&self) -> (usize, usize, usize) {
        (self.width, self.width / 2, self.width / 2)
    }

    fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

pub fn rgba_encode(width: u32, height: u32, buffer: &[u8]) -> Option<(u32, u32, Vec<u8>)> {
    let start_timestamp = chrono::Utc::now().timestamp_millis();
    // return match image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(
    //     width as u32,
    //     height as u32,
    //     buffer,
    // ) {
    //     Some(image) => Some((width, height, image.to_vec())),
    //     None => None,
    // };
    // Convert the frame from BGRA to RGBA
    let mut rgba_buffer = vec![0; buffer.len()];
    for i in 0..(buffer.len() / 4) {
        rgba_buffer[i * 4 + 0] = buffer[i * 4 + 2]; // R
        rgba_buffer[i * 4 + 1] = buffer[i * 4 + 1]; // G
        rgba_buffer[i * 4 + 2] = buffer[i * 4 + 0]; // B
        rgba_buffer[i * 4 + 3] = buffer[i * 4 + 3]; // A
    }
    let stop_timestamp = chrono::Utc::now().timestamp_millis();
    log::debug!("rgba encode spent {} ms", stop_timestamp - start_timestamp);
    Some((width, height, rgba_buffer))
}

pub fn h264_encode(width: u32, height: u32, buffer: &[u8]) -> Vec<u8> {
    let start_timestamp = chrono::Utc::now().timestamp_millis();
    let yuv = CapturedYUV::from_frame_buffer(width as usize, height as usize, &buffer);
    match Encoder::new() {
        Ok(mut encoder) => match encoder.encode(&yuv) {
            Ok(encoded_bit_stream) => {
                let stop_timestamp = chrono::Utc::now().timestamp_millis();
                log::debug!("h264 encode spent {} ms", stop_timestamp - start_timestamp);
                return encoded_bit_stream.to_vec();
            }
            Err(err) => log::error!("Fail to encode the given yuv source, Error: {:?}", err),
        },
        Err(err) => log::error!("Fail to get an encoder, Error: {:?}", err),
    }
    Vec::new()
}

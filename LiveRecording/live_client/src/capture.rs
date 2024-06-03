pub fn scrap_capture() -> Option<(u32, u32, Vec<u8>)> {
    use scrap::{Capturer, Display};
    let display = Display::primary().unwrap();
    let mut capture = Capturer::new(display).unwrap();
    let (width, height) = (capture.width(), capture.height());

    let mut frame = vec![0; 4 * width * height];

    let start_timestamp = chrono::Utc::now().timestamp_millis();
    loop {
        match capture.frame() {
            Ok(buffer) => {
                if frame.len() == buffer.len() {
                    frame.copy_from_slice(&buffer);
                    break;
                }
            }
            Err(error) => {
                if error.kind() == std::io::ErrorKind::WouldBlock {
                    continue;
                } else {
                    return None;
                }
            }
        }
    }
    let stop_timestamp = chrono::Utc::now().timestamp_millis();
    log::debug!("screenshot spent {} ms", stop_timestamp - start_timestamp);

    Some((width as u32, height as u32, frame.to_vec()))
}

pub static H264_PACKET_SENDER: once_cell::sync::Lazy<
    std::sync::Mutex<Option<std::sync::mpsc::Sender<Vec<u8>>>>,
> = once_cell::sync::Lazy::new(|| std::sync::Mutex::new(None));

#[link(name = "live_push")]
extern "C" {
    pub fn start_push(push: extern "C" fn(*const libc::c_uchar, libc::c_int) -> ());
    pub fn stop_push();
}

pub extern "C" fn push(buffer: *const libc::c_uchar, size: libc::c_int) {
    unsafe {
        let slice = std::slice::from_raw_parts(buffer, size as usize);
        if let Some(sender) = &*H264_PACKET_SENDER.lock().unwrap() {
            let _ = sender.send(slice.to_vec());
            // log::info!("timestamp {} ms", chrono::Utc::now().timestamp_millis());
            // log::info!("size {}", slice.len());
        }
    }
}

use std::{io::Write, thread::spawn};

use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;

mod capture;
mod encode;

fn recording_slow(rate: u8, seconds: u64, filename: &str) {
    let _ = std::fs::remove_file(filename);
    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
    {
        Ok(mut file) => {
            let start = std::time::Instant::now();
            let duration = std::time::Duration::new(seconds, 0);
            let frame_time = std::time::Duration::from_secs_f64(1.0 / rate as f64);
            while start.elapsed() < duration {
                let frame_start = std::time::Instant::now();
                if let Some((width, height, buffer)) = capture::scrap_capture() {
                    if let Some((width, height, buffer)) =
                        encode::rgba_encode(width, height, &buffer)
                    {
                        let _ = file.write_all(&encode::h264_encode(width, height, &buffer));
                    }
                }
                let elapsed = frame_start.elapsed();
                if frame_start.elapsed() < frame_time {
                    // std::thread::sleep(frame_time - elapsed);
                }
            }
        }
        Err(err) => log::error!("Fail to open file, Error: {:?}", err),
    }
}

fn recording(filename: &str) {
    let _ = std::fs::remove_file(filename);
    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
    {
        Ok(mut file) => {
            let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();
            std::thread::spawn(move || {
                {
                    let mut h264_packet_sender = capture::H264_PACKET_SENDER.lock().unwrap();
                    let _ = h264_packet_sender.insert(tx);
                }
                unsafe {
                    log::info!("start push");
                    capture::start_push(capture::push);
                    log::info!("stop push");
                }
            });
            loop {
                match rx.recv() {
                    Ok(packet) => {
                        let _ = file.write_all(&packet);
                    }
                    Err(_) => break,
                }
            }
        }
        Err(err) => log::error!("Fail to open file, Error: {:?}", err),
    }
}

#[tokio::main]
async fn live() {
    let address = "ws://localhost:8080";
    match tokio_tungstenite::connect_async(address).await {
        Ok((mut socket, _)) => {
            // loop {
            //     if let Some((width, height, buffer)) = capture::scrap_capture() {
            //         if let Some((width, height, buffer)) =
            //             encode::rgba_encode(width, height, &buffer)
            //         {
            //             let data = encode::h264_encode(width, height, &buffer);
            //             let _ = socket.send(Message::Binary(data)).await;
            //         }
            //     }
            //     tokio::time::sleep(tokio::time::Duration::from_millis(1000 / 16)).await;
            // }
            let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();
            std::thread::spawn(move || {
                {
                    let mut h264_packet_sender = capture::H264_PACKET_SENDER.lock().unwrap();
                    let _ = h264_packet_sender.insert(tx);
                }
                unsafe {
                    log::info!("start push");
                    capture::start_push(capture::push);
                    log::info!("stop push");
                }
            });
            loop {
                match rx.recv() {
                    Ok(packet) => {
                        let _ = socket.send(Message::Binary(packet)).await;
                    }
                    Err(_) => break,
                }
            }
            // if let Ok(packet) = rx.recv() {
            //     let _ = socket.send(Message::Binary(packet)).await;
            // }
            // if let Ok(packet) = rx.recv() {
            //     let _ = socket.send(Message::Binary(packet)).await;
            // }
            unsafe {
                capture::stop_push();
            }
        }
        Err(err) => log::error!("Fail to connect to server {}, Error: {:?}", address, err),
    }
}

fn bench() {
    let mut start_timestamp = chrono::Utc::now().timestamp_millis();
    let (width, height, buffer) = capture::scrap_capture().unwrap();
    let mut stop_timestamp = chrono::Utc::now().timestamp_millis();
    log::info!("capture spent: {} ms", stop_timestamp - start_timestamp);

    start_timestamp = chrono::Utc::now().timestamp_millis();
    let _ = encode::h264_encode(width, height, &buffer);
    stop_timestamp = chrono::Utc::now().timestamp_millis();
    log::info!("encode spent: {} ms", stop_timestamp - start_timestamp);
}

fn rtmp_push() {
    let address = "rtmp://192.168.1.191:1985/sr/ls";
    let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();
    std::thread::spawn(move || {
        {
            let mut h264_packet_sender = capture::H264_PACKET_SENDER.lock().unwrap();
            let _ = h264_packet_sender.insert(tx);
        }
        unsafe {
            log::info!("start push");
            capture::start_push(capture::push);
            log::info!("stop push");
        }
    });
    match std::process::Command::new("ffmpeg")
        .arg("-re")
        .arg("-f")
        .arg("h264")
        .arg("-i")
        .arg("pipe:0")
        .arg("-vcodec")
        .arg("copy")
        .arg("-f")
        .arg("flv")
        .arg(address)
        .stdin(std::process::Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            // send
            loop {
                // for _ in 0..3000 {
                match rx.recv() {
                    Ok(packet) => {
                        if let Some(child_stdin) = child.stdin.as_mut() {
                            if let Err(err) = child_stdin.write_all(&packet) {
                                log::error!("Fail to write child stdin, Error: {:?}", err);
                            }
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
                // }
            }
        }
        Err(err) => log::error!("Command Error: {:?}", err),
    }
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // recording(r"C:\Users\yingj\Desktop\test\demo.mp4");
    // live();
    rtmp_push();
}

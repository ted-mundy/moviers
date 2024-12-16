// use std::sync::Arc;

// use moviers::{clip::color, render::render as render};




// fn main() {
//   let clip = color::ColorClip {
//     color: color::Color::RGB(255, 0, 0),
//     width: 4096,
//     height: 2160,
//     duration: 10,
//   };

//   let renderer_result = render::ClipRendererBuilder::default()
//     .output_path(String::from("output.mp4"))
//     .clips(Vec::new())
//     .build();

//   assert!(renderer_result.is_ok());

//   let mut renderer = renderer_result.unwrap();

//   renderer.push_clip(Arc::new(clip));

//   renderer.write_video();
// }

use rayon::prelude::*;
use std::io::{BufReader, BufWriter, Read, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::thread;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const FPS: usize = 30;
const DURATION: usize = 30;
const FRAME_COUNT: usize = FPS * DURATION;
const FRAME_SIZE: usize = WIDTH * HEIGHT * 3; // RGB format

/// Generates a single frame of solid red color
fn generate_red_frame() -> Vec<u8> {
    let mut frame = vec![0u8; FRAME_SIZE];

    // Use parallel iteration to fill the frame with red (R=255, G=0, B=0)
    frame.par_chunks_mut(3).for_each(|pixel| {
        pixel[0] = 255; // Red
        pixel[1] = 0;   // Green
        pixel[2] = 0;   // Blue
    });

    frame
}

// fn generate_frame_from_mp4_file() -> Vec<u8> {
//     let mut frame = vec![0u8; FRAME_SIZE];

//     frame
// }

/// Write raw RGB frames to ffmpeg to create a video
// fn create_video() -> std::io::Result<()> {
//     // Generate a single red frame
//     let frame = generate_red_frame();

//     // Spawn the ffmpeg process
//     let mut ffmpeg = Command::new("ffmpeg")
//         .args(&[
//             "-y",
//             "-loglevel",
//             "error",
//             "-f",
//             "rawvideo",
//             "-vcodec",
//             "rawvideo",
//             "-s",
//             &format!("{}x{}", WIDTH, HEIGHT),
//             "-pix_fmt",
//             "rgb24",
//             "-r",
//             &format!("{}", FPS),
//             "-an",
//             "-i",
//             "-",
//             "-vcodec",
//             "libx264",
//             "-preset",
//             "ultrafast",
//             "-pix_fmt",
//             "yuv420p",
//             "output.mp4",
//         ])
//         .stdin(Stdio::piped())
//         .spawn()?;

//     // Open a buffered writer to the stdin of the ffmpeg process
//     {
//         let stdin = ffmpeg.stdin.as_mut().expect("Failed to open stdin");
//         let mut writer = BufWriter::new(stdin);

//         // Write each frame to the ffmpeg process
//         for _ in 0..FRAME_COUNT {
//             writer.write_all(&frame)?;
//         }

//         writer.flush()?;
//     }

//     ffmpeg.wait()?; // Wait for ffmpeg to finish

//     Ok(())
// }

fn create_video_from_video() -> std::io::Result<()> {
  const input: &str = "bunny.mp4";

  // we read the frame into memory
  let mut ffmpeg = Command::new("ffmpeg")
    .args(&[
      "-i",
      input,
      "-f",
      "rawvideo",
      "-pix_fmt",
      "rgb24",
      "-",
    ])
    .stderr(Stdio::null())
    .stdout(Stdio::piped())
    .spawn()?;

  const MAX_FRAMES: usize = DURATION * FPS;

  let stdout = ffmpeg.stdout.take().unwrap();

  let mut reader = BufReader::new(stdout);

  // we write the frame to the output file
  let (sender, receiver) = channel::<(usize, Vec<u8>)>();

  thread::spawn(move || {
    let mut frame_index = 0;
    let mut buffer = vec![0u8; FRAME_SIZE];

    while frame_index < MAX_FRAMES {
      if let Ok(_) = reader.read_exact(&mut buffer) {
        // Send the frame to the processing channel
        if sender.send((frame_index, buffer.clone())).is_err() {
          break;
        }
        frame_index += 1;
      }
    }
  });

// Use Rayon to process frames in parallel
receiver
    .into_iter()
    .par_bridge() // Convert the iterator to a parallel iterator
    .for_each(|(frame_index, frame_data)| {
        process_frame(frame_index, frame_data);
    });

  // Wait for FFmpeg process to finish
  ffmpeg.wait().expect("Failed to wait for ffmpeg process");

  Ok(())
}

fn process_frame(frame_index: usize, frame_data: Vec<u8>) {
    // Process the frame here
    // For now, we just print the frame index
    // println!("Processing frame {}", frame_index);
}

fn main() {
    match create_video_from_video() {
        Ok(_) => println!("Video generated successfully."),
        Err(e) => eprintln!("Error generating video: {}", e),
    }
}

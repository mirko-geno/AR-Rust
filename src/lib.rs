use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};
use std::{
    ops::Drop,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle}
};


pub struct Stream {
    cam_thread: Option<JoinHandle<()>>,
    frame: Arc<Mutex<Mat>>
}

impl Stream {
    pub fn new(cam_idx: i32) -> Stream {
        let frame = Arc::new(Mutex::new(Mat::default()));
        let frame_clone = Arc::clone(&frame);

        let cam_thread = thread::spawn(move || {
            let mut cam = match VideoCapture::new(cam_idx, CAP_ANY) { // Open 'cam_idx' camera
                Ok(cam) => cam,
                Err(e) => {
                    eprintln!("Failed to open camera: {}", e);
                    return;
                }
            };


            while opencv::highgui::wait_key(1).unwrap() != 27 {
                let mut guard = match frame_clone.lock() {
                    Ok(guard) => guard,
                    Err(_) => {
                        eprintln!("Mutex poisoned, exiting thread...");
                        break;
                    }
                };
                
                if let Err(e) = cam.read(&mut *guard) {
                    eprintln!("Failed to read frame: {}", e)
                }
            }
        });

        Stream { cam_thread: Some(cam_thread), frame }
    }

    pub fn camera_state(&self) -> &Option<JoinHandle<()>> {
        &self.cam_thread
    }

    pub fn frame(&mut self) -> Arc<Mutex<Mat>> {
        Arc::clone(&self.frame)
    }

    pub fn show(&self) {
        if let Ok(guard) = self.frame.lock() {
            let size = guard.size().unwrap();
            if size.width > 0 && size.height > 0 {
                if let Err(e) = opencv::highgui::imshow("Camera Feed", &*guard) {
                    eprintln!("Failed to display frame: {}", e);
                }
            } else {
                eprintln!("Invalid frame dimensions, skipping display.");
            }
        } else {
            eprintln!("Failed to lock frame for display.");
        }
        
    }
}


impl Drop for Stream {
    fn drop(&mut self) {
        match self.cam_thread.take() {
            Some(thread) => {
                println!("Dropping camera thread...");
                thread.join().unwrap();
            },
            None => println!("Camera thread already dropped")
        }
    }
}
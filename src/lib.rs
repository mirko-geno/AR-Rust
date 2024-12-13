use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};
use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, Condvar},
    thread,
};

pub struct Stream {
    frame: Arc<(Mutex<Mat>, Condvar)>,
    stop: Arc<AtomicBool>,
}

impl Stream {
    pub fn new(cam_idx: i32) -> Self {
        let mut cam = VideoCapture::new(cam_idx, CAP_ANY).unwrap(); // Open 'cam_idx' camera

        let frame = Arc::new((Mutex::new(Mat::default()), Condvar::new()));
        let frame_clone = Arc::clone(&frame);

        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = Arc::clone(&stop);

        thread::spawn(move || {
            while !stop_clone.load(Ordering::Relaxed) {
                let (lock, cvar) = &*frame_clone;
                let mut guard = lock.lock().unwrap();

                // Read frame without locking the Mutex for too long
                if let Err(e) = cam.read(&mut *guard) {
                    eprintln!("Failed to read frame: {}", e);
                    continue;
                }

                // Notify that a new frame is available
                cvar.notify_one();
            }
        });

        Stream { frame, stop }
    }

    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    pub fn frame(&self) -> Arc<(Mutex<Mat>, Condvar)> {
        Arc::clone(&self.frame)
    }

    pub fn show(&self) {
        let (lock, cvar) = &*self.frame;
        let mut guard = lock.lock().unwrap();
    
        // Wait for the condition variable to notify that a frame is ready
        while guard.size().unwrap().width == 0 || guard.size().unwrap().height == 0 {
            guard = cvar.wait(guard).unwrap(); // Wait until notified and re-acquire the lock
        }
    
        opencv::highgui::imshow("Camera Feed", &*guard).unwrap();
    }
}

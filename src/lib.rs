use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};

pub struct Stream {
    cam: VideoCapture,
    frame: Mat
}

impl Stream {
    pub fn new(cam_idx: i32) -> Stream {
        Stream {
            cam: VideoCapture::new(cam_idx, CAP_ANY).unwrap(), // Open 'cam_idx' camera
            frame: Mat::default()
        }
    }

    pub fn frame(&mut self) -> &Mat {
        self.cam.read(&mut self.frame).unwrap();
        &self.frame
    }

    pub fn update(&mut self) {
        self.cam.read(&mut self.frame).unwrap();
    }

    pub fn show(&self) {
        opencv::highgui::imshow("Camera Feed", &self.frame).unwrap();
    }
}

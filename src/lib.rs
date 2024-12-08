use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};
use opencv::imgproc::circle;
use opencv::core::{Point, Scalar, Vector};
use mediapipe_rs::tasks::vision::{HandLandmarkerBuilder, HandLandmarker};


pub struct Stream {
    cam: VideoCapture,
    frame: Mat,
    hand_landmarker: Option<HandLandmarker>, // Use HandLandmarker instead of HandLandmarkerSession
}

impl Stream {
    pub fn new(cam_idx: i32, model_path: &str) -> Stream {
        let hand_landmarker = HandLandmarkerBuilder::new()
            .num_hands(1) // Detect up to 1 hand
            .build_from_file(model_path) // Load model
            .unwrap();
        
        Stream {
            cam: VideoCapture::new(cam_idx, CAP_ANY).unwrap(),
            frame: Mat::default(),
            hand_landmarker: Some(hand_landmarker), // Now correct
        }
    }

    pub fn frame_as_bytes(&mut self) -> Vec<u8> {
        self.cam.read(&mut self.frame).unwrap();

        let params = Vector::new();
        let mut bytes = Vector::new();
        opencv::imgcodecs::imencode(".jpg", &self.frame, &mut bytes, &params).unwrap();
        bytes.to_vec()
    }

    pub fn update(&mut self) {
        self.cam.read(&mut self.frame).unwrap();
    }

    pub fn process_frame<F>(&mut self, mut process_fn: F)
    where
        F: FnMut(&mut Mat),
    {
        self.update(); // Update frame
        process_fn(&mut self.frame); // Apply external processing
    }

    pub fn show(&self) {
        opencv::highgui::imshow("Camera Feed", &self.frame).unwrap();
    }

    // Detect hands and draw landmarks on the frame
    pub fn detect_and_draw_landmarks(&mut self) {
        // First, obtain the frame bytes mutably
        let frame_bytes = self.frame_as_bytes(); 
    
        // Now borrow `self.hand_landmarker` immutably
        if let Some(landmarker) = &self.hand_landmarker {
            let img = image::load_from_memory(&frame_bytes).unwrap();
            let hand_landmarks = landmarker.detect(&img).unwrap();
    
            // Process the hand landmarks
            for result in hand_landmarks {
                // Borrow the first landmark by reference
                let landmarks = &result.hand_landmarks[0];
                let coordinates: Vec<(f32, f32)> = vec![
                    (landmarks.x, landmarks.y) // Access directly by field
                ];
                self.draw_landmarks(&coordinates);
            }
        }
    }
    

    pub fn draw_landmarks(&mut self, landmarks: &[(f32, f32)]) {
        for (x, y) in landmarks {
            let point = Point::new(*x as i32, *y as i32);
            circle(
                &mut self.frame,
                point,
                5,
                Scalar::new(0.0, 255.0, 0.0, 0.0), // Green color
                -1,
                opencv::imgproc::LINE_8,
                0,
            )
            .unwrap();
        }
    }
}

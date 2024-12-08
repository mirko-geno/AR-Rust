use AR_Rust::Stream;
use opencv::highgui;

const MODEL_PATH: &str = "hand_landmarker.task";

fn main() {
    // Create a new stream with the model path
    let mut stream = Stream::new(7, MODEL_PATH);

    // Loop to capture video and process it
    while highgui::wait_key(1).unwrap() != 27 { // Exit on 'Esc'
        stream.update(); // Update stream with new frames
        stream.detect_and_draw_landmarks(); // Detect and draw landmarks
        stream.show(); // Display the processed frame
    }
}

use ar_rust::Stream;



fn main() {
    let stream = Stream::new(7);
    while opencv::highgui::wait_key(1).unwrap() != 27 { // Exit on 'Esc'
        stream.show();
    } 
}
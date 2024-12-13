use ar_rust::Stream;


fn main() {
	let mut stream = Stream::new(7);

	while opencv::highgui::wait_key(1).unwrap() != 27 {
		stream.show();
	}

	stream.stop();
}

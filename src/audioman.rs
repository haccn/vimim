use rand::Rng;
use rodio::source::Source;

pub static EXPLOSION_BYTES: &'static [u8] = include_bytes!("../audio/explosion.mp3");
static FOOTSTEPS: &'static [&'static [u8]] = &[
	include_bytes!("../audio/footstep0.mp3"),
	include_bytes!("../audio/footstep1.mp3"),
	include_bytes!("../audio/footstep2.mp3"),
	include_bytes!("../audio/footstep3.mp3"),
];
static MUSIC_BYTES: &'static [u8] = include_bytes!("../audio/music.mp3");

pub struct AudioMan {
	_stream: rodio::OutputStream,
	stream_handle: rodio::OutputStreamHandle,
	footstep_track: rodio::Sink,
}

impl AudioMan {
	pub fn new() -> AudioMan {
		let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
		let sink = rodio::Sink::try_new(&stream_handle).unwrap();

		let reader = std::io::Cursor::new(MUSIC_BYTES);
		let source = rodio::Decoder::new(reader).unwrap()
			.repeat_infinite()
			.amplify(0.2)
			.convert_samples();
		stream_handle.play_raw(source).unwrap();

		AudioMan {
			_stream,
			stream_handle,
			footstep_track: sink,
		}
	}

	pub fn play(&self, audio_bytes: &'static [u8]) {
		let reader = std::io::Cursor::new(audio_bytes);
		let source = rodio::Decoder::new(reader).unwrap()
			.convert_samples();
		self.stream_handle.play_raw(source).unwrap();
	}

	pub fn play_rand_footstep(&self) {
		if self.footstep_track.len() > 0 {
			return;
		}
		let rand_idx = rand::thread_rng().gen_range(0..4);
		let reader = std::io::Cursor::new(FOOTSTEPS[rand_idx]);
		let source = rodio::Decoder::new(reader).unwrap();
		self.footstep_track.append(source);
	}
}

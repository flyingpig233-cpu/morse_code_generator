#![cfg_attr(
all(
target_os = "windows",
not(feature = "console"),
),
windows_subsystem = "windows"
)]
mod pcm;

use std::sync::{Arc, Mutex};
use fltk::{app, prelude::*, text, window::Window};
use fltk::button::Button;
use rodio::{OutputStream, Sink, Source};
use crate::pcm::MorseCodePCM;

fn main() {
    let pcm: Arc<Mutex<Option<MorseCodePCM>>> = Arc::new(Mutex::new(None));
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::new(100, 100, 400, 190, "Morse Code Generator");
    let mut input_editor = text::TextEditor::new(5, 5, 390, 100, "");
    input_editor.set_buffer(text::TextBuffer::default());
    let mut generate_button = Button::new(5, 110, 390, 30, "Generate");
    let mut play_button = Button::new(5, 145, 390 / 2 - 5, 30, "Play");
    let mut save_button = Button::new(390 / 2 + 5, 145, 390 / 2, 30, "Save");
    let pcm1 = Arc::clone(&pcm);
    generate_button.set_callback(move |_| {
        let input = input_editor.buffer().expect("Error!").text();
        let mut pcm = pcm1.lock().unwrap();
        *pcm = Some(MorseCodePCM::from_text(&input, 100));
    });
    let pcm2 = Arc::clone(&pcm);
    save_button.set_callback(move |_| {
        let pcm = pcm2.lock().unwrap();
        if let Some(pcm) = &*pcm {
            let filename = fltk::dialog::file_chooser("Save to", "*.wav", ".", true).unwrap();
            pcm.clone().save_to_file(filename.as_str());
        } else {
            fltk::dialog::alert(200, 200, "Please generate first!");
        }
    });
    let pcm3 = Arc::clone(&pcm);
    play_button.set_callback(move |_| {
        let pcm = pcm3.lock().unwrap();
        if let Some(pcm) = &*pcm {
            //pcm.index = 0;
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            sink.append(pcm.clone().amplify(0.20));
            sink.sleep_until_end();
        } else {
            fltk::dialog::alert(200, 200, "Please generate first!");
        }
    });
    wind.end();
    wind.show();
    app.run().unwrap();
}
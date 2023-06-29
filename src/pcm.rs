use std::time::Duration;
use hound::WavSpec;
use rodio::Source;

#[derive(Clone)]
pub struct MorseCodePCM {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub data: Vec<i16>,
    // ms
    pub unit_duration: u32,
    pub index: usize,
}

impl Iterator for MorseCodePCM {
    type Item = i16;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let result = self.data[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}


impl Source for MorseCodePCM {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.channels
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl MorseCodePCM {
    pub fn new(unit_duration: u32) -> Self {
        Self {
            sample_rate: 44100,
            channels: 1,
            bits_per_sample: 16,
            data: Vec::new(),
            unit_duration,
            index: 0,
        }
    }
    pub fn from_text(text: &str, unit_duration: u32) -> Self {
        const FREQUENCY: u32 = 1500;
        let mut pcm = Self::new(unit_duration);
        for action in small_morse::encode(text) {
            let mut is_up = true;
            if action.state == small_morse::State::On {
                for i in 0..pcm.sample_rate / 1000 * pcm.unit_duration * action.duration as u32 {
                    if (i % (pcm.sample_rate / FREQUENCY)) == 0 {
                        is_up = !is_up;
                    }
                    pcm.data.push(if is_up { i16::MAX } else { i16::MIN });
                }
            } else {
                for _ in 0..pcm.sample_rate / 1000 * pcm.unit_duration {
                    pcm.data.push(0);
                }
            }
        }
        pcm
    }

    pub fn save_to_file(self, filename: &str) {
        let spec = WavSpec {
            channels: self.channels,
            sample_rate: self.sample_rate,
            bits_per_sample: self.bits_per_sample,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(filename, spec).unwrap();
        for sample in &self.data {
            writer.write_sample(*sample).unwrap();
        }
    }
}
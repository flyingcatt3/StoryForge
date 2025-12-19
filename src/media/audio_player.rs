use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use anyhow::Result;

pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    music_sink: Option<Sink>,
    sound_sink: Option<Sink>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        
        Ok(Self {
            _stream: stream,
            stream_handle,
            music_sink: None,
            sound_sink: None,
        })
    }
    
    pub fn play_music(&mut self, path: &str, volume: f32) -> Result<()> {
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;
        
        let sink = Sink::try_new(&self.stream_handle)?;
        sink.set_volume(volume / 100.0);
        sink.append(source);
        
        self.music_sink = Some(sink);
        Ok(())
    }
    
    pub fn play_sound(&mut self, path: &str, volume: f32) -> Result<()> {
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;
        
        let sink = Sink::try_new(&self.stream_handle)?;
        sink.set_volume(volume / 100.0);
        sink.append(source);
        sink.detach();
        
        Ok(())
    }
    
    pub fn stop_music(&mut self) {
        if let Some(sink) = &self.music_sink {
            sink.stop();
        }
        self.music_sink = None;
    }
    
    pub fn pause_music(&self) {
        if let Some(sink) = &self.music_sink {
            sink.pause();
        }
    }
    
    pub fn resume_music(&self) {
        if let Some(sink) = &self.music_sink {
            sink.play();
        }
    }
    
    pub fn set_music_volume(&self, volume: f32) {
        if let Some(sink) = &self.music_sink {
            sink.set_volume(volume / 100.0);
        }
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        // Return a disabled audio player if initialization fails
        match Self::new() {
            Ok(player) => player,
            Err(_) => {
                log::warn!("Failed to initialize audio player, audio will be disabled");
                // Create a minimal working state
                let (stream, stream_handle) = OutputStream::try_default()
                    .unwrap_or_else(|_| panic!("Audio system unavailable"));
                Self {
                    _stream: stream,
                    stream_handle,
                    music_sink: None,
                    sound_sink: None,
                }
            }
        }
    }
}

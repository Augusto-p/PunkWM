
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
// use crate::utils::config::print_in_tty;
use std::io::BufReader;
use std::sync::{mpsc::Receiver, Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::localaudio::entity::LocalAudioCommand;
use crate::custom_event::{main_thread_notifier::MainThreadNotifier};
use crate::ipc::senders::panel::music::sender_panel_music_local_current_time_song;

pub fn audio_thread(notifier: MainThreadNotifier,rx: Receiver<LocalAudioCommand>) {
    let (_stream, handle) = OutputStream::try_default().unwrap();

    let accumulated_time = Arc::new(Mutex::new(0u128));
    let last_start_time = Arc::new(Mutex::new(0u128));
    let paused = Arc::new(Mutex::new(true));

    let acc_clone = Arc::clone(&accumulated_time);
    let last_clone = Arc::clone(&last_start_time);
    let paused_clone = Arc::clone(&paused);

    thread::spawn(move || loop {
        let acc = *acc_clone.lock().unwrap();
        let last = *last_clone.lock().unwrap();
        let is_paused = *paused_clone.lock().unwrap();

        if !is_paused {
            sender_panel_music_local_current_time_song((acc + (now() - last)) / 1000);
        }

        thread::sleep(Duration::from_millis(900));
    });

    let mut sink: Option<Sink> = None;
    let mut path_song: Option<String> = None;

    while let Ok(cmd) = rx.recv() {
        match cmd {
            LocalAudioCommand::Load(path) => {
                path_song = Some(path.clone());

                let file = File::open(path).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();

                let new_sink = Sink::try_new(&handle).unwrap();
                new_sink.append(source);
                new_sink.play();
                *last_start_time.lock().unwrap() = now();
                *paused.lock().unwrap() = false;
                *accumulated_time.lock().unwrap() = 0;

                sink = Some(new_sink);
            }

            LocalAudioCommand::Play() => {
                if let Some(s) = &sink {
                    if s.is_paused() {
                        s.play();
                        *last_start_time.lock().unwrap() = now();
                        *paused.lock().unwrap() = false;
                    }
                }
            }

            LocalAudioCommand::Pause() => {
                if let Some(s) = &sink {
                    if !s.is_paused() {
                        s.pause();

                        let mut acc = accumulated_time.lock().unwrap();
                        let last = *last_start_time.lock().unwrap();

                        *acc += now() - last;
                        *paused.lock().unwrap() = true;
                    }
                }
            }

            LocalAudioCommand::Reset() => {
                if let Some(path) = &path_song {
                    let notifier_clone = notifier.clone();
                    notifier_clone.send(LocalAudioCommand::Load(path.to_string()));
                }
            }
            LocalAudioCommand::Stop() => {
                if let Some(s) = &sink {
                    s.stop();
                }
            }
        }
    }
}

fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

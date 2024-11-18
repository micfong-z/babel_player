use chrono::Duration;
use eframe::egui;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::mpsc;

use crate::component::colors::MfColors;
use crate::icons::material_design_icons::MDI_CHECK;
use crate::init::*;
use crate::lyrics_editor::LyricsEditor;

use crate::lyrics::BabelLyrics;
use crate::lyrics_editor::json_lyrics_file_loader;

#[derive(PartialEq)]
enum PlayerState {
    Playing,
    Paused,
    Stopped,
}

type AudioDetails = (
    Option<String>,
    Option<String>,
    Option<usize>,
    Option<Duration>,
);

pub struct BabelPlayerApp {
    audio_details_tx: mpsc::Sender<AudioDetails>,
    audio_details_rx: mpsc::Receiver<AudioDetails>,

    audio_data_tx: mpsc::Sender<Vec<u8>>,
    audio_data_rx: mpsc::Receiver<Vec<u8>>,

    selected_file: Option<String>,
    file_name: Option<String>,
    file_size: Option<usize>,
    file_data: Option<Vec<u8>>,

    arc_loading_file: Arc<Mutex<bool>>,

    lyrics_editor: LyricsEditor,
    lyrics_details_tx: mpsc::Sender<(Option<String>, Option<String>)>,
    lyrics_details_rx: mpsc::Receiver<(Option<String>, Option<String>)>,
    lyrics_data_tx: mpsc::Sender<BabelLyrics>,
    lyrics_data_rx: mpsc::Receiver<BabelLyrics>,

    arc_loading_lyrics: Arc<Mutex<bool>>,
    lyrics: Option<BabelLyrics>,
    selected_lyrics_file: Option<String>,
    lyrics_file_name: Option<String>,

    _rodio_stream: OutputStream,
    _rodio_stream_handle: OutputStreamHandle,
    arc_rodio_sink: Arc<Mutex<Sink>>,

    total_duration: Option<Duration>,

    /// Timestamp of the player.
    ///
    /// This is equal to `player_offset` + (`current_instant` - `player_start_instant`).
    player_timestamp: Duration,

    /// The instant when the player started/resumed.
    player_start_instant: Option<Instant>,

    /// The offset of the timestamp of the player.
    ///
    /// This is used to calculate the timestamp after pausing and resuming.
    player_offset: Duration,

    player_state: PlayerState,

    show_main_lyrics_window: bool,
    show_captions_window: bool,
}

impl Default for BabelPlayerApp {
    fn default() -> Self {
        let (file_details_tx, file_details_rx) = mpsc::channel(32);
        let (file_data_tx, file_data_rx) = mpsc::channel(32);
        let (lyrics_details_tx, lyrics_details_rx) = mpsc::channel(32);
        let (lyrics_data_tx, lyrics_data_rx) = mpsc::channel(32);
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        BabelPlayerApp {
            audio_details_tx: file_details_tx,
            audio_details_rx: file_details_rx,
            audio_data_tx: file_data_tx,
            audio_data_rx: file_data_rx,
            selected_file: None,
            file_name: None,
            file_size: None,
            arc_loading_file: Arc::new(Mutex::new(false)),
            file_data: None,
            lyrics_editor: LyricsEditor::default(),
            lyrics_details_tx,
            lyrics_details_rx,
            lyrics_data_tx,
            lyrics_data_rx,
            arc_loading_lyrics: Arc::new(Mutex::new(false)),
            lyrics: None,
            selected_lyrics_file: None,
            lyrics_file_name: None,
            player_timestamp: Duration::zero(),
            player_start_instant: None,
            player_offset: Duration::zero(),
            player_state: PlayerState::Stopped,
            show_main_lyrics_window: false,
            show_captions_window: false,
            _rodio_stream: stream,
            _rodio_stream_handle: stream_handle,
            arc_rodio_sink: Arc::new(Mutex::new(sink)),
            total_duration: None,
        }
    }
}

impl BabelPlayerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        setup_custom_styles(&cc.egui_ctx);

        Default::default()
    }
}

impl eframe::App for BabelPlayerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Babel Player")
            .collapsible(true)
            .resizable(true)
            .show(ctx, |ui| {
                let loading_file = *self.arc_loading_file.lock().unwrap();
                ui.add_enabled_ui(!loading_file, |ui| {
                    self.show_audio_file_picker(ui, loading_file);
                });
                if let Ok((selected_file, file_name, file_size, total_duration)) =
                    self.audio_details_rx.try_recv()
                {
                    *self.arc_loading_file.lock().unwrap() = false;
                    self.selected_file = selected_file;
                    self.file_name = file_name;
                    self.file_size = file_size;
                    self.total_duration = total_duration;
                }

                if let Ok(file_data) = self.audio_data_rx.try_recv() {
                    self.file_data = Some(file_data);
                }

                ui.separator();

                self.show_audio_file_details_grid(ui);

                ui.separator();

                ui.horizontal(|ui| {
                    ui.toggle_value(&mut self.lyrics_editor.show_lyrics_editor, "Lyrics editor");
                    ui.add_enabled_ui(self.lyrics_editor.lyrics.is_some(), |ui| {
                        if ui.button("Load from editor").clicked() {
                            let lyrics = self.lyrics_editor.lyrics.clone().unwrap();
                            let lyrics_data_tx = self.lyrics_data_tx.clone();
                            let lyrics_details_tx = self.lyrics_details_tx.clone();
                            tokio::spawn(async move {
                                let _ = lyrics_data_tx.send(lyrics).await;
                                let _ = lyrics_details_tx
                                    .send((
                                        Some("From editor".to_string()),
                                        Some("From editor".to_string()),
                                    ))
                                    .await;
                            });
                        }
                    });
                });

                let loading_file = *self.arc_loading_lyrics.lock().unwrap();
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(!loading_file, |ui| {
                        if ui.button("Select lyrics file").clicked() {
                            let details_tx = self.lyrics_details_tx.clone();
                            let data_tx = self.lyrics_data_tx.clone();
                            let arc_loading_lyrics = self.arc_loading_lyrics.clone();
                            tokio::spawn(async move {
                                json_lyrics_file_loader(arc_loading_lyrics, details_tx, data_tx)
                                    .await;
                            });
                        }
                    });

                    if loading_file {
                        ui.spinner();
                    } else if let Some(ref selected_lyrics_file) = self.selected_lyrics_file {
                        ui.label(selected_lyrics_file);
                    }
                });

                if let Ok((selected_lyrics_file, lyrics_file_name)) =
                    self.lyrics_details_rx.try_recv()
                {
                    *self.arc_loading_lyrics.lock().unwrap() = false;
                    self.selected_lyrics_file = selected_lyrics_file;
                    self.lyrics_file_name = lyrics_file_name;
                }

                if let Ok(lyrics) = self.lyrics_data_rx.try_recv() {
                    self.lyrics = Some(lyrics);
                    self.show_main_lyrics_window = true;
                    self.show_captions_window = true;
                }

                ui.separator();

                self.show_lyrics_file_details_grid(ui);

                if self.lyrics.is_some() {
                    ui.checkbox(&mut self.show_main_lyrics_window, "Main lyrics window");
                    ui.checkbox(&mut self.show_captions_window, "Captions window");
                }

                ui.separator();

                ui.horizontal(|ui| {
                    let original_timestamp = self.player_timestamp.num_milliseconds();
                    let mut timestamp_ms = self.player_timestamp.num_milliseconds();
                    ui.add(
                        egui::DragValue::new(&mut timestamp_ms)
                            .speed(100.0)
                            .range(
                                0.0..=self
                                    .total_duration
                                    .map(|d| d.num_milliseconds() as f64)
                                    .unwrap_or(i64::MAX as f64),
                            )
                            .custom_formatter(|n, _| {
                                format!(
                                    "{}:{:02}:{:02}.{:03}",
                                    n as i64 / 3_600_000,
                                    (n as i64 / 60_000) % 60,
                                    (n as i64 / 1_000) % 60,
                                    n as i64 % 1_000
                                )
                            }),
                    );
                    self.player_timestamp = Duration::milliseconds(timestamp_ms);

                    if self.player_state == PlayerState::Playing {
                        self.player_offset +=
                            Duration::milliseconds(timestamp_ms - original_timestamp);
                    } else if self.player_state == PlayerState::Paused
                        || self.player_state == PlayerState::Stopped
                    {
                        self.player_offset = self.player_timestamp;
                    }

                    ui.colored_label(MfColors::GRAY_500, "/");
                    if self.total_duration.is_some() {
                        ui.label(format!(
                            "{}:{:02}:{:02}.{:03}",
                            self.total_duration.unwrap().num_hours(),
                            self.total_duration.unwrap().num_minutes() % 60,
                            self.total_duration.unwrap().num_seconds() % 60,
                            self.total_duration.unwrap().num_milliseconds() % 1000
                        ));
                    } else {
                        ui.label("???");
                    }
                });

                match self.player_state {
                    PlayerState::Stopped => {
                        if ui.button("Play").clicked() {
                            self.player_state = PlayerState::Playing;
                            self.player_start_instant = Some(Instant::now());

                            let _ = self
                                .arc_rodio_sink
                                .lock()
                                .unwrap()
                                .try_seek(self.player_timestamp.to_std().unwrap());
                            self.arc_rodio_sink.lock().unwrap().play();
                        }
                    }
                    PlayerState::Paused => {
                        ui.horizontal(|ui| {
                            if ui.button("Resume").clicked() {
                                self.player_state = PlayerState::Playing;
                                self.player_start_instant = Some(Instant::now());
                                let _ = self
                                    .arc_rodio_sink
                                    .lock()
                                    .unwrap()
                                    .try_seek(self.player_timestamp.to_std().unwrap());
                                self.arc_rodio_sink.lock().unwrap().play();
                            }

                            if ui.button("Reset").clicked() {
                                self.player_state = PlayerState::Stopped;
                                self.player_timestamp = Duration::zero();
                                self.player_offset = Duration::zero();
                                self.player_start_instant = None;
                                self.arc_rodio_sink.lock().unwrap().pause();
                                let _ = self
                                    .arc_rodio_sink
                                    .lock()
                                    .unwrap()
                                    .try_seek(std::time::Duration::from_secs(0));
                            }
                        });
                    }
                    PlayerState::Playing => {
                        self.player_timestamp = self.player_offset
                            + Duration::milliseconds(
                                self.player_start_instant
                                    .map(|start_instant| {
                                        (Instant::now() - start_instant).as_millis() as i64
                                    })
                                    .unwrap_or(0),
                            );

                        ui.horizontal(|ui| {
                            if ui.button("Pause").clicked() {
                                self.player_state = PlayerState::Paused;
                                self.player_offset = self.player_timestamp;
                                self.arc_rodio_sink.lock().unwrap().pause();
                            }

                            if ui.button("Reset").clicked() {
                                self.player_state = PlayerState::Stopped;
                                self.player_timestamp = Duration::zero();
                                self.player_offset = Duration::zero();
                                self.player_start_instant = None;
                                self.arc_rodio_sink.lock().unwrap().pause();
                                let _ = self
                                    .arc_rodio_sink
                                    .lock()
                                    .unwrap()
                                    .try_seek(std::time::Duration::from_secs(0));
                            }
                        });

                        ctx.request_repaint();
                    }
                }
            });

        if self.show_main_lyrics_window {
            self.show_lyrics_window(ctx, self.lyrics.as_ref().unwrap());
        }
        if self.lyrics_editor.show_lyrics_editor {
            self.lyrics_editor.show_lyrics_editor_window(ctx).unwrap();
        }
        if self.show_captions_window {
            egui::Window::new("Captions")
                .title_bar(false)
                .show(ctx, |ui| {
                    ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
                    for line in &self.lyrics.as_ref().unwrap().lyrics.lines {
                        let current_time = self.player_timestamp;
                        if current_time > line.begin && current_time < line.end {
                            let mut current_translations_index_vec = Vec::new();
                            ui.horizontal(|ui| {
                                for segment in &line.original {
                                    if current_time > segment.begin && current_time < segment.end {
                                        ui.colored_label(MfColors::ORANGE_500, &segment.text);
                                        current_translations_index_vec
                                            .extend(segment.translations.clone());
                                    } else {
                                        ui.label(&segment.text);
                                    }
                                }
                            });
                            for (id, words) in &line.translations {
                                let language_translations_index_vec =
                                    current_translations_index_vec
                                        .iter()
                                        .filter_map(|(translation_id, word_index_list)| {
                                            if translation_id == id {
                                                Some(word_index_list)
                                            } else {
                                                None
                                            }
                                        })
                                        .flatten()
                                        .copied()
                                        .collect::<Vec<usize>>();
                                if !words.is_empty() {
                                    ui.horizontal(|ui| {
                                        for (index, word) in words.iter().enumerate() {
                                            if language_translations_index_vec.contains(&index) {
                                                ui.colored_label(MfColors::ORANGE_500, word);
                                            } else {
                                                ui.colored_label(MfColors::GRAY_500, word);
                                            }
                                        }
                                    });
                                }
                            }
                        }
                    }
                });
        }
    }
}

impl BabelPlayerApp {
    fn show_audio_file_picker(&mut self, ui: &mut egui::Ui, loading_file: bool) {
        ui.horizontal(|ui| {
            if ui.button("Select Audio File").clicked() {
                let details_tx = self.audio_details_tx.clone();
                let data_tx = self.audio_data_tx.clone();
                let arc_loading_file = self.arc_loading_file.clone();
                let arc_sink = self.arc_rodio_sink.clone();
                tokio::spawn(async move {
                    audio_file_loader(arc_loading_file, details_tx, data_tx, arc_sink).await;
                });
            }
            if loading_file {
                ui.spinner();
            } else if let Some(ref selected_file) = self.selected_file {
                ui.label(selected_file);
            }
        });
    }

    fn show_audio_file_details_grid(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("audio_file_details_grid").show(ui, |ui| {
            ui.label("File name");
            ui.label(self.file_name.as_deref().unwrap_or("-"));
            ui.end_row();

            ui.label("File size");
            ui.label(
                self.file_size
                    .map(|file_size| format!("{:.2} MiB", file_size as f64 / (1024.0 * 1024.0)))
                    .unwrap_or("-".to_string()),
            );

            ui.end_row();

            ui.label("File data");
            ui.label(if self.file_data.is_some() {
                format!("{} In memory", MDI_CHECK)
            } else {
                "-".to_string()
            });
            ui.end_row();
        });
    }

    fn show_lyrics_file_details_grid(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("lyrics_details_grid").show(ui, |ui| {
            ui.label("File name");
            ui.label(self.lyrics_file_name.as_deref().unwrap_or("-"));
            ui.end_row();

            ui.label("Lyrics data");
            ui.label(if self.lyrics.is_some() {
                format!("{} In memory", MDI_CHECK)
            } else {
                "-".to_string()
            });
            ui.end_row();
        });
    }

    fn show_lyrics_window(&self, ctx: &egui::Context, lyrics: &BabelLyrics) {
        egui::Window::new("Lyrics").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
                for line in &lyrics.lyrics.lines {
                    let current_time = self.player_timestamp;
                    if current_time > line.begin && current_time < line.end {
                        let mut current_translations_index_vec = Vec::new();
                        ui.horizontal(|ui| {
                            for segment in &line.original {
                                if current_time > segment.begin && current_time < segment.end {
                                    ui.colored_label(MfColors::ORANGE_500, &segment.text);
                                    current_translations_index_vec
                                        .extend(segment.translations.clone());
                                } else {
                                    ui.label(&segment.text);
                                }
                            }
                        });
                        for (id, words) in &line.translations {
                            let language_translations_index_vec = current_translations_index_vec
                                .iter()
                                .filter_map(|(translation_id, word_index_list)| {
                                    if translation_id == id {
                                        Some(word_index_list)
                                    } else {
                                        None
                                    }
                                })
                                .flatten()
                                .copied()
                                .collect::<Vec<usize>>();
                            if !words.is_empty() {
                                ui.horizontal(|ui| {
                                    for (index, word) in words.iter().enumerate() {
                                        if language_translations_index_vec.contains(&index) {
                                            ui.colored_label(MfColors::ORANGE_500, word);
                                        } else {
                                            ui.colored_label(MfColors::GRAY_500, word);
                                        }
                                    }
                                });
                            }
                        }
                    } else {
                        ui.horizontal(|ui| {
                            for segment in &line.original {
                                ui.colored_label(MfColors::GRAY_700, &segment.text);
                            }
                        });
                    }
                }
            });
        });
    }
}

async fn audio_file_loader(
    arc_loading_file: Arc<Mutex<bool>>,
    details_tx: mpsc::Sender<AudioDetails>,
    data_tx: mpsc::Sender<Vec<u8>>,
    arc_sink: Arc<Mutex<Sink>>,
) {
    let file = rfd::FileDialog::new()
        .add_filter("Audio Files", &["mp3"])
        .pick_file();

    if let Some(path) = file {
        *arc_loading_file.lock().unwrap() = true;
        let path_str = path.to_string_lossy().to_string();
        let file_name_str = path.file_name().unwrap().to_string_lossy().to_string();
        match tokio::fs::read(&path_str).await {
            Ok(data) => {
                let _ = data_tx.send(data.clone()).await;
                let len = data.len();

                let source = Decoder::new(std::io::Cursor::new(data)).unwrap();

                let _ = details_tx
                    .send((
                        Some(path_str),
                        Some(file_name_str),
                        Some(len),
                        source
                            .total_duration()
                            .map(|d| Duration::from_std(d).unwrap()),
                    ))
                    .await;
                arc_sink.lock().unwrap().append(source);
                arc_sink.lock().unwrap().pause();
            }
            Err(e) => {
                eprintln!("Failed to read file: {}", e);
            }
        }
    }
}

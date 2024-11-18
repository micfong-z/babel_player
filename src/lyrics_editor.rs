use crate::component::colors::MfColors;
use crate::icons;
use crate::icons::material_design_icons::MDI_CHECK;
use crate::lyrics::{
    BabelLyrics, Lyrics, LyricsLine, LyricsMetadata, LyricsSegment, TranslationEntry,
};
use amll_lyric::ttml;
use amll_lyric::ttml::TTMLLyric;
use chrono::Duration;
use eframe::egui;
use eframe::egui::RichText;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct LyricsEditor {
    pub show_lyrics_editor: bool,

    lyrics_details_tx: mpsc::Sender<(Option<String>, Option<String>)>,
    lyrics_details_rx: mpsc::Receiver<(Option<String>, Option<String>)>,

    lyrics_data_tx: mpsc::Sender<BabelLyrics>,
    lyrics_data_rx: mpsc::Receiver<BabelLyrics>,

    pub arc_loading_file: Arc<Mutex<bool>>,
    pub lyrics: Option<BabelLyrics>,

    selected_file: Option<String>,
    file_name: Option<String>,
}

impl Default for LyricsEditor {
    fn default() -> Self {
        let (lyrics_details_tx, lyrics_details_rx) = mpsc::channel(32);
        let (lyrics_data_tx, lyrics_data_rx) = mpsc::channel(32);

        LyricsEditor {
            show_lyrics_editor: false,
            lyrics_details_tx,
            lyrics_details_rx,
            lyrics_data_tx,
            lyrics_data_rx,
            arc_loading_file: Arc::new(Mutex::new(false)),
            lyrics: None,
            selected_file: None,
            file_name: None,
        }
    }
}

impl LyricsEditor {
    pub fn show_lyrics_editor_window(&mut self, ctx: &egui::Context) -> anyhow::Result<()> {
        egui::Window::new("Lyrics Editor").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let loading_lyrics_file = *self.arc_loading_file.lock().unwrap();
                ui.add_enabled_ui(!loading_lyrics_file, |ui| {
                    if ui
                        .button("Import AMLL TTML")
                        .on_hover_ui(|ui| {
                            ui.label("Import AMLL TTML file to edit lyrics.");
                            ui.horizontal(|ui| {
                                ui.label("You may wish to check");
                                ui.hyperlink_to(
                                    RichText::new("[ AMLL TTML Tool ]").color(MfColors::BLUE_300),
                                    "https://steve-xmh.github.io/amll-ttml-tool/",
                                );
                                ui.label("to create a word-by-word lyrics file first.");
                            });
                        })
                        .clicked()
                    {
                        let details_tx = self.lyrics_details_tx.clone();
                        let data_tx = self.lyrics_data_tx.clone();
                        let arc_loading_file = self.arc_loading_file.clone();
                        tokio::spawn(async move {
                            ttml_lyrics_file_loader(arc_loading_file, details_tx, data_tx).await;
                        });
                    }
                    if ui.button("Select lyrics file").clicked() {
                        let details_tx = self.lyrics_details_tx.clone();
                        let data_tx = self.lyrics_data_tx.clone();
                        let arc_loading_file = self.arc_loading_file.clone();
                        tokio::spawn(async move {
                            json_lyrics_file_loader(arc_loading_file, details_tx, data_tx).await;
                        });
                    }
                });
                if loading_lyrics_file {
                    ui.spinner();
                } else if let Some(ref selected_file) = self.selected_file {
                    ui.label(selected_file);
                }
            });
            ui.add_enabled_ui(self.lyrics.is_some(), |ui| {
                if ui.button("Export Babel Lyrics").clicked() {
                    let lyrics = self.lyrics.clone();
                    tokio::spawn(async move {
                        let file = rfd::FileDialog::new()
                            .add_filter("JSON", &["json"])
                            .save_file();
                        if let Some(path) = file {
                            let f = std::fs::File::create(&path).unwrap();
                            let mut writer = std::io::BufWriter::new(f);
                            let _ = serde_json::to_writer(&mut writer, lyrics.as_ref().unwrap());
                        }
                    });
                }
            });
            if let Ok((selected_file, file_name)) = self.lyrics_details_rx.try_recv() {
                *self.arc_loading_file.lock().unwrap() = false;
                self.selected_file = selected_file;
                self.file_name = file_name;
            }

            if let Ok(lyrics_data) = self.lyrics_data_rx.try_recv() {
                self.lyrics = Some(lyrics_data);
            }

            ui.separator();
            self.show_lyrics_file_details_grid(ui);
            ui.separator();
            if self.lyrics.is_some() {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // A map of translation language id to its name in metadata.
                    let translation_language_map: std::collections::HashMap<Uuid, String> = self
                        .lyrics
                        .as_ref()
                        .unwrap()
                        .metadata
                        .translations
                        .iter()
                        .map(|entry| (entry.id, entry.language.clone()))
                        .collect();

                    self.show_translation_languages_list(ui);

                    ui.separator();

                    let empty_translations_usize: Vec<(Uuid, Vec<usize>)> = self
                        .lyrics
                        .as_ref()
                        .unwrap()
                        .metadata
                        .translations
                        .iter()
                        .map(|entry| (entry.id, Vec::new()))
                        .collect();

                    let empty_translations_string: Vec<(Uuid, Vec<String>)> = self
                        .lyrics
                        .as_ref()
                        .unwrap()
                        .metadata
                        .translations
                        .iter()
                        .map(|entry| (entry.id, Vec::new()))
                        .collect();

                    self.show_lyrics_lines(ui, translation_language_map, empty_translations_usize);

                    if ui.button("+ Add Line").clicked() {
                        self.lyrics.as_mut().unwrap().lyrics.lines.push(LyricsLine {
                            begin: Duration::zero(),
                            end: Duration::zero(),
                            agent_id: String::new(),
                            original: Vec::new(),
                            translations: empty_translations_string.clone(),
                            uuid: Uuid::new_v4(),
                        });
                    }
                });
            }
        });
        Ok(())
    }

    fn show_lyrics_lines(
        &mut self,
        ui: &mut egui::Ui,
        translation_language_map: std::collections::HashMap<Uuid, String>,
        empty_translations_usize: Vec<(Uuid, Vec<usize>)>,
    ) {
        for line in self.lyrics.as_mut().unwrap().lyrics.lines.iter_mut() {
            egui::CollapsingHeader::new(
                line.original
                    .iter()
                    .map(|seg| seg.text.as_str())
                    .collect::<String>(),
            )
            .id_source(line.uuid)
            .default_open(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Agent");
                    ui.text_edit_singleline(&mut line.agent_id);
                });
                show_line_translations(ui, line, &translation_language_map);
                ui.separator();
                show_segment_edit_grid(line, ui, empty_translations_usize.clone());
            });
        }
    }

    fn show_translation_languages_list(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Translations", |ui| {
            let mut to_remove = Vec::<Uuid>::new();
            for language in &mut self.lyrics.as_mut().unwrap().metadata.translations {
                ui.horizontal(|ui| {
                    if ui
                        .button(icons::material_design_icons::MDI_DELETE)
                        .clicked()
                    {
                        to_remove.push(language.id);
                    }
                    ui.text_edit_singleline(&mut language.language);
                    ui.label(language.id.to_string())
                });
            }

            if ui.button("Add Language").clicked() {
                let new_id = Uuid::new_v4();
                self.lyrics
                    .as_mut()
                    .unwrap()
                    .metadata
                    .translations
                    .push(TranslationEntry {
                        language: String::new(),
                        id: new_id,
                    });
                for line in self.lyrics.as_mut().unwrap().lyrics.lines.iter_mut() {
                    line.translations.push((new_id, Vec::new()));
                    for segment in line.original.iter_mut() {
                        segment.translations.push((new_id, Vec::new()));
                    }
                }
            }

            for id in to_remove.iter() {
                self.lyrics
                    .as_mut()
                    .unwrap()
                    .metadata
                    .translations
                    .retain(|x| &x.id != id);
                for line in self.lyrics.as_mut().unwrap().lyrics.lines.iter_mut() {
                    line.translations.retain(|x| &x.0 != id);
                    for segment in line.original.iter_mut() {
                        segment.translations.retain(|x| &x.0 != id);
                    }
                }
            }
        });
    }

    fn show_lyrics_file_details_grid(&self, ui: &mut egui::Ui) {
        egui::Grid::new("lyrics_editor_file_details_grid").show(ui, |ui| {
            ui.label("File name");
            ui.label(self.file_name.as_deref().unwrap_or("-"));
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
}

fn show_line_translations(
    ui: &mut egui::Ui,
    line: &mut LyricsLine,
    translation_language_map: &std::collections::HashMap<Uuid, String>,
) {
    ui.collapsing("Translation", |ui| {
        for line_translation_pair in &mut line.translations {
            let translation_language = translation_language_map
                .get(&line_translation_pair.0)
                .unwrap_or(&"".to_string())
                .clone();
            egui::CollapsingHeader::new(translation_language)
                .id_source(format!("{}_{}", line.uuid, line_translation_pair.0))
                .default_open(false)
                .show(ui, |ui| {
                    egui::Grid::new(format!("translation_grid_{}", line_translation_pair.0))
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("(Original)");
                            let mut to_delete = Vec::<usize>::new();
                            for (index, translation_word) in
                                line_translation_pair.1.iter_mut().enumerate()
                            {
                                ui.horizontal(|ui| {
                                    if ui
                                        .button(icons::material_design_icons::MDI_DELETE)
                                        .clicked()
                                    {
                                        to_delete.push(index);
                                    }
                                    if translation_word == " " {
                                        ui.label(
                                            RichText::new("(space)").color(MfColors::GRAY_500),
                                        );
                                    } else {
                                        ui.add(egui::TextEdit::singleline(translation_word));
                                    }
                                });
                            }
                            for index in to_delete.iter().rev() {
                                line_translation_pair.1.remove(*index);
                                for segment in line.original.iter_mut() {
                                    for (lang_id, segment_translation_index) in
                                        segment.translations.iter_mut()
                                    {
                                        if lang_id == &line_translation_pair.0 {
                                            segment_translation_index.retain(|x| x != index);
                                            for x in segment_translation_index.iter_mut() {
                                                if *x > *index {
                                                    *x -= 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if ui.button("+").clicked() {
                                line_translation_pair.1.push(String::new());
                            }
                            ui.end_row();
                            for segment in line.original.iter_mut() {
                                if segment.text == " " {
                                    ui.label(RichText::new("(space)").color(MfColors::GRAY_500));
                                } else {
                                    ui.label(segment.text.as_str());
                                }
                                for (word_index, _) in
                                    line_translation_pair.1.iter_mut().enumerate()
                                {
                                    let (_, segment_translation_index) = segment
                                        .translations
                                        .iter_mut()
                                        .find(|(id, _)| id == &line_translation_pair.0)
                                        .unwrap();
                                    let is_in_translation =
                                        segment_translation_index.contains(&word_index);
                                    let mut change_to = is_in_translation;
                                    ui.checkbox(&mut change_to, "");
                                    if change_to != is_in_translation {
                                        if change_to {
                                            segment_translation_index.push(word_index);
                                        } else {
                                            segment_translation_index.retain(|x| x != &word_index);
                                        }
                                    }
                                }
                                ui.end_row();
                            }
                        });
                });
        }
    });
}

fn show_segment_edit_grid(
    line: &mut LyricsLine,
    ui: &mut egui::Ui,
    empty_translations_usize: Vec<(Uuid, Vec<usize>)>,
) {
    let mut to_remove = Vec::<usize>::new();
    let mut to_insert = Vec::<usize>::new();
    let mut to_move = Vec::<(usize, usize)>::new();
    egui::Grid::new(format!("grid_{}", line.uuid)).show(ui, |ui| {
        let mut size = ui.spacing().interact_size;
        size.x = 200.0;
        ui.label("Options");
        ui.label("Start");
        ui.label("End");
        ui.label("Text");
        ui.end_row();
        let word_count = line.original.len();
        for (index, seg) in line.original.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                if ui
                    .button(icons::material_design_icons::MDI_DELETE)
                    .clicked()
                {
                    to_remove.push(index);
                }
                if ui.button(icons::material_design_icons::MDI_PLUS).clicked() {
                    to_insert.push(index + 1);
                }
                if index != 0
                    && ui
                        .button(icons::material_design_icons::MDI_ARROW_UP)
                        .clicked()
                {
                    to_move.push((index, index - 1));
                }
                if index != word_count - 1
                    && ui
                        .button(icons::material_design_icons::MDI_ARROW_DOWN)
                        .clicked()
                {
                    to_move.push((index, index + 1));
                }
            });
            ui.horizontal(|ui| {
                let mut minutes = seg.begin.num_minutes();
                let mut seconds = (seg.begin.num_seconds() % 60) as u32;
                let mut milliseconds = (seg.begin.num_milliseconds() % 1000) as u32;
                ui.add(
                    egui::DragValue::new(&mut minutes)
                        .speed(1)
                        .range(0..=59)
                        .suffix("m"),
                );
                ui.add(
                    egui::DragValue::new(&mut seconds)
                        .speed(1)
                        .range(0..=59)
                        .suffix("s"),
                );
                ui.add(
                    egui::DragValue::new(&mut milliseconds)
                        .speed(1)
                        .range(0..=999),
                );

                seg.begin = Duration::milliseconds(
                    minutes * 60 * 1000 + seconds as i64 * 1000 + milliseconds as i64,
                );
            });
            ui.horizontal(|ui| {
                let mut minutes = seg.end.num_minutes();
                let mut seconds = (seg.end.num_seconds() % 60) as u32;
                let mut milliseconds = (seg.end.num_milliseconds() % 1000) as u32;
                ui.add(
                    egui::DragValue::new(&mut minutes)
                        .speed(1)
                        .range(0..=59)
                        .suffix("m"),
                );
                ui.add(
                    egui::DragValue::new(&mut seconds)
                        .speed(1)
                        .range(0..=59)
                        .suffix("s"),
                );
                ui.add(
                    egui::DragValue::new(&mut milliseconds)
                        .speed(1)
                        .range(0..=999),
                );

                seg.end = Duration::milliseconds(
                    minutes * 60 * 1000 + seconds as i64 * 1000 + milliseconds as i64,
                );
            });
            if seg.text == " " {
                ui.label(RichText::new("(space)").color(MfColors::GRAY_500));
            } else {
                ui.add_sized(size, |ui: &mut egui::Ui| {
                    ui.text_edit_singleline(&mut seg.text)
                });
            }
            ui.end_row();
        }
    });
    for index in to_remove.iter().rev() {
        line.original.remove(*index);
    }
    for index in to_insert.iter() {
        line.original.insert(
            *index,
            LyricsSegment {
                begin: Duration::zero(),
                end: Duration::zero(),
                text: String::new(),
                translations: empty_translations_usize.clone(),
            },
        );
    }
    for (from, to) in to_move.iter() {
        line.original.swap(*from, *to);
    }
}

async fn ttml_lyrics_file_loader(
    arc_loading_file: Arc<Mutex<bool>>,
    details_tx: mpsc::Sender<(Option<String>, Option<String>)>,
    data_tx: mpsc::Sender<BabelLyrics>,
) {
    let file = rfd::FileDialog::new()
        .add_filter("TTML Lyrics", &["ttml"])
        .pick_file();

    if let Some(path) = file {
        *arc_loading_file.lock().unwrap() = true;
        let path_str = path.to_string_lossy().to_string();
        let file_name_str = path.file_name().unwrap().to_string_lossy().to_string();
        match std::fs::File::open(&path) {
            Ok(file) => {
                let _ = details_tx.send((Some(path_str), Some(file_name_str))).await;
                let f = std::io::BufReader::new(file);
                match ttml::parse_ttml(f) {
                    Ok(ttml_lyrics) => {
                        let babel_lyrics = parse_ttml_lyrics(ttml_lyrics);
                        let _ = data_tx.send(babel_lyrics).await;
                    }
                    Err(e) => {
                        eprintln!("Failed to parse ttml: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
            }
        }
    }
}

pub async fn json_lyrics_file_loader(
    arc_loading_file: Arc<Mutex<bool>>,
    details_tx: mpsc::Sender<(Option<String>, Option<String>)>,
    data_tx: mpsc::Sender<BabelLyrics>,
) {
    let file = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .pick_file();

    if let Some(path) = file {
        *arc_loading_file.lock().unwrap() = true;
        let path_str = path.to_string_lossy().to_string();
        let file_name_str = path.file_name().unwrap().to_string_lossy().to_string();
        match std::fs::File::open(&path) {
            Ok(file) => {
                let _ = details_tx.send((Some(path_str), Some(file_name_str))).await;
                let f = std::io::BufReader::new(file);
                match serde_json::from_reader(f) {
                    Ok(babel_lyrics) => {
                        let _ = data_tx.send(babel_lyrics).await;
                    }
                    Err(e) => {
                        eprintln!("Failed to parse json: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
            }
        }
    }
}

fn parse_ttml_lyrics(ttml_lyrics: TTMLLyric) -> BabelLyrics {
    let lines = ttml_lyrics.lines;
    let mut babel_lines = Vec::<LyricsLine>::new();
    for line in lines {
        let mut babel_segments = Vec::<LyricsSegment>::new();
        for segment in line.words {
            let babel_segment = LyricsSegment {
                begin: Duration::milliseconds(segment.start_time as i64),
                end: Duration::milliseconds(segment.end_time as i64),
                text: segment.word.to_string(),
                translations: Vec::new(),
            };
            babel_segments.push(babel_segment);
        }
        let babel_line = LyricsLine {
            begin: Duration::milliseconds(line.start_time as i64),
            end: Duration::milliseconds(line.end_time as i64),
            agent_id: String::new(),
            original: babel_segments,
            translations: Vec::new(),
            uuid: Uuid::new_v4(),
        };
        babel_lines.push(babel_line);
    }
    BabelLyrics {
        metadata: LyricsMetadata {
            agents: Vec::new(),
            translations: Vec::new(),
        },
        lyrics: Lyrics { lines: babel_lines },
    }
}

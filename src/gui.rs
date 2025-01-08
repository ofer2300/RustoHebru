use iced::{
    button, scrollable, text_input, Alignment, Button, Column, Container, Element,
    Length, Row, Scrollable, Settings, Text, TextInput, Application, Command,
    window, executor, Subscription, Event, keyboard,
};
use iced_native::{Event as NativeEvent, event};
use std::path::PathBuf;
use std::collections::HashMap;
use crate::file_processor::{FileProcessor, ProcessedFile, FileType};
use crate::translation::{TranslationEngine, TranslationRequest, TranslationResult};
use crate::file_saver::{FileSaver, SaveOptions};
use crate::theme::{self, Theme, ColorPalette};
use crate::icons::Icon;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum Message {
    // אירועי קובץ
    FilesDropped(Vec<PathBuf>),
    FileProcessed(PathBuf, Result<ProcessedFile>),
    FileRemoved(PathBuf),
    ClearFiles,
    
    // אירועי תרגום
    TranslationStarted,
    TranslationComplete(PathBuf, Result<TranslationResult>),
    TranslationCancelled,
    EditTranslation(PathBuf, String),
    ApplyEdit(PathBuf),
    
    // אירועי שמירה
    SaveTranslation,
    SaveOptionsChanged(SaveOptions),
    SaveComplete(Result<SaveResult>),
    SelectOutputDir,
    OutputDirSelected(PathBuf),
    
    // אירועי ממשק
    ThemeChanged(Theme),
    LanguageChanged(Language),
    ZoomChanged(f32),
    Search(String),
    ToggleHelp,
    ToggleSettings,
    Undo,
    Redo,
    
    // אירועי מקלדת
    KeyPressed(keyboard::KeyCode),
    
    // אירועי מערכת
    Error(String),
}

pub struct TranslatorGui {
    // מצב הקבצים
    file_processor: FileProcessor,
    translation_engine: TranslationEngine,
    file_saver: FileSaver,
    processed_files: HashMap<PathBuf, ProcessedFile>,
    translation_results: HashMap<PathBuf, TranslationResult>,
    
    // מצב ממשק
    theme: Theme,
    language: Language,
    zoom_level: f32,
    show_help: bool,
    show_settings: bool,
    search_query: String,
    
    // מצב עריכה
    editing_file: Option<PathBuf>,
    edit_content: String,
    undo_stack: Vec<EditState>,
    redo_stack: Vec<EditState>,
    
    // מצב שמירה
    save_options: SaveOptions,
    
    // מצב UI
    drop_zone_state: DropZoneState,
    preview_scroll: scrollable::State,
    help_scroll: scrollable::State,
    settings_scroll: scrollable::State,
    search_input: text_input::State,
    save_button: button::State,
    clear_button: button::State,
    help_button: button::State,
    settings_button: button::State,
    error_message: Option<String>,
}

#[derive(Debug, Clone)]
struct EditState {
    file: PathBuf,
    content: String,
    cursor_position: usize,
}

#[derive(Debug, Clone)]
enum DropZoneState {
    Idle,
    DragOver,
    Processing,
}

impl Application for TranslatorGui {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                file_processor: FileProcessor::new(),
                translation_engine: TranslationEngine::new(),
                file_saver: FileSaver::new(),
                processed_files: HashMap::new(),
                translation_results: HashMap::new(),
                theme: Theme::default(),
                language: Language::Hebrew,
                zoom_level: 1.0,
                show_help: false,
                show_settings: false,
                search_query: String::new(),
                editing_file: None,
                edit_content: String::new(),
                undo_stack: Vec::new(),
                redo_stack: Vec::new(),
                save_options: SaveOptions::default(),
                drop_zone_state: DropZoneState::Idle,
                preview_scroll: scrollable::State::new(),
                help_scroll: scrollable::State::new(),
                settings_scroll: scrollable::State::new(),
                search_input: text_input::State::new(),
                save_button: button::State::new(),
                clear_button: button::State::new(),
                help_button: button::State::new(),
                settings_button: button::State::new(),
                error_message: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("מתרגם מסמכים טכניים")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FilesDropped(paths) => {
                self.drop_zone_state = DropZoneState::Processing;
                self.error_message = None;
                
                let mut commands = Vec::new();
                for path in paths {
                    let file_processor = self.file_processor.clone();
                    let path_clone = path.clone();
                    commands.push(Command::perform(
                        async move { file_processor.process_file(path_clone) },
                        move |result| Message::FileProcessed(path, result),
                    ));
                }
                Command::batch(commands)
            }
            
            Message::FileProcessed(path, result) => {
                match result {
                    Ok(processed_file) => {
                        self.processed_files.insert(path.clone(), processed_file.clone());
                        let translation_request = TranslationRequest::new(
                            processed_file.content,
                            self.language,
                        );
                        let translation_engine = self.translation_engine.clone();
                        Command::perform(
                            async move { translation_engine.translate(translation_request) },
                            move |result| Message::TranslationComplete(path, result),
                        )
                    }
                    Err(e) => {
                        self.error_message = Some(e.to_string());
                        self.drop_zone_state = DropZoneState::Idle;
                        Command::none()
                    }
                }
            }
            
            Message::FileRemoved(path) => {
                self.processed_files.remove(&path);
                self.translation_results.remove(&path);
                if self.editing_file == Some(path.clone()) {
                    self.editing_file = None;
                    self.edit_content.clear();
                }
                Command::none()
            }
            
            Message::ClearFiles => {
                self.processed_files.clear();
                self.translation_results.clear();
                self.editing_file = None;
                self.edit_content.clear();
                self.undo_stack.clear();
                self.redo_stack.clear();
                Command::none()
            }
            
            Message::TranslationStarted => {
                self.drop_zone_state = DropZoneState::Processing;
                Command::none()
            }
            
            Message::TranslationComplete(path, result) => {
                match result {
                    Ok(translation) => {
                        self.translation_results.insert(path, translation);
                        if self.processed_files.len() == self.translation_results.len() {
                            self.drop_zone_state = DropZoneState::Idle;
                        }
                        Command::none()
                    }
                    Err(e) => {
                        self.error_message = Some(e.to_string());
                        Command::none()
                    }
                }
            }
            
            Message::TranslationCancelled => {
                self.drop_zone_state = DropZoneState::Idle;
                Command::none()
            }
            
            Message::EditTranslation(path, content) => {
                if self.editing_file != Some(path.clone()) {
                    self.undo_stack.push(EditState {
                        file: path.clone(),
                        content: self.edit_content.clone(),
                        cursor_position: 0,
                    });
                }
                self.editing_file = Some(path);
                self.edit_content = content;
                Command::none()
            }
            
            Message::ApplyEdit(path) => {
                if let Some(translation) = self.translation_results.get_mut(&path) {
                    translation.apply_edit(self.edit_content.clone());
                }
                self.editing_file = None;
                self.edit_content.clear();
                Command::none()
            }
            
            Message::SaveTranslation => {
                let files_to_save: Vec<_> = self.translation_results
                    .iter()
                    .map(|(path, translation)| {
                        (
                            path.clone(),
                            self.processed_files.get(path).unwrap().clone(),
                            translation.clone(),
                        )
                    })
                    .collect();
                    
                let file_saver = self.file_saver.clone();
                let save_options = self.save_options.clone();
                
                Command::perform(
                    async move {
                        let mut results = Vec::new();
                        for (path, processed_file, translation) in files_to_save {
                            results.push(file_saver.save_translated_file(
                                &processed_file,
                                &translation.translated_text,
                                save_options.clone(),
                            ));
                        }
                        results
                    },
                    Message::SaveComplete,
                )
            }
            
            Message::SaveOptionsChanged(options) => {
                self.save_options = options;
                Command::none()
            }
            
            Message::SaveComplete(result) => {
                match result {
                    Ok(_) => {
                        self.error_message = None;
                        Command::none()
                    }
                    Err(e) => {
                        self.error_message = Some(e.to_string());
                        Command::none()
                    }
                }
            }
            
            Message::SelectOutputDir => {
                Command::perform(
                    async {
                        rfd::FileDialog::new()
                            .set_directory(".")
                            .pick_folder()
                    },
                    |result| {
                        result.map_or(Message::Error("לא נבחרה תיקייה".into()), Message::OutputDirSelected)
                    },
                )
            }
            
            Message::OutputDirSelected(path) => {
                self.save_options.output_dir = Some(path);
                Command::none()
            }
            
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                Command::none()
            }
            
            Message::LanguageChanged(language) => {
                self.language = language;
                Command::none()
            }
            
            Message::ZoomChanged(zoom) => {
                self.zoom_level = zoom;
                Command::none()
            }
            
            Message::Search(query) => {
                self.search_query = query;
                Command::none()
            }
            
            Message::ToggleHelp => {
                self.show_help = !self.show_help;
                Command::none()
            }
            
            Message::ToggleSettings => {
                self.show_settings = !self.show_settings;
                Command::none()
            }
            
            Message::Undo => {
                if let Some(state) = self.undo_stack.pop() {
                    self.redo_stack.push(EditState {
                        file: state.file.clone(),
                        content: self.edit_content.clone(),
                        cursor_position: 0,
                    });
                    self.editing_file = Some(state.file);
                    self.edit_content = state.content;
                }
                Command::none()
            }
            
            Message::Redo => {
                if let Some(state) = self.redo_stack.pop() {
                    self.undo_stack.push(EditState {
                        file: state.file.clone(),
                        content: self.edit_content.clone(),
                        cursor_position: 0,
                    });
                    self.editing_file = Some(state.file);
                    self.edit_content = state.content;
                }
                Command::none()
            }
            
            Message::KeyPressed(key_code) => {
                match key_code {
                    keyboard::KeyCode::Z if keyboard::Modifiers::CTRL.is_pressed() => {
                        self.update(Message::Undo)
                    }
                    keyboard::KeyCode::Y if keyboard::Modifiers::CTRL.is_pressed() => {
                        self.update(Message::Redo)
                    }
                    keyboard::KeyCode::S if keyboard::Modifiers::CTRL.is_pressed() => {
                        self.update(Message::SaveTranslation)
                    }
                    _ => Command::none(),
                }
            }
            
            Message::Error(error) => {
                self.error_message = Some(error);
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut content = Column::new().spacing(20).padding(20);
        
        // סרגל כלים
        content = content.push(
            Row::new()
                .spacing(10)
                .push(
                    Button::new(
                        &mut self.clear_button,
                        Row::new()
                            .spacing(5)
                            .push(Icon::new("trash"))
                            .push(Text::new("נקה הכל")),
                    )
                    .on_press(Message::ClearFiles)
                    .style(theme::Button::Destructive),
                )
                .push(
                    Button::new(
                        &mut self.help_button,
                        Row::new()
                            .spacing(5)
                            .push(Icon::new("question-circle"))
                            .push(Text::new("עזרה")),
                    )
                    .on_press(Message::ToggleHelp),
                )
                .push(
                    Button::new(
                        &mut self.settings_button,
                        Row::new()
                            .spacing(5)
                            .push(Icon::new("cog"))
                            .push(Text::new("הגדרות")),
                    )
                    .on_press(Message::ToggleSettings),
                ),
        );

        // אזור גרירת קבצים
        let drop_zone = Container::new(
            Column::new()
                .spacing(10)
                .push(Icon::new("cloud-upload-alt").size(48))
                .push(Text::new(match self.drop_zone_state {
                    DropZoneState::Idle => "גרור קבצים לכאן",
                    DropZoneState::DragOver => "שחרר את הקבצים",
                    DropZoneState::Processing => "מעבד...",
                }).size(24))
                .push(Text::new("תומך ב: PDF, Word, Excel, PowerPoint, HTML, CSV").size(14)),
        )
        .width(Length::Fill)
        .height(Length::Units(200))
        .center_x()
        .center_y()
        .style(match self.drop_zone_state {
            DropZoneState::Idle => theme::Container::Default,
            DropZoneState::DragOver => theme::Container::Highlighted,
            DropZoneState::Processing => theme::Container::Processing,
        });

        content = content.push(drop_zone);

        // תצוגה מקדימה של קבצים
        if !self.processed_files.is_empty() {
            let mut files_preview = Scrollable::new(&mut self.preview_scroll)
                .height(Length::Units(400))
                .spacing(10);

            for (path, processed_file) in &self.processed_files {
                let mut file_row = Row::new().spacing(10);
                
                // סוג הקובץ
                file_row = file_row.push(
                    Container::new(match processed_file.file_type {
                        FileType::PDF => Icon::new("file-pdf"),
                        FileType::Word => Icon::new("file-word"),
                        FileType::Excel => Icon::new("file-excel"),
                        FileType::PowerPoint => Icon::new("file-powerpoint"),
                        FileType::HTML => Icon::new("file-code"),
                        FileType::CSV => Icon::new("file-csv"),
                        FileType::Text => Icon::new("file-alt"),
                    })
                    .style(theme::Container::FileType),
                );
                
                // שם הקובץ
                file_row = file_row.push(
                    Text::new(&processed_file.metadata.file_name)
                        .size(16)
                        .width(Length::Fill),
                );
                
                // כפתורי פעולה
                file_row = file_row.push(
                    Row::new()
                        .spacing(5)
                        .push(
                            Button::new(
                                &mut button::State::new(),
                                Icon::new("edit"),
                            )
                            .on_press(Message::EditTranslation(
                                path.clone(),
                                self.translation_results
                                    .get(path)
                                    .map(|t| t.translated_text.clone())
                                    .unwrap_or_default(),
                            )),
                        )
                        .push(
                            Button::new(
                                &mut button::State::new(),
                                Icon::new("trash"),
                            )
                            .on_press(Message::FileRemoved(path.clone()))
                            .style(theme::Button::Destructive),
                        ),
                );

                files_preview = files_preview.push(file_row);

                // תצוגת תרגום
                if let Some(translation) = self.translation_results.get(path) {
                    let translation_content = if Some(path) == self.editing_file {
                        TextInput::new(
                            &mut text_input::State::new(),
                            "ערוך תרגום...",
                            &self.edit_content,
                            |content| Message::EditTranslation(path.clone(), content),
                        )
                        .padding(10)
                        .size(16)
                    } else {
                        TextInput::new(
                            &mut text_input::State::new(),
                            "",
                            &translation.translated_text,
                            |_| Message::EditTranslation(path.clone(), translation.translated_text.clone()),
                        )
                        .padding(10)
                        .size(16)
                        .style(theme::TextInput::ReadOnly)
                    };

                    files_preview = files_preview.push(
                        Column::new()
                            .push(Text::new("תרגום:").size(14))
                            .push(translation_content)
                            .padding(10),
                    );
                }
            }

            content = content.push(files_preview);
        }

        // הודעות שגיאה
        if let Some(error) = &self.error_message {
            content = content.push(
                Container::new(
                    Text::new(error)
                        .color(self.theme.colors().error),
                )
                .padding(10)
                .style(theme::Container::Error),
            );
        }

        // כפתור שמירה
        if !self.translation_results.is_empty() {
            content = content.push(
                Button::new(
                    &mut self.save_button,
                    Row::new()
                        .spacing(5)
                        .push(Icon::new("save"))
                        .push(Text::new("שמור תרגומים")),
                )
                .on_press(Message::SaveTranslation)
                .width(Length::Units(200))
                .style(theme::Button::Primary),
            );
        }

        // חלון עזרה
        if self.show_help {
            content = content.push(
                Container::new(
                    Scrollable::new(&mut self.help_scroll)
                        .push(
                            Column::new()
                                .spacing(10)
                                .push(Text::new("עזרה").size(24))
                                .push(Text::new("קיצורי מקלדת:"))
                                .push(Text::new("Ctrl+Z - בטל"))
                                .push(Text::new("Ctrl+Y - בצע שוב"))
                                .push(Text::new("Ctrl+S - שמור")),
                        ),
                )
                .padding(20)
                .style(theme::Container::Card),
            );
        }

        // חלון הגדרות
        if self.show_settings {
            content = content.push(
                Container::new(
                    Scrollable::new(&mut self.settings_scroll)
                        .push(
                            Column::new()
                                .spacing(10)
                                .push(Text::new("הגדרות").size(24))
                                .push(
                                    Row::new()
                                        .spacing(10)
                                        .push(Text::new("ערכת נושא:"))
                                        .push(
                                            Button::new(
                                                &mut button::State::new(),
                                                Text::new(if self.theme.is_dark() {
                                                    "בהיר"
                                                } else {
                                                    "כהה"
                                                }),
                                            )
                                            .on_press(Message::ThemeChanged(
                                                if self.theme.is_dark() {
                                                    Theme::Light
                                                } else {
                                                    Theme::Dark
                                                },
                                            )),
                                        ),
                                )
                                .push(
                                    Row::new()
                                        .spacing(10)
                                        .push(Text::new("שפה:"))
                                        .push(
                                            Button::new(
                                                &mut button::State::new(),
                                                Text::new(match self.language {
                                                    Language::Hebrew => "עברית",
                                                    Language::Russian => "רוסית",
                                                }),
                                            )
                                            .on_press(Message::LanguageChanged(
                                                if self.language == Language::Hebrew {
                                                    Language::Russian
                                                } else {
                                                    Language::Hebrew
                                                },
                                            )),
                                        ),
                                ),
                        ),
                )
                .padding(20)
                .style(theme::Container::Card),
            );
        }

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .style(theme::Container::Background)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            event::listen().map(|event| match event {
                NativeEvent::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                    Message::KeyPressed(key_code)
                }
                _ => Message::Error("אירוע לא ידוע".into()),
            }),
        ])
    }
} 
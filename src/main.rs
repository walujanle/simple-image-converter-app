//! Simple Image Converter App - Entry point and application logic.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod constants;
mod convert;
mod handlers;
mod heic;
mod message;
mod settings;
mod state;
mod theme;
mod view;

use crate::convert::{convert_image, get_target_filename};
use crate::message::Message;
use crate::state::{AppState, FileStatus};
use crate::view::view;
use iced::{executor, Application, Command, Element, Settings, Subscription, Theme};

#[cfg(not(debug_assertions))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
use std::io::Write;
use std::path::PathBuf;

/// Application entry point.
pub fn main() -> iced::Result {
    ImageConverterApp::run(Settings::default())
}

struct ImageConverterApp {
    state: AppState,
}

impl Application for ImageConverterApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    /// Initializes application with saved settings.
    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut state = AppState::default();
        state.options = settings::load_settings();
        (ImageConverterApp { state }, Command::none())
    }

    /// Returns window title.
    fn title(&self) -> String {
        String::from("Simple Image Converter App")
    }

    /// Returns current theme based on dark mode setting.
    fn theme(&self) -> Theme {
        if self.state.options.is_dark_mode {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    /// Routes messages to appropriate handlers.
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::WindowResized => Command::none(),
            Message::DarkThemeToggled(v) => handlers::handle_dark_theme(&mut self.state, v),
            Message::AddFilesClicked => {
                let dialog = rfd::AsyncFileDialog::new()
                    .add_filter("Images", &["jpg", "jpeg", "png", "webp", "heic", "heif"]);
                Command::perform(async move { dialog.pick_files().await }, |files_opt| {
                    Message::FilesSelected(
                        files_opt
                            .map(|h| h.into_iter().map(|f| f.path().to_path_buf()).collect())
                            .unwrap_or_default(),
                    )
                })
            }
            Message::FilesSelected(paths) => {
                handlers::handle_files_selected(&mut self.state, paths)
            }
            Message::ExternalFilesDropped(paths) => {
                handlers::handle_external_files(&mut self.state, paths)
            }
            Message::ItemDragStarted(i) => handlers::handle_item_drag_started(&mut self.state, i),
            Message::ItemDropped => handlers::handle_item_dropped(&mut self.state),
            Message::ItemHovered(i) => handlers::handle_item_hovered(&mut self.state, i),
            Message::ToggleSelection(i) => handlers::handle_toggle_selection(&mut self.state, i),
            Message::DeleteSelected => handlers::handle_delete_selected(&mut self.state),
            Message::ClearList => handlers::handle_clear_list(&mut self.state),
            Message::FormatSelected(f) => handlers::handle_format_selected(&mut self.state, f),
            Message::QualityChanged(q) => handlers::handle_quality_changed(&mut self.state, q),
            Message::QualityInputChanged(v) => handlers::handle_quality_input(&mut self.state, v),
            Message::PngCompressionToggled(v) => {
                handlers::handle_png_compression(&mut self.state, v)
            }
            Message::ResizeToggled(v) => handlers::handle_resize_toggled(&mut self.state, v),
            Message::WidthChanged(v) => handlers::handle_width_changed(&mut self.state, v),
            Message::HeightChanged(v) => handlers::handle_height_changed(&mut self.state, v),
            Message::PrefixChanged(v) => handlers::handle_prefix_changed(&mut self.state, v),
            Message::FindPatternChanged(v) => handlers::handle_find_pattern(&mut self.state, v),
            Message::ReplaceWithChanged(v) => handlers::handle_replace_with(&mut self.state, v),
            Message::AutoSuffixToggled(v) => handlers::handle_auto_suffix(&mut self.state, v),
            Message::ToggleCustomOutput(v) => handlers::handle_custom_output(&mut self.state, v),
            Message::BrowseOutputClicked => {
                let dialog = rfd::AsyncFileDialog::new();
                Command::perform(async move { dialog.pick_folder().await }, |h| {
                    Message::OutputFolderSelected(h.map(|f| f.path().to_path_buf()))
                })
            }
            Message::OutputFolderSelected(p) => {
                handlers::handle_output_selected(&mut self.state, p)
            }
            Message::ToggleKeepMetadata(v) => handlers::handle_keep_metadata(&mut self.state, v),
            Message::ToggleGenerateLog(v) => handlers::handle_generate_log(&mut self.state, v),
            Message::AddNumberingToggled(v) => handlers::handle_add_numbering(&mut self.state, v),
            Message::ManualGenerateLogClicked => {
                self.generate_log_file();
                Command::none()
            }
            Message::ConvertClicked => self.start_conversion(),
            Message::OverwriteDecision(proceed) => self.process_conversion(proceed),
            Message::FileConverted(id, res) => {
                handlers::handle_file_converted(&mut self.state, id, res)
            }
            Message::ConversionFinished => {
                let cmd = handlers::handle_conversion_finished(&mut self.state);
                if self.state.options.generate_log {
                    self.generate_log_file();
                }
                cmd
            }
        }
    }

    /// Renders the UI.
    fn view(&self) -> Element<'_, Message> {
        view(&self.state)
    }

    /// Subscribes to window events for drag-drop and keyboard.
    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(|event| match event {
            iced::Event::Window(_, iced::window::Event::FileDropped(path)) => {
                Message::ExternalFilesDropped(vec![path])
            }
            iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)) => {
                Message::ItemDropped
            }
            iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Delete),
                ..
            }) => Message::DeleteSelected,
            _ => Message::WindowResized,
        })
    }
}

impl ImageConverterApp {
    /// Checks for file collisions and prompts user before conversion.
    fn start_conversion(&self) -> Command<Message> {
        let mut collision_count = 0;
        for file_item in &self.state.files {
            let target_name = get_target_filename(&file_item.path, &self.state.options);
            let parent = if self.state.options.use_custom_output {
                self.state
                    .options
                    .custom_output_path
                    .as_ref()
                    .filter(|p| p.exists())
                    .map(|p| p.as_path())
                    .unwrap_or_else(|| file_item.path.parent().unwrap_or(std::path::Path::new(".")))
            } else {
                file_item.path.parent().unwrap_or(std::path::Path::new("."))
            };
            if parent.join(&target_name).exists() {
                collision_count += 1;
            }
        }

        if collision_count > 0 {
            Command::perform(
                async move {
                    rfd::AsyncMessageDialog::new()
                        .set_title("Confirm Overwrite")
                        .set_description(&format!(
                            "{} files already exist. Overwrite?",
                            collision_count
                        ))
                        .set_buttons(rfd::MessageButtons::YesNo)
                        .show()
                        .await
                },
                |res| Message::OverwriteDecision(res == rfd::MessageDialogResult::Yes),
            )
        } else {
            Command::perform(async {}, |_| Message::OverwriteDecision(true))
        }
    }

    /// Spawns async conversion tasks for all files.
    fn process_conversion(&mut self, proceed: bool) -> Command<Message> {
        if !proceed {
            return Command::none();
        }

        self.state.is_processing = true;
        for file in &mut self.state.files {
            file.status = FileStatus::Processing;
        }

        let options = self.state.options.clone();
        let batch_size = self
            .state
            .options
            .max_batch_size
            .min(self.state.files.len());

        let commands: Vec<Command<Message>> = self
            .state
            .files
            .iter()
            .take(batch_size)
            .map(|file| {
                let id = file.id;
                let path = file.path.clone();
                let opts = options.clone();
                Command::perform(
                    async move {
                        let res = tokio::task::spawn_blocking(move || convert_image(&path, &opts))
                            .await
                            .expect("Task panicked");
                        (id, res.map_err(|e| e.to_string()))
                    },
                    |(id, res)| Message::FileConverted(id, res),
                )
            })
            .collect();

        Command::batch(commands)
    }

    /// Generates dataset log file with converted filenames.
    fn generate_log_file(&self) {
        let target_dir = if self.state.options.use_custom_output {
            self.state
                .options
                .custom_output_path
                .clone()
                .unwrap_or_else(|| PathBuf::from("."))
        } else {
            PathBuf::from(".")
        };

        if let Ok(mut file) = std::fs::File::create(target_dir.join("dataset_log.txt")) {
            for (i, file_item) in self.state.files.iter().enumerate() {
                let target_name = get_target_filename(&file_item.path, &self.state.options);
                let line = if self.state.options.add_numbering {
                    format!("{}. {}", i + 1, target_name)
                } else {
                    target_name
                };
                let _ = writeln!(file, "{}", line);
            }
        }
    }
}

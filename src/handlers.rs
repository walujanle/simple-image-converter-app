//! Message handlers extracted from main.rs for cleaner architecture.

use crate::message::Message;
use crate::settings;
use crate::state::{AppState, FileItem, FileStatus};
use iced::Command;

/// Toggles dark mode theme and saves preference.
pub fn handle_dark_theme(state: &mut AppState, is_dark: bool) -> Command<Message> {
    state.options.is_dark_mode = is_dark;
    settings::save_settings(&state.options);
    Command::none()
}

/// Adds selected files to the conversion queue.
pub fn handle_files_selected(
    state: &mut AppState,
    paths: Vec<std::path::PathBuf>,
) -> Command<Message> {
    for path in paths {
        if !state.files.iter().any(|f| f.path == path) {
            state.files.push(FileItem::new(path));
        }
    }
    Command::none()
}

/// Handles files dropped from external file manager.
pub fn handle_external_files(
    state: &mut AppState,
    paths: Vec<std::path::PathBuf>,
) -> Command<Message> {
    for path in paths {
        if path.is_file() {
            state.files.push(FileItem::new(path));
        }
    }
    Command::none()
}

/// Initiates drag operation for file reordering.
pub fn handle_item_drag_started(state: &mut AppState, index: usize) -> Command<Message> {
    state.dragging_index = Some(index);
    Command::none()
}

/// Completes drag-drop file reordering.
pub fn handle_item_dropped(state: &mut AppState) -> Command<Message> {
    if let Some(from_index) = state.dragging_index {
        if let Some(to_index) = state.hovered_index {
            if from_index != to_index
                && from_index < state.files.len()
                && to_index < state.files.len()
            {
                let item = state.files.remove(from_index);
                state.files.insert(to_index, item);
                state.selected_indices.clear();
            }
        }
    }
    state.dragging_index = None;
    state.hovered_index = None;
    Command::none()
}

/// Updates hover target during drag operation.
pub fn handle_item_hovered(state: &mut AppState, index_opt: Option<usize>) -> Command<Message> {
    if state.dragging_index.is_some() {
        state.hovered_index = index_opt;
    }
    Command::none()
}

/// Toggles file selection state for batch operations.
pub fn handle_toggle_selection(state: &mut AppState, index: usize) -> Command<Message> {
    if state.selected_indices.contains(&index) {
        state.selected_indices.remove(&index);
    } else {
        state.selected_indices.insert(index);
    }
    Command::none()
}

/// Removes all selected files from the queue.
pub fn handle_delete_selected(state: &mut AppState) -> Command<Message> {
    let mut indices: Vec<usize> = state.selected_indices.iter().cloned().collect();
    indices.sort_by(|a, b| b.cmp(a));
    for idx in indices {
        if idx < state.files.len() {
            state.files.remove(idx);
        }
    }
    state.selected_indices.clear();
    Command::none()
}

/// Clears all files from the conversion queue.
pub fn handle_clear_list(state: &mut AppState) -> Command<Message> {
    state.files.clear();
    state.selected_indices.clear();
    Command::none()
}

/// Updates output format selection.
pub fn handle_format_selected(
    state: &mut AppState,
    format: crate::state::ImageFormat,
) -> Command<Message> {
    state.options.format = format;
    settings::save_settings(&state.options);
    Command::none()
}

/// Updates quality level from slider.
pub fn handle_quality_changed(state: &mut AppState, q: u8) -> Command<Message> {
    state.options.quality = q;
    settings::save_settings(&state.options);
    Command::none()
}

/// Updates quality level from text input.
pub fn handle_quality_input(state: &mut AppState, value: String) -> Command<Message> {
    if let Ok(num) = value.parse::<u8>() {
        state.options.quality = num.min(100);
        settings::save_settings(&state.options);
    }
    Command::none()
}

/// Toggles PNG compression optimization.
pub fn handle_png_compression(state: &mut AppState, v: bool) -> Command<Message> {
    state.options.png_compressed = v;
    settings::save_settings(&state.options);
    Command::none()
}

/// Toggles image resize option.
pub fn handle_resize_toggled(state: &mut AppState, v: bool) -> Command<Message> {
    state.options.resize = v;
    settings::save_settings(&state.options);
    Command::none()
}

/// Updates target resize width.
pub fn handle_width_changed(state: &mut AppState, v: String) -> Command<Message> {
    if v.chars().all(|c| c.is_numeric()) {
        state.options.target_width = v;
        settings::save_settings(&state.options);
    }
    Command::none()
}

/// Updates target resize height.
pub fn handle_height_changed(state: &mut AppState, v: String) -> Command<Message> {
    if v.chars().all(|c| c.is_numeric()) {
        state.options.target_height = v;
        settings::save_settings(&state.options);
    }
    Command::none()
}

/// Updates filename prefix.
pub fn handle_prefix_changed(state: &mut AppState, v: String) -> Command<Message> {
    state.options.prefix = v;
    settings::save_settings(&state.options);
    Command::none()
}

/// Updates find pattern for filename replacement.
pub fn handle_find_pattern(state: &mut AppState, v: String) -> Command<Message> {
    state.options.find_pattern = v;
    settings::save_settings(&state.options);
    Command::none()
}

/// Updates replacement string for filename pattern.
pub fn handle_replace_with(state: &mut AppState, v: String) -> Command<Message> {
    state.options.replace_with = v;
    settings::save_settings(&state.options);
    Command::none()
}

/// Toggles automatic resolution/quality suffix.
pub fn handle_auto_suffix(state: &mut AppState, v: bool) -> Command<Message> {
    state.options.auto_suffix = v;
    settings::save_settings(&state.options);
    Command::none()
}

/// Toggles custom output folder usage.
pub fn handle_custom_output(state: &mut AppState, v: bool) -> Command<Message> {
    state.options.use_custom_output = v;
    settings::save_settings(&state.options);
    Command::none()
}

/// Sets custom output folder path.
pub fn handle_output_selected(
    state: &mut AppState,
    path_opt: Option<std::path::PathBuf>,
) -> Command<Message> {
    if let Some(path) = path_opt {
        state.options.custom_output_path = Some(path);
        settings::save_settings(&state.options);
    }
    Command::none()
}

/// Toggles EXIF metadata preservation.
pub fn handle_keep_metadata(state: &mut AppState, v: bool) -> Command<Message> {
    state.options.keep_metadata = v;
    settings::save_settings(&state.options);
    Command::none()
}

/// Toggles dataset log file generation.
pub fn handle_generate_log(state: &mut AppState, v: bool) -> Command<Message> {
    state.options.generate_log = v;
    settings::save_settings(&state.options);
    Command::none()
}

/// Toggles numbering in log file entries.
pub fn handle_add_numbering(state: &mut AppState, v: bool) -> Command<Message> {
    state.options.add_numbering = v;
    settings::save_settings(&state.options);
    Command::none()
}

/// Processes file conversion result and updates status.
pub fn handle_file_converted(
    state: &mut AppState,
    id: uuid::Uuid,
    result: Result<(), String>,
) -> Command<Message> {
    if let Some(file) = state.files.iter_mut().find(|f| f.id == id) {
        match result {
            Ok(_) => file.status = FileStatus::Done,
            Err(e) => file.status = FileStatus::Error(e),
        }
    }
    if !state
        .files
        .iter()
        .any(|f| matches!(f.status, FileStatus::Processing))
    {
        state.is_processing = false;
        return Command::perform(async {}, |_| Message::ConversionFinished);
    }
    Command::none()
}

/// Finalizes conversion batch and triggers memory cleanup.
pub fn handle_conversion_finished(state: &mut AppState) -> Command<Message> {
    state.is_processing = false;
    settings::save_settings(&state.options);
    state.files.shrink_to_fit();
    state.selected_indices.shrink_to(0);

    #[cfg(not(debug_assertions))]
    unsafe {
        extern "C" {
            fn mi_collect(force: bool);
            fn mi_option_set(option: i32, value: i64);
        }
        mi_option_set(1, 0);
        mi_collect(true);
        mi_collect(true);
    }

    Command::none()
}

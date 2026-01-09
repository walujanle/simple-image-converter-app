//! UI components and layout for the image converter application.

use crate::message::Message;
use crate::state::{AppState, FileItem, FileStatus, ImageFormat};
use crate::theme::{colors, dark, dimensions, spacing, typography};
use iced::widget::{
    button, checkbox, column, container, horizontal_space, mouse_area, pick_list, row, scrollable,
    slider, text, text_input, vertical_space,
};
use iced::Length::Fixed;
use iced::{Background, Color, Element, Length, Theme};

/// Main view function rendering the entire UI.
pub fn view<'a>(state: &'a AppState) -> Element<'a, Message> {
    let is_dark = state.options.is_dark_mode;

    let (bg, _surface, border, txt, txt_secondary, primary, success, error, processing) = if is_dark
    {
        (
            dark::BACKGROUND,
            dark::SURFACE,
            dark::BORDER,
            dark::TEXT,
            dark::TEXT_SECONDARY,
            dark::PRIMARY,
            dark::SUCCESS,
            dark::ERROR,
            dark::PROCESSING,
        )
    } else {
        (
            colors::BACKGROUND,
            colors::SURFACE,
            colors::BORDER,
            colors::TEXT,
            colors::TEXT_SECONDARY,
            colors::PRIMARY,
            colors::SUCCESS,
            colors::ERROR,
            colors::PROCESSING,
        )
    };

    // Header section
    let header = container(
        row![
            text("Image Converter")
                .size(typography::TITLE)
                .style(iced::theme::Text::Color(txt)),
            horizontal_space(),
            checkbox("Dark Mode", state.options.is_dark_mode)
                .on_toggle(Message::DarkThemeToggled)
                .text_size(typography::BODY)
        ]
        .align_items(iced::Alignment::Center),
    )
    .padding([spacing::LG, spacing::XL])
    .width(Length::Fill);

    // Action bar with file selection and convert button
    let add_files_btn = button(text("+ Select Files").size(typography::BODY))
        .on_press(Message::AddFilesClicked)
        .padding([spacing::SM, spacing::LG])
        .style(iced::theme::Button::Primary);

    let convert_label = if state.is_processing {
        "Processing..."
    } else {
        "Start Conversion"
    };

    let convert_btn = button(
        container(text(convert_label).size(typography::BODY))
            .width(Length::Fill)
            .center_x(),
    )
    .on_press(Message::ConvertClicked)
    .padding([spacing::MD, spacing::XL])
    .width(Length::FillPortion(2))
    .style(iced::theme::Button::Primary);

    // Format and quality section
    let format_pick = pick_list(
        &ImageFormat::ALL[..],
        Some(state.options.format),
        Message::FormatSelected,
    )
    .placeholder("Format")
    .padding(spacing::SM);

    let quality_section: Element<'_, Message> = match state.options.format {
        ImageFormat::Png => row![checkbox("Optimize PNG", state.options.png_compressed)
            .on_toggle(Message::PngCompressionToggled)
            .text_size(typography::BODY)]
        .align_items(iced::Alignment::Center)
        .into(),
        _ => {
            let quality_str = state.options.quality.to_string();
            row![
                text("Quality")
                    .size(typography::BODY)
                    .style(iced::theme::Text::Color(txt_secondary)),
                slider(1..=100, state.options.quality, Message::QualityChanged).width(Fixed(140.0)),
                text_input("", &quality_str)
                    .on_input(Message::QualityInputChanged)
                    .width(Fixed(48.0))
                    .padding(spacing::XS)
            ]
            .spacing(spacing::SM)
            .align_items(iced::Alignment::Center)
            .into()
        }
    };

    let metadata_row: Element<'_, Message> = if matches!(state.options.format, ImageFormat::Jpeg) {
        checkbox("Keep Metadata", state.options.keep_metadata)
            .on_toggle(Message::ToggleKeepMetadata)
            .text_size(typography::BODY)
            .into()
    } else {
        horizontal_space().height(Fixed(0.0)).into()
    };

    let format_card = card(
        column![
            text("Output Settings")
                .size(typography::HEADING)
                .style(iced::theme::Text::Color(txt)),
            vertical_space().height(Fixed(spacing::SM as f32)),
            row![
                column![
                    text("Format")
                        .size(typography::CAPTION)
                        .style(iced::theme::Text::Color(txt_secondary)),
                    format_pick
                ]
                .spacing(spacing::XS),
                horizontal_space().width(Fixed(spacing::XL as f32)),
                quality_section
            ]
            .align_items(iced::Alignment::End),
            metadata_row
        ]
        .spacing(spacing::SM),
        is_dark,
    );

    // Filename options card
    let prefix_input = text_input("e.g., converted_", &state.options.prefix)
        .on_input(Message::PrefixChanged)
        .padding(spacing::SM);

    let find_input = text_input("Text to find...", &state.options.find_pattern)
        .on_input(Message::FindPatternChanged)
        .padding(spacing::SM);

    let replace_input = text_input("Replace with...", &state.options.replace_with)
        .on_input(Message::ReplaceWithChanged)
        .padding(spacing::SM);

    let filename_card = card(
        column![
            text("Filename Options")
                .size(typography::HEADING)
                .style(iced::theme::Text::Color(txt)),
            vertical_space().height(Fixed(spacing::XS as f32)),
            row![
                column![
                    text("Prefix")
                        .size(typography::CAPTION)
                        .style(iced::theme::Text::Color(txt_secondary)),
                    prefix_input
                ]
                .spacing(spacing::XXS)
                .width(Length::FillPortion(1)),
                column![
                    text("Find & Replace")
                        .size(typography::CAPTION)
                        .style(iced::theme::Text::Color(txt_secondary)),
                    row![
                        find_input,
                        text("->").style(iced::theme::Text::Color(txt_secondary)),
                        replace_input
                    ]
                    .spacing(spacing::XS)
                    .align_items(iced::Alignment::Center)
                ]
                .spacing(spacing::XXS)
                .width(Length::FillPortion(2))
            ]
            .spacing(spacing::LG),
            checkbox(
                "Auto Suffix (resolution + quality)",
                state.options.auto_suffix
            )
            .on_toggle(Message::AutoSuffixToggled)
            .text_size(typography::BODY)
        ]
        .spacing(spacing::SM),
        is_dark,
    );

    // Output and resize section
    let output_path_display = state
        .options
        .custom_output_path
        .as_ref()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "Same as input folder".to_string());

    let browse_btn = button(text("Browse").size(typography::CAPTION))
        .on_press(Message::BrowseOutputClicked)
        .padding([spacing::XS, spacing::SM])
        .style(iced::theme::Button::Secondary);

    let output_section = column![
        row![
            text("Output")
                .size(typography::HEADING)
                .style(iced::theme::Text::Color(txt)),
            horizontal_space(),
            checkbox("Custom folder", state.options.use_custom_output)
                .on_toggle(Message::ToggleCustomOutput)
                .text_size(typography::CAPTION)
        ],
        if state.options.use_custom_output {
            row![
                text_input("Select folder...", &output_path_display).padding(spacing::SM),
                browse_btn
            ]
            .spacing(spacing::SM)
        } else {
            row![container(
                text(&output_path_display)
                    .size(typography::CAPTION)
                    .style(iced::theme::Text::Color(txt_secondary))
            )
            .padding(spacing::SM)]
        }
    ]
    .spacing(spacing::SM);

    let width_input = text_input("W", &state.options.target_width)
        .on_input(Message::WidthChanged)
        .width(Fixed(60.0))
        .padding(spacing::XS);
    let height_input = text_input("H", &state.options.target_height)
        .on_input(Message::HeightChanged)
        .width(Fixed(60.0))
        .padding(spacing::XS);

    let resize_section = column![
        row![
            text("Resize")
                .size(typography::HEADING)
                .style(iced::theme::Text::Color(txt)),
            horizontal_space(),
            checkbox("Enable", state.options.resize)
                .on_toggle(Message::ResizeToggled)
                .text_size(typography::CAPTION)
        ],
        if state.options.resize {
            row![
                width_input,
                text("x").style(iced::theme::Text::Color(txt_secondary)),
                height_input
            ]
            .spacing(spacing::XS)
            .align_items(iced::Alignment::Center)
        } else {
            row![text("Original size")
                .size(typography::CAPTION)
                .style(iced::theme::Text::Color(txt_secondary))]
        }
    ]
    .spacing(spacing::SM);

    let settings_row = row![
        card(output_section, is_dark).width(Length::FillPortion(3)),
        card(resize_section, is_dark).width(Length::FillPortion(2))
    ]
    .spacing(spacing::LG);

    // Dataset options
    let gen_txt_btn = button(text("Generate").size(typography::CAPTION))
        .on_press(Message::ManualGenerateLogClicked)
        .padding([spacing::XS, spacing::SM])
        .style(iced::theme::Button::Secondary);

    let dataset_section = row![
        checkbox("Generate list file", state.options.generate_log)
            .on_toggle(Message::ToggleGenerateLog)
            .text_size(typography::BODY),
        checkbox("# Numbering", state.options.add_numbering)
            .on_toggle(Message::AddNumberingToggled)
            .text_size(typography::CAPTION),
        gen_txt_btn
    ]
    .spacing(spacing::LG)
    .align_items(iced::Alignment::Center);

    // File list section
    let file_count = state.files.len();
    let selected_count = state.selected_indices.len();
    let list_title = if selected_count > 0 {
        format!("Files ({} of {} selected)", selected_count, file_count)
    } else {
        format!("Files ({})", file_count)
    };

    let delete_btn = button(text("Delete").size(typography::CAPTION))
        .on_press(Message::DeleteSelected)
        .padding([spacing::XS, spacing::SM])
        .style(iced::theme::Button::Destructive);

    let clear_btn = button(text("Clear All").size(typography::CAPTION))
        .on_press(Message::ClearList)
        .padding([spacing::XS, spacing::SM])
        .style(iced::theme::Button::Secondary);

    let list_header = row![
        text(&list_title)
            .size(typography::HEADING)
            .style(iced::theme::Text::Color(txt)),
        horizontal_space(),
        delete_btn,
        clear_btn
    ]
    .spacing(spacing::SM)
    .align_items(iced::Alignment::Center);

    let file_list: Element<Message> = if state.files.is_empty() {
        container(
            column![text("Drop files here or click Select Files")
                .size(typography::BODY)
                .style(iced::theme::Text::Color(txt_secondary))]
            .spacing(spacing::SM)
            .align_items(iced::Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    } else {
        let items: Vec<Element<Message>> = state
            .files
            .iter()
            .enumerate()
            .map(|(i, f)| {
                file_item_view(
                    i,
                    f,
                    state.dragging_index,
                    state.hovered_index,
                    state.selected_indices.contains(&i),
                    is_dark,
                    success,
                    error,
                    processing,
                    txt,
                    txt_secondary,
                )
            })
            .collect();

        scrollable(column(items).spacing(spacing::XXS))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    };

    let list_card = container(file_list)
        .padding(spacing::SM)
        .width(Length::Fill)
        .height(Fixed(dimensions::FILE_LIST_HEIGHT))
        .style(card_style(is_dark));

    // Progress and status bar
    let done = state
        .files
        .iter()
        .filter(|f| matches!(f.status, FileStatus::Done | FileStatus::Error(_)))
        .count();
    let progress = if file_count > 0 {
        done as f32 / file_count as f32
    } else {
        0.0
    };

    let status_text = if state.is_processing {
        format!("Processing {} of {}...", done, file_count)
    } else if file_count == 0 {
        "Ready - add files to begin".to_string()
    } else {
        format!("{} files ready", file_count)
    };

    let is_processing = state.is_processing;
    let progress_bar = container(text(""))
        .width(Length::FillPortion((progress * 100.0).max(1.0) as u16))
        .height(Fixed(dimensions::PROGRESS_BAR_HEIGHT))
        .style(move |_: &Theme| container::Appearance {
            background: if is_processing {
                Some(Background::Color(primary))
            } else {
                None
            },
            border: iced::Border {
                radius: 3.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

    let progress_bg = container(progress_bar)
        .width(Length::Fill)
        .style(move |_: &Theme| container::Appearance {
            background: Some(Background::Color(border)),
            border: iced::Border {
                radius: 3.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

    let status_bar = row![
        progress_bg.width(Length::FillPortion(2)),
        text(&status_text)
            .size(typography::CAPTION)
            .style(iced::theme::Text::Color(txt_secondary))
    ]
    .spacing(spacing::LG)
    .align_items(iced::Alignment::Center)
    .padding([spacing::SM, 0]);

    // Main layout
    let main_content = column![
        header,
        container(
            column![
                row![add_files_btn, horizontal_space(), convert_btn]
                    .spacing(spacing::LG)
                    .align_items(iced::Alignment::Center),
                vertical_space().height(Fixed(spacing::LG as f32)),
                format_card,
                filename_card,
                settings_row,
                dataset_section,
                vertical_space().height(Fixed(spacing::SM as f32)),
                list_header,
                list_card,
                status_bar
            ]
            .spacing(spacing::MD)
        )
        .padding([0, spacing::XL, spacing::XL, spacing::XL])
    ]
    .spacing(0);

    container(scrollable(main_content))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_: &Theme| container::Appearance {
            background: Some(Background::Color(bg)),
            ..Default::default()
        })
        .into()
}

/// Creates a styled card container.
fn card<'a>(
    content: impl Into<Element<'a, Message>>,
    is_dark: bool,
) -> container::Container<'a, Message> {
    container(content)
        .padding(spacing::LG)
        .width(Length::Fill)
        .style(card_style(is_dark))
}

/// Card container style.
fn card_style(is_dark: bool) -> impl Fn(&Theme) -> container::Appearance {
    move |_: &Theme| {
        let (surface, border) = if is_dark {
            (dark::SURFACE_ELEVATED, dark::BORDER)
        } else {
            (colors::SURFACE, colors::BORDER)
        };
        container::Appearance {
            background: Some(Background::Color(surface)),
            border: iced::Border {
                color: border,
                width: 1.0,
                radius: dimensions::CARD_RADIUS.into(),
            },
            ..Default::default()
        }
    }
}

/// Renders individual file item in list.
fn file_item_view(
    index: usize,
    file: &FileItem,
    dragging: Option<usize>,
    hovered: Option<usize>,
    selected: bool,
    is_dark: bool,
    success: Color,
    error: Color,
    processing: Color,
    txt: Color,
    txt_secondary: Color,
) -> Element<'static, Message> {
    let is_dragging = dragging == Some(index);
    let is_hovered = hovered == Some(index);

    let status_el: Element<'static, Message> = match &file.status {
        FileStatus::Pending => text("").into(),
        FileStatus::Processing => text("...")
            .size(typography::BODY)
            .style(iced::theme::Text::Color(processing))
            .into(),
        FileStatus::Done => text("OK")
            .size(typography::BODY)
            .style(iced::theme::Text::Color(success))
            .into(),
        FileStatus::Error(e) => text(format!("ERR: {}", e.chars().take(20).collect::<String>()))
            .size(typography::CAPTION)
            .style(iced::theme::Text::Color(error))
            .into(),
    };

    let file_name = file.path.file_name().unwrap_or_default().to_string_lossy();

    let bg = if selected {
        if is_dark {
            Color::from_rgba(0.45, 0.56, 1.0, 0.15)
        } else {
            Color::from_rgba(0.35, 0.47, 0.98, 0.1)
        }
    } else if is_dragging {
        if is_dark {
            Color::from_rgba(1.0, 1.0, 1.0, 0.05)
        } else {
            Color::from_rgba(0.0, 0.0, 0.0, 0.03)
        }
    } else if is_hovered {
        if is_dark {
            Color::from_rgba(1.0, 1.0, 1.0, 0.03)
        } else {
            Color::from_rgba(0.0, 0.0, 0.0, 0.02)
        }
    } else {
        Color::TRANSPARENT
    };

    let drag_handle = button(
        text("::")
            .size(typography::CAPTION)
            .style(iced::theme::Text::Color(txt_secondary)),
    )
    .on_press(Message::ItemDragStarted(index))
    .padding([spacing::XS, spacing::SM])
    .style(iced::theme::Button::Text);

    let content = row![
        text(format!("{:02}", index + 1))
            .size(typography::CAPTION)
            .style(iced::theme::Text::Color(txt_secondary))
            .width(Fixed(24.0)),
        text(file_name.to_string())
            .size(typography::BODY)
            .style(iced::theme::Text::Color(txt))
            .width(Length::Fill),
        status_el
    ]
    .spacing(spacing::SM)
    .align_items(iced::Alignment::Center)
    .padding([spacing::XS, spacing::SM]);

    let item = container(
        mouse_area(content)
            .on_press(Message::ToggleSelection(index))
            .on_enter(Message::ItemHovered(Some(index))),
    )
    .style(move |_: &Theme| container::Appearance {
        background: Some(Background::Color(bg)),
        border: iced::Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .width(Length::Fill);

    row![drag_handle, item]
        .spacing(spacing::XXS)
        .align_items(iced::Alignment::Center)
        .into()
}

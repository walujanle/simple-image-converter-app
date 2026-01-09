//! Settings persistence using SQLite in platform-specific config directory.

use crate::state::{ConversionOptions, ImageFormat};
use rusqlite::{Connection, Result as SqlResult};
use std::path::PathBuf;

/// Returns cross-platform application config directory.
fn get_app_data_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| {
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.to_path_buf()))
                .unwrap_or_else(|| PathBuf::from("."))
        })
        .join("SimpleImageConverter")
}

/// Returns full path to settings database file.
fn get_db_path() -> PathBuf {
    let dir = get_app_data_dir();
    let _ = std::fs::create_dir_all(&dir);
    dir.join("settings.db")
}

/// Initializes database connection and creates schema.
pub fn init_db() -> SqlResult<Connection> {
    let conn = Connection::open(get_db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL)",
        [],
    )?;
    Ok(conn)
}

/// Loads all settings from database into ConversionOptions.
pub fn load_settings() -> ConversionOptions {
    let conn = match init_db() {
        Ok(c) => c,
        Err(_) => return ConversionOptions::default(),
    };

    let mut opts = ConversionOptions::default();

    if let Ok(v) = get_value(&conn, "format") {
        opts.format = match v.as_str() {
            "Png" => ImageFormat::Png,
            "WebP" => ImageFormat::WebP,
            _ => ImageFormat::Jpeg,
        };
    }
    if let Ok(v) = get_value(&conn, "quality") {
        opts.quality = v.parse().unwrap_or(80);
    }
    if let Ok(v) = get_value(&conn, "png_compressed") {
        opts.png_compressed = v == "true";
    }
    if let Ok(v) = get_value(&conn, "resize") {
        opts.resize = v == "true";
    }
    if let Ok(v) = get_value(&conn, "target_width") {
        opts.target_width = v;
    }
    if let Ok(v) = get_value(&conn, "target_height") {
        opts.target_height = v;
    }
    if let Ok(v) = get_value(&conn, "prefix") {
        opts.prefix = v;
    }
    if let Ok(v) = get_value(&conn, "auto_suffix") {
        opts.auto_suffix = v == "true";
    }
    if let Ok(v) = get_value(&conn, "keep_metadata") {
        opts.keep_metadata = v == "true";
    }
    if let Ok(v) = get_value(&conn, "use_custom_output") {
        opts.use_custom_output = v == "true";
    }
    if let Ok(v) = get_value(&conn, "custom_output_path") {
        if !v.is_empty() {
            opts.custom_output_path = Some(PathBuf::from(v));
        }
    }
    if let Ok(v) = get_value(&conn, "generate_log") {
        opts.generate_log = v == "true";
    }
    if let Ok(v) = get_value(&conn, "add_numbering") {
        opts.add_numbering = v == "true";
    }
    if let Ok(v) = get_value(&conn, "is_dark_mode") {
        opts.is_dark_mode = v == "true";
    }
    if let Ok(v) = get_value(&conn, "max_batch_size") {
        opts.max_batch_size = v.parse().unwrap_or(10);
    }

    opts
}

/// Saves all settings from ConversionOptions to database.
pub fn save_settings(opts: &ConversionOptions) {
    let conn = match init_db() {
        Ok(c) => c,
        Err(_) => return,
    };

    let format_str = match opts.format {
        ImageFormat::Jpeg => "Jpeg",
        ImageFormat::Png => "Png",
        ImageFormat::WebP => "WebP",
    };

    let _ = set_value(&conn, "format", format_str);
    let _ = set_value(&conn, "quality", &opts.quality.to_string());
    let _ = set_value(
        &conn,
        "png_compressed",
        if opts.png_compressed { "true" } else { "false" },
    );
    let _ = set_value(&conn, "resize", if opts.resize { "true" } else { "false" });
    let _ = set_value(&conn, "target_width", &opts.target_width);
    let _ = set_value(&conn, "target_height", &opts.target_height);
    let _ = set_value(&conn, "prefix", &opts.prefix);
    let _ = set_value(
        &conn,
        "auto_suffix",
        if opts.auto_suffix { "true" } else { "false" },
    );
    let _ = set_value(
        &conn,
        "keep_metadata",
        if opts.keep_metadata { "true" } else { "false" },
    );
    let _ = set_value(
        &conn,
        "use_custom_output",
        if opts.use_custom_output {
            "true"
        } else {
            "false"
        },
    );
    let _ = set_value(
        &conn,
        "custom_output_path",
        opts.custom_output_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
            .as_str(),
    );
    let _ = set_value(
        &conn,
        "generate_log",
        if opts.generate_log { "true" } else { "false" },
    );
    let _ = set_value(
        &conn,
        "add_numbering",
        if opts.add_numbering { "true" } else { "false" },
    );
    let _ = set_value(
        &conn,
        "is_dark_mode",
        if opts.is_dark_mode { "true" } else { "false" },
    );
    let _ = set_value(&conn, "max_batch_size", &opts.max_batch_size.to_string());
}

/// Retrieves a single setting value by key.
fn get_value(conn: &Connection, key: &str) -> SqlResult<String> {
    conn.query_row("SELECT value FROM settings WHERE key = ?1", [key], |row| {
        row.get(0)
    })
}

/// Stores a single setting value by key.
fn set_value(conn: &Connection, key: &str, value: &str) -> SqlResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        [key, value],
    )?;
    Ok(())
}

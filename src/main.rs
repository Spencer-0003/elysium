// SPDX-FileCopyrightText: 2025 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

#![warn(clippy::pedantic)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::option_if_let_else)]
#![deny(clippy::allow_attributes_without_reason)]
#![deny(clippy::string_to_string)]
#![deny(clippy::get_unwrap)]
#![deny(clippy::str_to_string)]
#![allow(clippy::unreadable_literal, reason = "'Readable' literals are ugly")]

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

fn main() {
    env_logger::init();

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let flatpak = std::env::var("FLATPAK_ID").is_ok() && std::fs::exists("/.flatpak-info").unwrap_or(false);
        log::info!("Elysium v{} (Linux) (Flatpak: {}, AppImage: {})", env!("CARGO_PKG_VERSION"), flatpak, !flatpak && std::env::var("APPIMAGE").is_ok());
    }

    #[cfg(target_os = "macos")]
    log::info!("Elysium v{} (MacOS)", env!("CARGO_PKG_VERSION"));

    #[cfg(windows)]
    log::info!("Elysium v{} (Windows)", env!("CARGO_PKG_VERSION"));
}

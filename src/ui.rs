// SPDX-FileCopyrightText: 2025 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

slint::include_modules!();

pub fn run() {
    let app = App::new().unwrap();

    slint::set_xdg_app_id("moe.spencer.Elysium").unwrap();

    app.run().unwrap();
}

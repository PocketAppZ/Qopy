use tauri::{
    menu::{MenuBuilder, MenuItemBuilder}, tray::TrayIconBuilder, Emitter, Manager
};

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let window = app.get_webview_window("main").unwrap();
    let window_clone_for_tray = window.clone();

    let icon_bytes = include_bytes!("../../icons/Square71x71Logo.png");
    let icon = tauri::image::Image::from_bytes(icon_bytes).unwrap();

    let _tray = TrayIconBuilder::new()
        .menu(
            &MenuBuilder::new(app)
                .items(&[&MenuItemBuilder::with_id("app_name", "Qopy")
                    .enabled(false)
                    .build(app)?])
                .items(&[&MenuItemBuilder::with_id("show", "Show/Hide").build(app)?])
                .items(&[&MenuItemBuilder::with_id("keybind", "Change keybind").build(app)?])
                .items(&[&MenuItemBuilder::with_id("quit", "Quit").build(app)?])
                .build()?,
        )
        .on_menu_event(move |_app, event| match event.id().as_ref() {
            "quit" => {
                std::process::exit(0);
            }
            "show" => {
                let is_visible = window_clone_for_tray.is_visible().unwrap();
                if is_visible {
                    window_clone_for_tray.hide().unwrap();
                } else {
                    window_clone_for_tray.show().unwrap();
                    window_clone_for_tray.set_focus().unwrap();
                }
                window_clone_for_tray.emit("main_route", ()).unwrap();
            }
            "keybind" => {
                window_clone_for_tray.emit("change_keybind", ()).unwrap();
            }
            _ => (),
        })
        .icon(icon)
        .build(app)?;

    Ok(())
}

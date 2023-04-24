slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });
    ui.on_select_dir(||{
        println!("select dir...");
        let path = std::env::current_dir().unwrap();
        let dialog = rfd::FileDialog::new()
        .set_directory(&path)
        .pick_folder();
        match dialog {
            Some(new_path) => {
                println!("目标目录: {:#?}", new_path.as_path().to_str());
            },
            None => {
                println!("请选择目标目录");
            }
        }
    });

    ui.run()
}

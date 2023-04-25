mod generate_code {
    slint::include_modules!();
}
pub use generate_code::*;
use worker::Worker;

mod worker;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let worker = Worker::new(&ui);
    let ui_handle = ui.as_weak();

    // ui.on_request_increase_value(move || {
    //     let ui = ui_handle.unwrap();
    //     ui.set_counter(ui.get_counter() + 1);
    // });

    ui.on_select_dir({
        let channel = worker.channel.clone();
        move || channel.send(worker::Message::ShowOpenDialog).unwrap()
    });

    ui.run()
}

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let ui = TemplateWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    ui.run()?;
    Ok(())
}

// fn main() -> Result<(), slint::PlatformError> {
//     let ui = TemplateWindow::new()?;

//     let ui_handle = ui.as_weak();
//     ui.on_request_increase_value(move || {
//         let ui = ui_handle.unwrap();
//         ui.set_counter(ui.get_counter() + 1);
//     });

//     ui.run()
// }

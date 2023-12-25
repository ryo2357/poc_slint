slint::slint! {
    import { Button, VerticalBox } from "std-widgets.slint";
    export component TemplateWindow inherits Window {
        in-out property<int> counter: 42;
        callback request-increase-value();
        VerticalBox {
            Text {
                text: "Counter_incode:" + root.counter;
            }
            Button {
                text: "Increase value";
                clicked => {
                    root.request-increase-value();
                }
            }
        }
    }
}

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

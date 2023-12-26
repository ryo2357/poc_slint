slint::slint! {

    export component MainWindow inherits Window {
        // width: 326px;
        // height: 326px;
        out property<length> mouse_x;
        out property<length> mouse_y;
        callback clicked;
        in property window_width <=> self.width;
        in property window_height <=> self.height;

        TouchArea {
            clicked => {
                root.mouse_x = self.pressed-x;
                root.mouse_y = self.pressed-y;
                root.clicked();

            }
        }
    }

}

fn main() -> anyhow::Result<()> {
    let mut ui = MainWindow::new()?;
    ui.set_window_width(500.0);
    ui.set_window_height(400.0);

    let weak = ui.as_weak();

    ui.on_clicked(move || {
        let app = weak.upgrade().unwrap();
        println!("Clicked");
        let x = app.get_mouse_x();
        let y = app.get_mouse_y();

        println!("x:{:?},y:{:?}", x, y);
        app.set_window_width(800.0);
        app.set_window_height(800.0);
    });

    ui.run()?;
    Ok(())
}

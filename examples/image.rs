slint::slint! {

    export component MainWindow inherits Window {
        // width: 326px;
        // height: 326px;
        out property<length> mouse_x;
        out property<length> mouse_y;
        callback clicked;
        in property window_width <=> self.width;
        in property window_height <=> self.height;

        in property<image> screen;

        Image {
            // source: screen;
            source: screen;
        }

        TouchArea {
            clicked => {
                root.mouse_x = self.pressed-x;
                root.mouse_y = self.pressed-y;
                root.clicked();

            }
        }
    }

}

// use image::io::Reader as ImageReader;
use image::{imageops::resize, imageops::FilterType, ImageBuffer};
use slint::Image;
use slint::Rgba8Pixel;
use slint::SharedPixelBuffer;

fn main() -> anyhow::Result<()> {
    // let img = ImageReader::open("assets/icons/at.png")?.decode()?;
    let img = image::open("assets/temp/screen_1.png")?.into_rgba8();

    // リサイズ
    let (width, height) = img.dimensions();
    let new_width = width / 2;
    let new_height = height / 2;

    let img: ImageBuffer<_, _> = resize(&img, new_width, new_height, FilterType::Nearest);
    // let buffer =
    //     SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(img.as_raw(), img.width(), img.height());
    let buffer =
        SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(img.as_raw(), img.width(), img.height());
    let image = Image::from_rgba8(buffer);
    let width = img.width();
    let height = img.height();

    let mut ui = MainWindow::new()?;
    ui.set_window_width(width as f32);
    ui.set_window_height(height as f32);
    println!("width:{:?},height:{:?}", width, height);
    ui.set_screen(image);

    let weak = ui.as_weak();

    ui.on_clicked(move || {
        let app = weak.upgrade().unwrap();
        println!("Clicked");
        let x = app.get_mouse_x() * 2.0;
        let y = app.get_mouse_y() * 2.0;

        println!("x:{:?},y:{:?}", x, y);
    });

    ui.run()?;
    Ok(())
}

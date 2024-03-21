use std::time::Instant;
use xcap::Window;

fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

fn main() {
    let start = Instant::now();
    let windows = Window::all().unwrap();

    let mut i = 0;

    for window in windows {
        // Screenshots cannot be taken from a minimized window
        if window.is_minimized() {
            continue;
        }

        println!(
            "Window: {:?} {:?} {:?}",
            window.title(),
            (window.x(), window.y(), window.width(), window.height()),
            (window.is_minimized(), window.is_maximized())
        );

        let image = window.capture_image().unwrap();
        image
            .save(format!(
                "target/window-{}-{}.png",
                i,
                normalized(window.title())
            ))
            .unwrap();

        i += 1;
    }

    println!("Running time: {:?}", start.elapsed());
}
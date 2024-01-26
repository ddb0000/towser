use std::io::{self, Write};
use web_view::*;
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn get_screen_size() -> (i32, i32) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_visible(false).build(&event_loop).unwrap();
    
    if let Some(monitor) = window.current_monitor() {
        let size = monitor.size();
        let width = size.width as i32;
        let height = size.height as i32;
        (width, height)
    } else {
        (1920, 1080)
    }
}

fn main() {
    let (width, height) = get_screen_size();
    print!("URL:");
    io::stdout().flush().unwrap();

    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap();

    web_view::builder()
        .title("Towser")
        .content(Content::Url(url.trim()))
        .size(width, height)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}

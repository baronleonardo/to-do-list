pub mod ui {
    use gtk::prelude::*;

    // static mut file_path: String = String::new();

    pub struct UI {
        x_pos: i32,
        y_pos: i32,
        width: i32,
        height: i32,
        window: Option<gtk::Window>,
        text_buf: Option<gtk::TextBuffer>,
    }

    impl UI {
        pub fn new(x_pos:i32, y_pos: i32, width: i32, height: i32) -> UI {
            gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
            UI {
                x_pos,
                y_pos,
                width,
                height,
                window: None,
                text_buf: None,
            }
        }

        pub fn build(&mut self) {
            let window = gtk::Window::new(gtk::WindowType::Toplevel);
            window.set_keep_below(true);
            window.set_type_hint(gdk::WindowTypeHint::Normal);
            window.set_decorated(false);
            window.set_deletable(false);
            window.set_skip_pager_hint(true);
            window.set_skip_taskbar_hint(true);
            window.stick();
            window.move_(self.x_pos, self.y_pos);
            window.set_border_width(4);
            window.set_default_size(self.width, self.height);

            window.set_title("TODO desktop widget");

            let text_view = gtk::TextView::new();

            window.add(&text_view);

            self.window   = Some(window);
            self.text_buf = Some(text_view.buffer().unwrap());
        }

        pub fn run(&self) {
            match &self.window {
                Some(window) => {
                    window.show_all();
                    gtk::main();
                },
                None => panic!("You must build UI first before show"),
            }
        }

        pub fn set_text(&self, text: &str) {
            match &self.text_buf {
                Some(text_buf) => {
                    text_buf.set_text(text);
                },
                None => panic!("You must build UI first before show"),
            }
        }

        #[warn(dead_code)]
        pub fn get_text(&self) -> String {
            match &self.text_buf {
                Some(text_buf) => {
                    let (start_itr, end_itr) = text_buf.bounds();
                    text_buf.text(&start_itr, &end_itr, false).unwrap().to_string()
                },
                None => panic!("You must build UI first before show"),
            }
        }

        pub fn signal_content_changed<F>(&self, func: F)
        where F: Fn(String) + 'static {
            self.text_buf.as_ref().unwrap().connect_changed(move |buf| {
                let (start_itr, end_itr) = buf.bounds();
                let text = buf.text(&start_itr, &end_itr, false).unwrap();

                func(text.to_string());
            });
        }
    }
}
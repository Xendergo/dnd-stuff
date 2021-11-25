// use druid::widget::{Align, Label};
// use druid::{AppLauncher, Data, Env, Lens, Widget, WindowDesc};

// use crate::utils::get_local_ip;

use iced::Sandbox;

// #[derive(Clone, Data, Lens)]
pub struct Gui {
}

// pub fn create_window() {
//     let main_window = WindowDesc::new(build)
//         .title("DnD server");

//     // create the initial app state
//     let initial_state = Gui {
//     };

//     // start the application
//     AppLauncher::with_window(main_window)
//         .launch(initial_state)
//         .expect("Failed to launch application");

//     // Exit when the window closes
//     std::process::exit(0);
// }

// fn build() -> impl Widget<Gui> {
//     let label = Label::new(|data: &Gui, _env: &Env| get_local_ip().unwrap_or("Couldn't get your ip address".to_owned()));

//     Align::centered(label)
// }

impl Sandbox for Gui {
    type Message = ();

    fn new() -> Gui {
        Gui {}
    }

    fn title(&self) -> String {
        "DnD stuff".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        iced::Text::new("bruh").into()
    }
}
use druid::widget::{Align, Label};
use druid::{AppLauncher, Data, Env, Lens, Widget, WindowDesc};

#[derive(Clone, Data, Lens)]
struct Gui {
}

pub fn create_window() {
    let main_window = WindowDesc::new(build)
        .title("DnD server")
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = Gui {
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");

    // Exit when the window closes
    std::process::exit(0);
}

fn build() -> impl Widget<Gui> {
    let label = Label::new(|data: &Gui, _env: &Env| "Test".to_string());

    Align::centered(label)
}
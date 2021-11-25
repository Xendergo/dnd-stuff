use iced::Sandbox;

pub struct Gui {
}

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
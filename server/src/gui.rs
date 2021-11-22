use iced::Sandbox;

pub enum Gui {
    CampaignSelector {},
}

#[derive(Debug)]
enum Message {}

impl Sandbox for Gui {
    type Message = Message;

    fn new() -> Self {
        Gui::CampaignSelector {}
    }

    fn title(&self) -> String {
        "DnD stuff server".to_owned()
    }
}

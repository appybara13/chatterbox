use chatter::{Chat, ChatContent, ChatPosition, Choice, Line};

mod utils;

pub struct Player {
    chat: Chat,
}

impl Player {
    pub fn new(chat: Chat) -> Player {
        Player { chat }
    }

    pub fn run(&self) {
        let mut current_position = ChatPosition::start();

        while current_position != ChatPosition::end() {
            current_position = self.step(current_position);
        }
    }

    pub fn step(&self, position: ChatPosition) -> ChatPosition {
        let content = match self.chat.get(position) {
            Some(content) => content,
            _ => return ChatPosition::end(),
        };

        match content {
            ChatContent::Line(line) => self.line_step(line),
            ChatContent::Choices(choices) => self.choices_step(choices),
        }
    }

    pub fn line_step(&self, line: &Line) -> ChatPosition {
        println!("{}", line.get_text());
        line.get_next()
    }

    pub fn choices_step(&self, choices: &[Choice]) -> ChatPosition {
        for (i, choice) in choices.iter().enumerate() {
            println!("{}. {}", i + 1, choice.get_text());
        }

        let choice = utils::read_usize(1, choices.len()) - 1;
        choices[choice].get_next()
    }
}

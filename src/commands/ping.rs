use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "Pong!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("Replies with Pong!!")
}

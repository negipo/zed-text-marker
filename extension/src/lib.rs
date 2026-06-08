use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree};

struct TextMarkerExtension;

impl zed::Extension for TextMarkerExtension {
    fn new() -> Self {
        TextMarkerExtension
    }

    fn language_server_command(
        &mut self,
        _id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let command = worktree
            .which("text-marker")
            .ok_or_else(|| "text-marker バイナリが PATH に見つかりません".to_string())?;
        Ok(Command {
            command,
            args: vec!["serve".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(TextMarkerExtension);

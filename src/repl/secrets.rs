use std::borrow::Cow;

use rustyline::{
    ColorMode, Editor,
    config::Configurer,
    highlight::{CmdKind, Highlighter},
};

use crate::{
    config::ConfigConnection,
    repl::def::{MaskingHighlighter, SecretCache, SecretState::Cached, SecretState::Interactive},
};

pub trait SecretProvider {
    fn get_secret(&mut self, connection: &ConfigConnection) -> String;
    fn invalidate_secret(&mut self, connection: &ConfigConnection);
}

impl SecretProvider for SecretCache {
    fn get_secret(&mut self, connection: &ConfigConnection) -> String {
        let secret = match self.secrets.get(&connection.name) {
            Some(Cached(secret)) => secret.clone(),
            None if !connection.auth.client_secret.is_empty() => {
                connection.auth.client_secret.clone()
            }
            _ => Self::read_interactive(),
        };
        self.secrets
            .insert(connection.name.clone(), Cached(secret.clone()));
        secret
    }

    fn invalidate_secret(&mut self, connection: &ConfigConnection) {
        self.secrets.insert(connection.name.clone(), Interactive);
    }
}

impl SecretCache {
    fn read_interactive() -> String {
        let Ok(mut editor) = Editor::new() else {
            return String::new();
        };
        editor.set_helper(Some(MaskingHighlighter {}));
        editor.set_color_mode(ColorMode::Forced);
        editor.set_auto_add_history(false);
        editor.readline("Client secret: ").unwrap_or_default()
    }
}

impl Highlighter for MaskingHighlighter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Owned("\u{2022}".repeat(line.len()))
    }

    fn highlight_char(&self, _line: &str, _pos: usize, kind: CmdKind) -> bool {
        !matches!(kind, CmdKind::MoveCursor)
    }
}

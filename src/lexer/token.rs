use crate::lexer::{FormatToken, TerminalIndex};
use miette::SourceSpan;
use std::borrow::Cow;
use std::convert::From;
use std::fmt::{Debug, Display, Error, Formatter};

use super::Location;

//
// Special token constants the lexer has to deal with regularly.
// There are some special fix values used for common token types.
//

/// End of input constant
pub const EOI: TerminalIndex = 0;
/// New line token
pub const NEW_LINE: TerminalIndex = 1;
/// Whitespace token
pub const WHITESPACE: TerminalIndex = 2;
/// Line comment token
pub const LINE_COMMENT: TerminalIndex = 3;
/// Block comment token
pub const BLOCK_COMMENT: TerminalIndex = 4;
/// Index of the first user token.
pub const FIRST_USER_TOKEN: TerminalIndex = 5;

const EOI_TOKEN: &str = "$";

///
/// The Token<'t> type represents a scanned token.
/// It has a reference to the scanned text in the text member.
///
/// The lifetime parameter `'t` refers to the lifetime of the scanned text.
///
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Token<'t> {
    /// The matched string
    pub(crate) text: Cow<'t, str>,

    /// The index of the terminal in the augmented terminal list
    pub token_type: TerminalIndex,

    /// Position information
    pub location: Location,
}

impl<'t> Token<'t> {
    ///
    /// Creates an End-Of-Input token
    ///
    pub fn eoi() -> Self {
        Self {
            text: EOI_TOKEN.into(),
            token_type: EOI,
            location: Location::default(),
        }
    }

    ///
    /// Creates a token with given values.
    ///
    pub fn with<T>(text: T, token_type: TerminalIndex, location: Location) -> Self
    where
        T: Into<Cow<'t, str>>,
    {
        Self {
            text: text.into(),
            token_type,
            location,
        }
    }

    ///
    /// Indicates wether the token is normally skipped by the TokenStream.
    /// The behavior is independent from the language.
    ///
    pub fn is_skip_token(&self) -> bool {
        self.token_type > EOI && self.token_type < FIRST_USER_TOKEN
    }

    ///
    /// Accesses the token's scanned text
    ///
    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    ///
    /// Creates an owned variant of the token
    ///
    pub fn to_owned(&self) -> Self {
        Self {
            text: self.text.to_owned(),
            token_type: self.token_type,
            location: self.location.clone(),
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        let (c1, c2) = if self.text.starts_with('\'') {
            ('<', '>')
        } else {
            ('\'', '\'')
        };
        write!(
            f,
            "{}{}{}, Ty:{}, at {}",
            c1, self.text, c2, self.token_type, self.location
        )
    }
}

impl FormatToken for Token<'_> {
    fn format(&self, terminal_names: &'static [&'static str]) -> String {
        let name = terminal_names[self.token_type];
        format!(
            "'{}'({}) at {}",
            self.text.escape_default(),
            name,
            self.location,
        )
    }
}

impl From<&Token<'_>> for SourceSpan {
    fn from(token: &Token<'_>) -> Self {
        (&token.location).into()
    }
}

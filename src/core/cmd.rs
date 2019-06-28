//! Command system for xi-term. A command represents
//! a task the user wants the editor to preform,
/// currently commands can only be input through the CommandPrompt. Vim style.
use xrl::ViewId;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::KeymapEntry;
use crate::widgets::CommandPromptMode;

pub trait FromPrompt {
    fn from_prompt(vals: &str) -> Result<Command, ParseCommandError>;
}

pub trait ToPrompt {
    fn to_prompt(&self) -> String;
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum RelativeMoveDistance {
    /// Move only one character
    characters,
    /// Move a line
    lines,
    /// Move to new word
    words,
    /// Move to end of word
    word_ends,
    /// Move to new subword
    subwords,
    /// Move to end of subword
    subword_ends,
    /// Move a page
    pages,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct RelativeMove {
    pub by: RelativeMoveDistance,
    pub forward: bool,
    #[serde(default)]
    pub extend: bool
}

impl ToPrompt for RelativeMove {
    fn to_prompt(&self) -> String {
        use RelativeMoveDistance::*;

        let mut ret = "move ".to_string();
        match self.by {
            characters => {ret.push_str( if self.forward {"left"} else {"right"} ) },
            lines => {ret.push_str( if self.forward {"down"} else {"up"} )},
            words => {ret.push_str( if self.forward {"wordleft"} else {"wordright"} )},
            word_ends => {ret.push_str( if self.forward {"wendleft"} else {"wendright"} )},
            subwords => {ret.push_str( if self.forward {"subwordleft"} else {"subwordright"} )},
            subword_ends => {ret.push_str( if self.forward {"subwendleft"} else {"subwendright"} )},
            pages => {ret.push_str( if self.forward {"page-down"} else {"page-up"} )},
        }

        if self.extend {
            ret.push_str(" (e)xtend");
        }
        ret
    }
}

impl FromPrompt for RelativeMove {
    fn from_prompt(args: &str) -> Result<Command, ParseCommandError> {
        let vals : Vec<&str> = args.split(' ').collect();
        if vals.is_empty() {
            return Err(ParseCommandError::ExpectedArgument{cmd: "move".to_string()});
        }

        if vals.len() > 2 {
            return Err(ParseCommandError::TooManyArguments{cmd: "move".to_string(), expected: 2, found: vals.len()});
        }

        let extend = vals.len() == 2;
        match vals[0] {
            "d" | "down" => Ok(Command::RelativeMove(
                                RelativeMove{
                                            by: RelativeMoveDistance::lines, 
                                            forward: true, 
                                            extend
                                            }
                               )),
            "u" | "up" => Ok(Command::RelativeMove(
                                RelativeMove{
                                            by: RelativeMoveDistance::lines, 
                                            forward: false, 
                                            extend
                                            }
                               )),
            "r" | "right" => Ok(Command::RelativeMove(
                                RelativeMove{
                                            by: RelativeMoveDistance::characters, 
                                            forward: true, 
                                            extend
                                            }
                               )),
            "l" | "left" => Ok(Command::RelativeMove(
                                RelativeMove{
                                            by: RelativeMoveDistance::characters, 
                                            forward: false, 
                                            extend
                                            }
                               )),
            "pd" | "page-down" => Ok(Command::RelativeMove(
                                        RelativeMove{
                                                    by: RelativeMoveDistance::pages, 
                                                    forward: true, 
                                                    extend
                                                    }
                                       )),
            "pu" | "page-up" => Ok(Command::RelativeMove(
                                        RelativeMove{
                                                    by: RelativeMoveDistance::pages, 
                                                    forward: false, 
                                                    extend
                                                    }
                                       )),
            command => Err(ParseCommandError::UnknownCommand(command.into()))
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum AbsoluteMovePoint {
    /// Beginning of file
    bof,
    /// End of file
    eof,
    /// Beginning of line
    bol,
    /// End of line
    eol,
    /// Enclosing brackets
    brackets,
    /// Line number
    line(u64)
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct AbsoluteMove {
    pub to: AbsoluteMovePoint,
    #[serde(default)]
    pub extend: bool
}

impl ToPrompt for AbsoluteMove {
    fn to_prompt(&self) -> String {
        use AbsoluteMovePoint::*;

        let mut ret = "move ".to_string();
        match self.to {
            bof => {ret.push_str("bof")}
            eof => {ret.push_str("eof")}
            bol => {ret.push_str("bol")}
            eol => {ret.push_str("eol")}
            brackets => {ret.push_str("brackets")}
            line(_) => {ret.push_str("<line>")}
        }

        if self.extend {
            ret.push_str(" (e)xtend");
        }
        ret
    }
}


impl FromPrompt for AbsoluteMove {
    fn from_prompt(args: &str) -> Result<Command, ParseCommandError> {
        let vals : Vec<&str> = args.split(' ').collect();
        if vals.is_empty() {
            return Err(ParseCommandError::ExpectedArgument{cmd: "move_to".to_string()});
        }

        if vals.len() > 2 {
            return Err(ParseCommandError::TooManyArguments{cmd: "move_to".to_string(), expected: 2, found: vals.len()});
        }

        let extend = vals.len() == 2;
        match vals[0] {
            "bof" | "beginning-of-file" => Ok(Command::AbsoluteMove(
                                                    AbsoluteMove{
                                                                to: AbsoluteMovePoint::bof,
                                                                extend
                                                                }
                                                   )),
            "eof" | "end-of-file" => Ok(Command::AbsoluteMove(
                                                    AbsoluteMove{
                                                                to: AbsoluteMovePoint::eof,
                                                                extend
                                                                }
                                                   )),
            "bol" | "beginning-of-line" => Ok(Command::AbsoluteMove(
                                                    AbsoluteMove{
                                                                to: AbsoluteMovePoint::bol,
                                                                extend
                                                                }
                                                   )),
            "eol" | "end-of-line" => Ok(Command::AbsoluteMove(
                                                    AbsoluteMove{
                                                                to: AbsoluteMovePoint::eol,
                                                                extend
                                                                }
                                                   )),

            command => {
                let number = command.parse::<u64>().map_err(|_| ParseCommandError::UnknownCommand(command.into()))?;
                Ok(Command::AbsoluteMove(
                                AbsoluteMove{
                                            to: AbsoluteMovePoint::line(number),
                                            extend: false
                                            }
                                )
                )
            }
        }

    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ExpandLinesDirection {
    pub forward: bool
}

#[derive(Debug, Clone, PartialEq)]
pub struct FindConfig {
    pub search_term: String,
    pub case_sensitive: bool,
    pub regex: bool,
    pub whole_words: bool,
}

impl FromPrompt for FindConfig {
    fn from_prompt(args: &str) -> Result<Command, ParseCommandError> {
        if args.is_empty() {
            return Err(ParseCommandError::ExpectedArgument{cmd: "find".to_string()})
        }

        let mut search_term = args;
        let mut case_sensitive = false;
        let mut regex = false;
        let mut whole_words = false;

        let argsvec : Vec<&str> = args.splitn(2, ' ').collect();

        if argsvec.len() == 2 && argsvec[0].len() <= 3 {
            // We might have search control characters here
            let control_chars = argsvec[0];

            let mut failed = false;
            let mut shadows = [false, false, false];
            for cc in control_chars.chars() {
                match cc {
                    'c' => shadows[0] = true,
                    'r' => shadows[1] = true,
                    'w' => shadows[2] = true,
                    _ => {
                        // Ooops! This first part is NOT a control-sequence after all. Treat it as normal text
                        failed = true;
                        break;
                    }
                }
            }

            if !failed {
                // Strip away control characters of search_term
                search_term = argsvec[1];
                case_sensitive = shadows[0];
                regex          = shadows[1];
                whole_words    = shadows[2];
            }
        }

        let config = FindConfig{
            search_term: search_term.to_string(),
            case_sensitive,
            regex,
            whole_words,
        };
        Ok(Command::Find(config))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    /// Close the CommandPrompt.
    Cancel,
    /// Quit editor.
    Quit,
    /// Save the current file buffer.
    Save(Option<ViewId>),
    /// Backspace
    Back,
    /// Delete
    Delete,
    /// Open A new file.
    Open(Option<String>),
    /// Cycle to the next View.
    NextBuffer,
    /// Cycle to the previous buffer.
    PrevBuffer,
    /// Relative move like line up/down, page up/down, left, right, word left, ..
    RelativeMove(RelativeMove),
    /// Relative move like line ending/beginning, file ending/beginning, line-number, ...
    AbsoluteMove(AbsoluteMove),
    /// Change current color theme
    SetTheme(String),
    /// Toggle displaying line numbers.
    ToggleLineNumbers,
    /// Open prompt for user-input
    OpenPrompt(CommandPromptMode),
    /// Insert a character
    Insert(char),
    /// Undo last action
    Undo,
    /// Redo last undone action
    Redo,
    /// Find the given string
    Find(FindConfig),
    /// Find next occurence of active search
    FindNext,
    /// Find previous occurence of active search
    FindPrev,
    /// Find word and set another cursor there
    FindUnderExpand,
    /// Set a new cursor below or above current position
    CursorExpandLines(ExpandLinesDirection),
    /// Copy the current selection
    CopySelection,
    /// Paste previously copied or cut text
    Paste,
    /// Copy the current selection
    CutSelection,
    /// Close the current view
    CloseCurrentView,
    /// Select all text in the current view
    SelectAll,
}

#[derive(Debug)]
pub enum ParseCommandError {
    /// Didnt expect a command to take an argument.
    UnexpectedArgument,
    /// The given command expected an argument.
    ExpectedArgument {
        cmd: String,
        // expected: usize,
        // found: usize,
    },
    /// The given command was given to many arguments.
    TooManyArguments {
        cmd: String,
        expected: usize,
        found: usize,
    },
    /// Invalid input was received.
    UnknownCommand(String),
}

impl Command {

    pub fn from_keymap_entry(val: KeymapEntry) -> Result<Command, ParseCommandError> {
        match val.command.as_ref() {
            "select_all" => Ok(Command::SelectAll),
            "close" => Ok(Command::CloseCurrentView),
            "copy" => Ok(Command::CopySelection),
            "cut" => Ok(Command::CutSelection),
            "paste" => Ok(Command::Paste),
            "fue" | "find_under_expand" => Ok(Command::FindUnderExpand),
            "fn" | "find_next" => Ok(Command::FindNext),
            "fp" | "find_prev" => Ok(Command::FindPrev),
            "hide_overlay" => Ok(Command::Cancel),
            "s" | "save" => Ok(Command::Save(None)),
            "q" | "quit" | "exit" => Ok(Command::Quit),
            "b" | "back" | "left_delete" => Ok(Command::Back),
            "d" | "delete" | "right_delete" => Ok(Command::Delete),
            "bn" | "next-buffer" | "next_view" => Ok(Command::NextBuffer),
            "bp" | "prev-buffer" | "prev_view" => Ok(Command::PrevBuffer),
            "undo" => Ok(Command::Undo),
            "redo" => Ok(Command::Redo),
            "ln" | "line-numbers" => Ok(Command::ToggleLineNumbers),
            "op" | "open-prompt" => Ok(Command::OpenPrompt(CommandPromptMode::Command)),
            "show_overlay" => {
                let args = val.args.ok_or(ParseCommandError::ExpectedArgument{cmd: "show_overlay".to_string()})?;
                match args.get("overlay") {
                    None => Err(ParseCommandError::UnexpectedArgument),
                    Some(value) => match value {
                                        // We should catch "command_palette" here instead, but because of a bug in termion
                                        // we can't parse ctrl+shift+p...
                                        // Later on we might introduce another prompt mode for "goto" as well.
                                        Value::String(x) if x == "goto" => Ok(Command::OpenPrompt(CommandPromptMode::Command)),
                                        _ => Err(ParseCommandError::UnexpectedArgument),
                                   }
                }
            }

            "show_panel" => {
                let args = val.args.ok_or(ParseCommandError::ExpectedArgument{cmd: "show_panel".to_string()})?;
                match args.get("panel") {
                    None => Err(ParseCommandError::UnexpectedArgument),
                    Some(value) => match value {
                                        Value::String(x) if x == "find" => Ok(Command::OpenPrompt(CommandPromptMode::Find)),
                                        _ => Err(ParseCommandError::UnexpectedArgument),
                                   }
                }
            }


            "move"    => {
                let args = val.args.ok_or(ParseCommandError::ExpectedArgument{cmd: "move".to_string()})?;
                let cmd : RelativeMove = serde_json::from_value(args).map_err(|_| ParseCommandError::UnexpectedArgument)?;
                Ok(Command::RelativeMove(cmd))
            },
            "move_to" => {
                let args = val.args.ok_or(ParseCommandError::ExpectedArgument{cmd: "move_to".to_string()})?;
                let cmd : AbsoluteMove = serde_json::from_value(args).map_err(|_| ParseCommandError::UnexpectedArgument)?;
                Ok(Command::AbsoluteMove(cmd))
            },
            "select_lines" => {
                let args = val.args.ok_or(ParseCommandError::ExpectedArgument{cmd: "select_lines".to_string()})?;
                let cmd : ExpandLinesDirection = serde_json::from_value(args).map_err(|_| ParseCommandError::UnexpectedArgument)?;
                Ok(Command::CursorExpandLines(cmd))
            },
            command => Err(ParseCommandError::UnknownCommand(command.into())),
        }
    }
}

impl FromPrompt for Command {
    fn from_prompt(input: &str) -> Result<Command, ParseCommandError> {
        let mut parts: Vec<&str> = input.splitn(2, ' ').collect();
        let cmd = parts.remove(0);

        // If we have prompt-arguments, we parse them directly to a command instead of going via json
        let args = parts.get(0);
        match cmd.as_ref() {
            // First, catch some prompt-specific commands (usually those with arguments),
            // which need different parsing than whats coming from the keymap-file
            "move"    => {
                let arg = args.ok_or(ParseCommandError::ExpectedArgument{cmd: "move".to_string()})?;
                RelativeMove::from_prompt(arg)
            },
            "move_to" => {
                let arg = args.ok_or(ParseCommandError::ExpectedArgument{cmd: "move".to_string()})?;
                AbsoluteMove::from_prompt(arg)
            },
            "t" | "theme" => {
                let theme = args.ok_or(ParseCommandError::ExpectedArgument{cmd: "theme".to_string()})?;
                Ok(Command::SetTheme(theme.to_string()))
            },
            "o" | "open" => {
                // Don't split given arguments by space, as filenames can have spaces in them as well!
                let filename = match args {
                    Some(name) => {
                        // We take the value given from the prompt and run it through shellexpand,
                        // to translate to a real path (e.g. "~/.bashrc" doesn't work without this)
                        let expanded_name = shellexpand::full(name)
                                               .map_err(|_| ParseCommandError::UnknownCommand(name.to_string()))?;
                        Some(expanded_name.to_string())
                    },

                    // If no args where given we open with "None", which is ok, too.
                    None => None,
                };
                Ok(Command::Open(filename))
            }

            "f" | "find" => {
                let needle = args.ok_or(ParseCommandError::ExpectedArgument{cmd: "find".to_string()})?;
                FindConfig::from_prompt(needle)
            },

            // The stuff we don't handle here, we pass on to the default parsing function
            // Since there is no way to know the shape of "args", we drop all 
            // potentially given prompt-args for this command here.
            command => Command::from_keymap_entry(KeymapEntry{keys: Vec::new(), 
                                                  command: command.to_string(), 
                                                  args: None, 
                                                  context: None})
        }
    }
}

impl ToPrompt for Command {
    fn to_prompt(&self) -> String {
        use Command::*;

        let mut ret = String::new();
        match self {
            Cancel => ret.push_str("cancel"),
            Quit => ret.push_str("quit"),
            Save(_) => ret.push_str("save"),
            Back => ret.push_str("back"),
            Delete => ret.push_str("delete"),
            Open(_) => ret.push_str("open"),
            NextBuffer => ret.push_str("buffernext"),
            PrevBuffer => ret.push_str("bufferprev"),
            RelativeMove(x) => ret.push_str(&x.to_prompt()),
            AbsoluteMove(x) => ret.push_str(&x.to_prompt()),
            SetTheme(_) => ret.push_str("settheme"),
            ToggleLineNumbers => ret.push_str("togglelinenumbers"),
            OpenPrompt(_) => ret.push_str("open-prompt"),
            Insert(_) => ret.push_str("insert"),
            Undo => ret.push_str("undo"),
            Redo => ret.push_str("redo"),
            Find(_) => ret.push_str("find"),
            FindNext => ret.push_str("findnext"),
            FindPrev => ret.push_str("findprev"),
            FindUnderExpand => ret.push_str("find_under_expand"),
            CursorExpandLines(_) => ret.push_str("cursor_expand_lines"),
            CopySelection => ret.push_str("copy"),
            Paste => ret.push_str("paste"),
            CutSelection => ret.push_str("cut"),
            CloseCurrentView => ret.push_str("close"),
            SelectAll => ret.push_str("selecta_ll"),
        }
        ret
    }
}

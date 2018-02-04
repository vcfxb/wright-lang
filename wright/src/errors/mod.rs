//! Error module.
//! Contains traits and Constants for error printing.

use std::fmt;
use std::fmt::Debug;
use position::span::Span;
extern crate ansi_term;
use self::ansi_term::Color::*;
use self::ansi_term::Color;


/// Color code for errors used throughout entire error reporting system.
///
#[deprecated(since = "0.5.0")]
pub const ERROR_COLORS: [Color; 4] = [
    Red, Cyan, Green, Cyan,
];

/// Color for names which are printed by the compiler.
pub const NAME_COLOR: Color = Red;

/// Color for modules when printed by the compiler.
pub const MODULE_COLOR: Color = Cyan;

/// Trait for Errors. Any error used throughout the wright compiler/interpreter must implement
/// this trait for consistency.
pub trait Error : Debug + Sized {
    /// Return the name of the error.
    fn get_name(&self)   -> &str;
    /// Return the module or location the error came from.
    fn get_module(&self) -> &str;
    /// Returns a vector of the content spans (if any) of the offending content.
    fn get_spans(&self)  -> Vec<Span>;
    /// Turn error into one which can be displayed.
    /// (ErrorToDisplay implements fmt::Display)
    fn into_displayable(self) ->  ErrorToDisplay {
        ErrorToDisplay {
            name:      self.get_name().to_string(),
            module:    self.get_module().to_string(),
            line_info: ErrorToDisplay::get_line_info(self.get_spans()),
        }
    }
}

#[derive(Debug, Clone)]
/// ErrorToDisplay is an intermediate type used to go from a raw error
/// into a format that can be printed easily.
pub struct ErrorToDisplay {
    name:      String,
    module:    String,
    line_info: String,
}

impl ErrorToDisplay {
    /// Prints this error into the terminal.
    pub fn display(&self) {
        println!("{}", self);
    }
    /// Takes set of spans and uses it to set line info.
    pub fn get_line_info(span_vec: Vec<Span>) -> String {
        unimplemented!()    // todo!
    }
}

impl fmt::Display for ErrorToDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{n} in {m}{l_info}",
            n = NAME_COLOR.paint(self.name.clone()),
            m = MODULE_COLOR.paint(self.module.clone()),
            l_info = self.clone(),
        )
    }
}


// old:

//impl fmt::Display for ErrorToDisplay {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        if !(self.last_line.is_some()) {
//            write!(f,"{name} in {module} on line {line} characters {s_char}->{e_char}:\n{five}{i}\n\
//                    {five} {b}\n{line:>width$} {b} {l}\n{five} {b} {a}\n",
//                   name = NAME_COLOR.paint(self.name),
//                   line = self.start_position.get_line(),
//                   width = 5,
//                   module = MODULE_COLOR.paint(self.module_name.clone()),
//                   s_char = self.start_position.get_col(),
//                   e_char = self.end_position.get_col(),
//                   five = " ".repeat(5),
//                   i = ERROR_COLORS[3].paint(self.info.clone()),
//                   l = ERROR_COLORS[2].paint(self.first_line.clone()),
//                   b = ERROR_COLORS[3].paint("|"),
//                   a = ERROR_COLORS[0].bold().paint(self.first_line_arrow_str.clone())
//            )
//        } else {
//            write!(f, "{name} in {module} from {s_line}:{s_col} to {e_line}:{e_col}:\n{five}{i}\n\
//                    {five} {b}\n{s_line:>width$} {b} {s_source}\n{five} {b} {a_one}\n{five} {b} \
//                    ...\n{e_line:>width$} {b} {e_source}\n{five} {b} {a_two}\n",
//                   name = ERROR_COLORS[0].paint("ParserError"),
//                   s_line = self.start_position.get_line(),
//                   e_line = self.end_position.get_line(),
//                   width = 5,
//                   module = ERROR_COLORS[1].paint(self.module_name.clone()),
//                   s_col = self.start_position.get_col(),
//                   e_col = self.end_position.get_col(),
//                   five = " ".repeat(5),
//                   i = ERROR_COLORS[3].paint(self.info.clone()),
//                   s_source = ERROR_COLORS[2].paint(self.first_line.clone()),
//                   e_source = ERROR_COLORS[2].paint(self.last_line.clone().unwrap()),
//                   b = ERROR_COLORS[3].paint("|"),
//                   a_one = ERROR_COLORS[0].bold().paint(self.first_line_arrow_str.clone()),
//                   a_two = ERROR_COLORS[0].bold().paint(self.last_line_arrow_str.clone()),
//            )
//        }
//    }
//}
//! Statements in Wright source code. 

use self::bind::Bind;

pub mod bind;

pub enum Statement<'src> {
    /// A variable or constant binding. 
    Bind(Bind<'src>)
}

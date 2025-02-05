pub mod commands;
pub mod hir;
pub mod parse;
pub mod parse_command;

pub use crate::commands::classified::{ClassifiedCommand, ClassifiedPipeline, InternalCommand};
pub use crate::commands::ExternalCommand;
pub use crate::hir::syntax_shape::flat_shape::FlatShape;
pub use crate::hir::syntax_shape::{expand_syntax, ExpandSyntax, PipelineShape, SignatureRegistry};
pub use crate::hir::tokens_iterator::TokensIterator;
pub use crate::parse::files::Files;
pub use crate::parse::flag::Flag;
pub use crate::parse::operator::Operator;
pub use crate::parse::parser::pipeline;
pub use crate::parse::parser::Number;
pub use crate::parse::token_tree::{Delimiter, TokenNode};
pub use crate::parse::token_tree_builder::TokenTreeBuilder;

use nu_errors::ShellError;
use nu_source::nom_input;

pub fn parse(input: &str) -> Result<TokenNode, ShellError> {
    let _ = pretty_env_logger::try_init();

    match pipeline(nom_input(input)) {
        Ok((_rest, val)) => Ok(val),
        Err(err) => Err(ShellError::parse_error(err)),
    }
}

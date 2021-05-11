use crate::parser::Rule;
use crate::types::LineNo;
use either::Either;
use pest::error::{InputLocation, LineColLocation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ErrorCode {}

#[derive(new, Debug, Serialize, Deserialize, Clone)]
pub struct Error {
    lno: LineNo,
    variant: ErrorVariant,
}

#[derive(new, Debug, Serialize, Deserialize, Clone)]
pub enum ErrorVariant {
    Message(String),
    ErrorCode(ErrorCode),
    Parse(PestErrorInfo),
}

impl Error {
    pub fn new_from_parse(error: pest::error::Error<Rule>) -> Self {
        let lno = match error.line_col {
            LineColLocation::Pos(pos) => (pos.0, pos.0),
            LineColLocation::Span(start, end) => (start.0, end.0),
        };

        Error {
            lno,
            variant: ErrorVariant::Parse(error.extract()),
        }
    }
}

/*
This is a super hacky way to deserialize and
import information from Pest Errors into Serde,
this isn't perfect nor good, but the only way I could come up with
*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PestErrorInfo {
    Error {
        variant: Box<PestErrorInfo>,
        location: Box<PestErrorInfo>,
        line_col: Box<PestErrorInfo>,
    },
    ErrorVariantParsingError {
        positives: Vec<String>,
        negatives: Vec<String>,
    },
    ErrorVariantCustomError(String),
    InputLocation(Either<usize, (usize, usize)>),
    LineColLocation(Either<(usize, usize), ((usize, usize), (usize, usize))>),
}

trait ExtractInformation {
    fn extract(&self) -> PestErrorInfo;
}

impl ExtractInformation for pest::error::ErrorVariant<Rule> {
    fn extract(&self) -> PestErrorInfo {
        match self {
            pest::error::ErrorVariant::ParsingError {
                positives,
                negatives,
            } => PestErrorInfo::ErrorVariantParsingError {
                positives: positives.iter().map(|f| format!("{:?}", f)).collect(),
                negatives: negatives.iter().map(|f| format!("{:?}", f)).collect(),
            },
            pest::error::ErrorVariant::CustomError { message } => {
                PestErrorInfo::ErrorVariantCustomError(message.clone())
            }
        }
    }
}

impl ExtractInformation for pest::error::LineColLocation {
    fn extract(&self) -> PestErrorInfo {
        match self {
            LineColLocation::Pos(pos) => PestErrorInfo::LineColLocation(Either::Left(pos.clone())),
            LineColLocation::Span(start, end) => {
                PestErrorInfo::LineColLocation(Either::Right((start.clone(), end.clone())))
            }
        }
    }
}

impl ExtractInformation for pest::error::InputLocation {
    fn extract(&self) -> PestErrorInfo {
        match self {
            InputLocation::Pos(pos) => PestErrorInfo::InputLocation(Either::Left(pos.clone())),
            InputLocation::Span(pos) => PestErrorInfo::InputLocation(Either::Right(pos.clone())),
        }
    }
}

impl ExtractInformation for pest::error::Error<Rule> {
    fn extract(&self) -> PestErrorInfo {
        PestErrorInfo::Error {
            variant: Box::new(self.variant.extract()),
            location: Box::new(self.location.extract()),
            line_col: Box::new(self.line_col.extract()),
        }
    }
}

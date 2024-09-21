use thiserror::Error;

use crate::parse::{token, Program};
use crate::util::Location;

use self::env::Env;

pub mod env;
pub mod value;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("{0} RuntimeError: Invalid binary operation '{1}' between '{2}' and '{3}'")]
    InvalidBinaryOp(Location, token::BinaryOp, &'static str, &'static str),

    #[error("{0} RuntimeError: Invalid unary operation '{1}' on '{2}'")]
    InvalidUnaryOp(Location, token::UnaryOp, &'static str),

    #[error("{0} RuntimeError: Trying to access undefined variable: '{1}'")]
    UndefinedVariable(Location, String),
}

impl RuntimeError {
    pub fn loc(&self) -> Location {
        match self {
            RuntimeError::InvalidBinaryOp(loc, _, _, _) => *loc,
            RuntimeError::InvalidUnaryOp(loc, _, _) => *loc,
            RuntimeError::UndefinedVariable(loc, _) => *loc,
        }
    }
}

pub struct Interpreter {
    environment: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Env::new(),
        }
    }

    pub fn interpret(&mut self, program: Program) -> Result<(), RuntimeError> {
        let env = &mut self.environment;
        for stmt in program.statements.into_iter() {
            stmt.execute(env)?
        }
        Ok(())
    }
}

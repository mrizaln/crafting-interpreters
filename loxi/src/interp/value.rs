use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::rc::Rc;

use lasso::{Rodeo, Spur};

use super::{
    function::{Function, NativeFunction},
    object::Object,
};

// TODO: use Rc for heavy object (String and LoxObject)
#[derive(PartialEq, PartialOrd)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(Rc<String>),
    Object(Rc<Object>),
    Function(Rc<Function>),
    NativeFunction(Rc<NativeFunction>),

    /// `StringLiteral` is a special case of string, the value is static.
    /// It can only produces real `String` if it was operated on.
    ///
    /// NOTE: There is no distinction between `String` and string literal in Lox, this is just an
    ///       optimization.
    StringLiteral(Spur),
}

impl Value {
    pub fn nil() -> Self {
        Value::Nil
    }

    pub fn bool(b: bool) -> Self {
        Value::Bool(b)
    }

    pub fn number(num: f64) -> Self {
        Value::Number(num)
    }

    pub fn string(str: String) -> Self {
        Value::String(Rc::new(str))
    }

    pub fn object(obj: Object) -> Self {
        Value::Object(Rc::new(obj))
    }

    pub fn function(func: Function) -> Self {
        Value::Function(Rc::new(func))
    }

    pub fn native_function(func: NativeFunction) -> Self {
        Value::NativeFunction(Rc::new(func))
    }

    pub fn string_literal(spur: Spur) -> Self {
        Value::StringLiteral(spur)
    }

    /// follows Ruby's simple rule: `false` and `nil` are falsy, everything else truthy
    pub fn truthiness(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }

    pub fn not(&self) -> Option<Value> {
        Some(Value::Bool(!self.truthiness()))
    }

    pub fn minus(&self) -> Option<Value> {
        match self {
            Value::Number(num) => Some(Value::number(-num)),
            _ => None,
        }
    }

    pub fn add(self, other: Self, arena: &Rodeo) -> Option<Value> {
        match (self, other) {
            (Value::Number(num1), Value::Number(num2)) => Some(Value::number(num1 + num2)),
            (Value::String(str1), Value::String(str2)) => {
                let mut new_str = str1.deref().clone();
                new_str.push_str(str2.deref().as_str());
                Some(Value::string(new_str))
            }
            (Value::String(str1), Value::StringLiteral(spur)) => {
                let mut new_str = str1.deref().clone();
                let str2 = arena.resolve(&spur);
                new_str.push_str(str2);
                Some(Value::string(new_str))
            }
            _ => None,
        }
    }

    pub fn sub(self, other: Self) -> Option<Value> {
        match (self, other) {
            (Value::Number(num1), Value::Number(num2)) => Some(Value::number(num1 - num2)),
            _ => None,
        }
    }

    pub fn mul(self, other: Self) -> Option<Value> {
        match (self, other) {
            (Value::Number(num1), Value::Number(num2)) => Some(Value::number(num1 * num2)),
            _ => None,
        }
    }

    pub fn div(self, other: Self) -> Option<Value> {
        match (self, other) {
            (Value::Number(num1), Value::Number(num2)) => Some(Value::number(num1 / num2)),
            _ => None,
        }
    }

    pub fn eq(&self, other: &Self, arena: &Rodeo) -> Option<Value> {
        match (self, other) {
            (Value::Nil, Value::Nil) => Some(Value::bool(true)),
            (Value::Bool(b1), Value::Bool(b2)) => Some(Value::bool(b1 == b2)),
            (Value::Number(num1), Value::Number(num2)) => Some(Value::bool(num1 == num2)),
            (Value::String(str1), Value::String(str2)) => Some(Value::bool(str1 == str2)),
            (Value::Object(_), Value::Object(_)) => unimplemented!(),
            (Value::String(str1), Value::StringLiteral(str2)) => {
                Some(Value::bool(str1.as_str() == arena.resolve(str2)))
            }
            (Value::StringLiteral(str1), Value::String(str2)) => {
                Some(Value::bool(arena.resolve(str1) == str2.as_str()))
            }
            (Value::StringLiteral(str1), Value::StringLiteral(str2)) => {
                Some(Value::bool(str1 == str2))
            }
            _ => Some(Value::bool(false)),
        }
    }

    pub fn neq(&self, other: &Self, arena: &Rodeo) -> Option<Value> {
        self.eq(other, arena)?.not()
    }

    pub fn gt(&self, other: &Self) -> Option<Value> {
        match (self, other) {
            (Value::Number(num1), Value::Number(num2)) => Some(Value::bool(*num1 > *num2)),
            _ => None,
        }
    }

    pub fn ge(&self, other: &Self) -> Option<Value> {
        match (self, other) {
            (Value::Number(num1), Value::Number(num2)) => Some(Value::bool(*num1 >= *num2)),
            _ => None,
        }
    }

    pub fn lt(&self, other: &Self) -> Option<Value> {
        match (self, other) {
            (Value::Number(num1), Value::Number(num2)) => Some(Value::bool(*num1 < *num2)),
            _ => None,
        }
    }

    pub fn le(&self, other: &Self) -> Option<Value> {
        match (self, other) {
            (Value::Number(num1), Value::Number(num2)) => Some(Value::bool(*num1 <= *num2)),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Value::Nil => "<nil>",
            Value::Bool(_) => "<bool>",
            Value::Number(_) => "<number>",
            Value::String(_) => "<string>",
            Value::Object(_) => "<object>",
            Value::Function(_) => "<function>",
            Value::NativeFunction(_) => "<function>",
            Value::StringLiteral(_) => "<string>",
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Nil => Value::Nil,
            Value::Bool(b) => Value::Bool(*b),
            Value::Number(num) => Value::Number(*num),
            Value::Function(fun) => Value::Function(Rc::clone(fun)),
            Value::NativeFunction(fun) => Value::NativeFunction(Rc::clone(fun)),
            Value::String(str) => Value::String(Rc::clone(str)),
            Value::Object(obj) => Value::Object(Rc::clone(obj)),
            Value::StringLiteral(spur) => Value::StringLiteral(*spur),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Bool(b) => write!(f, "Bool({b})"),
            Value::Number(num) => write!(f, "Number({num})"),
            Value::String(str) => write!(f, "String({})", str.deref()),
            Value::Object(_) => write!(f, "Object(<dummy>)"),
            Value::Function(func) => {
                write!(f, "Function(spur|{}|)", func.name.into_inner())
            }
            Value::NativeFunction(func) => {
                write!(f, "Function(spur|{}|)", func.name.into_inner())
            }
            Value::StringLiteral(spur) => write!(f, "StringLiteral(spur|{}|)", spur.into_inner()),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Number(num) => write!(f, "{num}"),
            Value::String(str) => write!(f, "{}", str.deref()),
            Value::Object(_) => write!(f, "<object>"),
            Value::Function(func) => {
                write!(f, "<fun spur|{}|>", func.name.into_inner())
            }
            Value::NativeFunction(func) => {
                write!(f, "<fun spur|{}|>", func.name.into_inner())
            }
            Value::StringLiteral(spur) => write!(f, "StringLiteral(spur|{}|)", spur.into_inner()),
        }
    }
}

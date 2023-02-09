use std::{convert::TryFrom, fmt};

use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::field::LurkField;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum ExprTag {
    Nil = 0b0000_0000_0000_0000,
    Cons,
    Sym,
    Fun,
    Num,
    Thunk,
    Str,
    Char,
    Comm,
    U64,
    Key,
}

impl From<ExprTag> for u16 {
    fn from(val: ExprTag) -> Self {
        val as u16
    }
}

impl From<ExprTag> for u64 {
    fn from(val: ExprTag) -> Self {
        val as u64
    }
}

impl TryFrom<u16> for ExprTag {
    type Error = String;

    fn try_from(x: u16) -> Result<Self, <ExprTag as TryFrom<u16>>::Error> {
        match x {
            f if f == ExprTag::Nil as u16 => Ok(ExprTag::Nil),
            f if f == ExprTag::Cons as u16 => Ok(ExprTag::Cons),
            f if f == ExprTag::Sym as u16 => Ok(ExprTag::Sym),
            f if f == ExprTag::Fun as u16 => Ok(ExprTag::Fun),
            f if f == ExprTag::Thunk as u16 => Ok(ExprTag::Thunk),
            f if f == ExprTag::Num as u16 => Ok(ExprTag::Num),
            f if f == ExprTag::Str as u16 => Ok(ExprTag::Str),
            f if f == ExprTag::Char as u16 => Ok(ExprTag::Char),
            f if f == ExprTag::Comm as u16 => Ok(ExprTag::Comm),
            f if f == ExprTag::U64 as u16 => Ok(ExprTag::U64),
            f if f == ExprTag::Key as u16 => Ok(ExprTag::Key),
            f => Err(format!("Invalid ExprTag value: {}", f)),
        }
    }
}

impl fmt::Display for ExprTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprTag::Nil => write!(f, "nil#"),
            ExprTag::Cons => write!(f, "cons#"),
            ExprTag::Sym => write!(f, "sym#"),
            ExprTag::Fun => write!(f, "fun#"),
            ExprTag::Num => write!(f, "num#"),
            ExprTag::Thunk => write!(f, "thunk#"),
            ExprTag::Str => write!(f, "str#"),
            ExprTag::Key => write!(f, "key#"),
            ExprTag::Char => write!(f, "char#"),
            ExprTag::Comm => write!(f, "comm#"),
            ExprTag::U64 => write!(f, "u64#"),
        }
    }
}

impl ExprTag {
    pub fn from_field<F: LurkField>(f: &F) -> Option<Self> {
        Self::try_from(f.to_u16()?).ok()
    }

    pub fn as_field<F: LurkField>(&self) -> F {
        F::from(*self as u64)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ContTag {
    Outermost = 0b0001_0000_0000_0000,
    Call0,
    Call,
    Call2,
    Tail,
    Error,
    Lookup,
    Unop,
    Binop,
    Binop2,
    If,
    Let,
    LetRec,
    Dummy,
    Terminal,
    Emit,
}

impl From<ContTag> for u16 {
    fn from(val: ContTag) -> Self {
        val as u16
    }
}

impl From<ContTag> for u64 {
    fn from(val: ContTag) -> Self {
        val as u64
    }
}

impl TryFrom<u16> for ContTag {
    type Error = String;

    fn try_from(x: u16) -> Result<Self, <ContTag as TryFrom<u16>>::Error> {
        match x {
            f if f == ContTag::Outermost as u16 => Ok(ContTag::Outermost),
            f if f == ContTag::Call0 as u16 => Ok(ContTag::Call0),
            f if f == ContTag::Call as u16 => Ok(ContTag::Call),
            f if f == ContTag::Call2 as u16 => Ok(ContTag::Call2),
            f if f == ContTag::Tail as u16 => Ok(ContTag::Tail),
            f if f == ContTag::Error as u16 => Ok(ContTag::Error),
            f if f == ContTag::Lookup as u16 => Ok(ContTag::Lookup),
            f if f == ContTag::Unop as u16 => Ok(ContTag::Unop),
            f if f == ContTag::Binop as u16 => Ok(ContTag::Binop),
            f if f == ContTag::Binop2 as u16 => Ok(ContTag::Binop2),
            f if f == ContTag::If as u16 => Ok(ContTag::If),
            f if f == ContTag::Let as u16 => Ok(ContTag::Let),
            f if f == ContTag::LetRec as u16 => Ok(ContTag::LetRec),
            f if f == ContTag::Dummy as u16 => Ok(ContTag::Dummy),
            f if f == ContTag::Terminal as u16 => Ok(ContTag::Terminal),
            f if f == ContTag::Emit as u16 => Ok(ContTag::Emit),
            f => Err(format!("Invalid ContTag value: {}", f)),
        }
    }
}

impl ContTag {
    pub fn from_field<F: LurkField>(f: &F) -> Option<Self> {
        Self::try_from(f.to_u16()?).ok()
    }

    pub fn as_field<F: LurkField>(&self) -> F {
        F::from(*self as u64)
    }
}

impl fmt::Display for ContTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContTag::Outermost => write!(f, "outermost#"),
            ContTag::Call0 => write!(f, "call0#"),
            ContTag::Call => write!(f, "call#"),
            ContTag::Call2 => write!(f, "call2#"),
            ContTag::Tail => write!(f, "tail#"),
            ContTag::Error => write!(f, "error#"),
            ContTag::Lookup => write!(f, "lookup#"),
            ContTag::Unop => write!(f, "unop#"),
            ContTag::Binop => write!(f, "binop#"),
            ContTag::Binop2 => write!(f, "binop2#"),
            ContTag::If => write!(f, "if#"),
            ContTag::Let => write!(f, "let#"),
            ContTag::LetRec => write!(f, "letrec#"),
            ContTag::Dummy => write!(f, "dummy#"),
            ContTag::Terminal => write!(f, "terminal#"),
            ContTag::Emit => write!(f, "emit#"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum Op1 {
    Car = 0b0010_0000_0000_0000,
    Cdr,
    Atom,
    Emit,
    Open,
    Secret,
    Commit,
    Num,
    Comm,
    Char,
    Eval,
    U64,
}

impl From<Op1> for u16 {
    fn from(val: Op1) -> Self {
        val as u16
    }
}

impl From<Op1> for u64 {
    fn from(val: Op1) -> Self {
        val as u64
    }
}

impl TryFrom<u16> for Op1 {
    type Error = String;

    fn try_from(x: u16) -> Result<Self, <Op1 as TryFrom<u16>>::Error> {
        match x {
            f if f == Op1::Car as u16 => Ok(Op1::Car),
            f if f == Op1::Cdr as u16 => Ok(Op1::Cdr),
            f if f == Op1::Atom as u16 => Ok(Op1::Atom),
            f if f == Op1::Emit as u16 => Ok(Op1::Emit),
            f if f == Op1::Open as u16 => Ok(Op1::Open),
            f if f == Op1::Secret as u16 => Ok(Op1::Secret),
            f if f == Op1::Commit as u16 => Ok(Op1::Commit),
            f if f == Op1::Num as u16 => Ok(Op1::Num),
            f if f == Op1::Comm as u16 => Ok(Op1::Comm),
            f if f == Op1::Char as u16 => Ok(Op1::Char),
            f if f == Op1::Eval as u16 => Ok(Op1::Eval),
            f if f == Op1::U64 as u16 => Ok(Op1::U64),
            f => Err(format!("Invalid Op1 value: {}", f)),
        }
    }
}

impl Op1 {
    pub fn from_field<F: LurkField>(f: &F) -> Option<Self> {
        Self::try_from(f.to_u16()?).ok()
    }

    pub fn as_field<F: LurkField>(&self) -> F {
        F::from(*self as u64)
    }
}

impl fmt::Display for Op1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op1::Car => write!(f, "car#"),
            Op1::Cdr => write!(f, "cdr#"),
            Op1::Atom => write!(f, "atom#"),
            Op1::Emit => write!(f, "emit#"),
            Op1::Open => write!(f, "open#"),
            Op1::Secret => write!(f, "secret#"),
            Op1::Commit => write!(f, "commit#"),
            Op1::Num => write!(f, "num#"),
            Op1::Comm => write!(f, "comm#"),
            Op1::Char => write!(f, "char#"),
            Op1::Eval => write!(f, "eval#"),
            Op1::U64 => write!(f, "u64#"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum Op2 {
    Sum = 0b0011_0000_0000_0000,
    Diff,
    Product,
    Quotient,
    Equal,
    NumEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Cons,
    StrCons,
    Begin,
    Hide,
    Modulo,
    Eval,
}

impl From<Op2> for u16 {
    fn from(val: Op2) -> Self {
        val as u16
    }
}

impl From<Op2> for u64 {
    fn from(val: Op2) -> Self {
        val as u64
    }
}

impl TryFrom<u16> for Op2 {
    type Error = String;

    fn try_from(x: u16) -> Result<Self, <Op2 as TryFrom<u16>>::Error> {
        match x {
            f if f == Op2::Sum as u16 => Ok(Op2::Sum),
            f if f == Op2::Diff as u16 => Ok(Op2::Diff),
            f if f == Op2::Product as u16 => Ok(Op2::Product),
            f if f == Op2::Quotient as u16 => Ok(Op2::Quotient),
            f if f == Op2::Equal as u16 => Ok(Op2::Equal),
            f if f == Op2::NumEqual as u16 => Ok(Op2::NumEqual),
            f if f == Op2::Less as u16 => Ok(Op2::Less),
            f if f == Op2::Greater as u16 => Ok(Op2::Greater),
            f if f == Op2::LessEqual as u16 => Ok(Op2::LessEqual),
            f if f == Op2::GreaterEqual as u16 => Ok(Op2::GreaterEqual),
            f if f == Op2::Cons as u16 => Ok(Op2::Cons),
            f if f == Op2::StrCons as u16 => Ok(Op2::StrCons),
            f if f == Op2::Begin as u16 => Ok(Op2::Begin),
            f if f == Op2::Hide as u16 => Ok(Op2::Hide),
            f if f == Op2::Modulo as u16 => Ok(Op2::Modulo),
            f if f == Op2::Eval as u16 => Ok(Op2::Eval),
            f => Err(format!("Invalid Op2 value: {}", f)),
        }
    }
}

impl Op2 {
    pub fn rom_field<F: LurkField>(f: &F) -> Option<Self> {
        Self::try_from(f.to_u16()?).ok()
    }

    pub fn as_field<F: From<u64> + ff::Field>(&self) -> F {
        F::from(*self as u64)
    }

    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Op2::Sum
                | Op2::Diff
                | Op2::Product
                | Op2::Quotient
                | Op2::Less
                | Op2::Greater
                | Op2::LessEqual
                | Op2::GreaterEqual
                | Op2::NumEqual
                | Op2::Modulo
        )
    }
}

impl fmt::Display for Op2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op2::Sum => write!(f, "sum#"),
            Op2::Diff => write!(f, "diff#"),
            Op2::Product => write!(f, "product#"),
            Op2::Quotient => write!(f, "quotient#"),
            Op2::Equal => write!(f, "equal#"),
            Op2::NumEqual => write!(f, "numequal#"),
            Op2::Less => write!(f, "less#"),
            Op2::Greater => write!(f, "greater"),
            Op2::LessEqual => write!(f, "lessequal#"),
            Op2::GreaterEqual => write!(f, "greaterequal#"),
            Op2::Cons => write!(f, "cons"),
            Op2::StrCons => write!(f, "strcons#"),
            Op2::Begin => write!(f, "begin"),
            Op2::Hide => write!(f, "hide"),
            Op2::Modulo => write!(f, "modulo"),
            Op2::Eval => write!(f, "eval#"),
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use proptest::prelude::*;

    impl Arbitrary for ExprTag {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            prop_oneof!(
                Just(ExprTag::Nil),
                Just(ExprTag::Cons),
                Just(ExprTag::Sym),
                Just(ExprTag::Fun),
                Just(ExprTag::Num),
                Just(ExprTag::Thunk),
                Just(ExprTag::Str),
                Just(ExprTag::Char),
                Just(ExprTag::Comm),
                Just(ExprTag::U64),
                Just(ExprTag::Key),
            )
            .boxed()
        }
    }

    impl Arbitrary for ContTag {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            prop_oneof!(
                Just(ContTag::Outermost),
                Just(ContTag::Call),
                Just(ContTag::Call2),
                Just(ContTag::Tail),
                Just(ContTag::Error),
                Just(ContTag::Lookup),
                Just(ContTag::Unop),
                Just(ContTag::Binop),
                Just(ContTag::Binop2),
                Just(ContTag::If),
                Just(ContTag::Let),
                Just(ContTag::LetRec),
                Just(ContTag::Dummy),
                Just(ContTag::Terminal),
                Just(ContTag::Emit),
            )
            .boxed()
        }
    }

    impl Arbitrary for Op1 {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            prop_oneof!(
                Just(Op1::Car),
                Just(Op1::Cdr),
                Just(Op1::Atom),
                Just(Op1::Emit),
                Just(Op1::Secret),
                Just(Op1::Commit),
                Just(Op1::Num),
                Just(Op1::Comm),
                Just(Op1::Char),
                Just(Op1::Eval),
            )
            .boxed()
        }
    }

    impl Arbitrary for Op2 {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            prop_oneof!(
                Just(Op2::Sum),
                Just(Op2::Diff),
                Just(Op2::Product),
                Just(Op2::Quotient),
                Just(Op2::Equal),
                Just(Op2::NumEqual),
                Just(Op2::Less),
                Just(Op2::Greater),
                Just(Op2::LessEqual),
                Just(Op2::GreaterEqual),
                Just(Op2::Cons),
                Just(Op2::StrCons),
                Just(Op2::Begin),
                Just(Op2::Hide),
                Just(Op2::Eval)
            )
            .boxed()
        }
    }

    proptest! {
    #[test]
    fn prop_expr_tag_u16(x in any::<ExprTag>()) {
        let x_u16: u16 = x.into();
        let x2 = ExprTag::try_from(x_u16).expect("read ExprTag from u16");
        assert_eq!(x, x2);
    }
    }

    proptest! {
    #[test]
    fn prop_cont_tag_u16(x in any::<ContTag>()) {
        let x_u16: u16 = x.into();
        let x2 = ContTag::try_from(x_u16).expect("read ContTag from u16");
        assert_eq!(x, x2)
    }
    }

    proptest! {
    #[test]
    fn prop_op1_u16(x in any::<Op1>()) {
        let x_u16: u16 = x.into();
        let x2 = Op1::try_from(x_u16).expect("read Op1 from u16");
        assert_eq!(x, x2)
    }
    }

    proptest! {
    #[test]
    fn prop_op2_u16(x in any::<Op2>()) {
        let x_u16: u16 = x.into();
        let x2 = Op2::try_from(x_u16).expect("read Op2 from u16");
        assert_eq!(x, x2)
    }
    }
}
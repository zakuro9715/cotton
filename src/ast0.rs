#[derive(Debug, PartialEq, Eq)]
pub struct Ast<'a> {
    pub decls: Vec<Decl<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Decl<'a> {
    Variable(VariableDecl<'a>),
    Data(DataDecl<'a>),
    Precedence(OperatorPrecedence<'a>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VariableDecl<'a> {
    pub identifier: &'a str,
    pub type_annotation: Option<(InfixTypeSequence<'a>, Forall<'a>)>,
    pub value: OpSequence<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type<'a> {
    Identifier(&'a str),
    Paren(InfixTypeSequence<'a>),
}

pub type InfixTypeSequence<'a> = Vec<OpSequenceUnit<'a, Type<'a>>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OpSequenceUnit<'a, T> {
    Operand(T),
    Operator(&'a str),
    Apply(T),
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Forall<'a> {
    pub type_variable_names: Vec<&'a str>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DataDecl<'a> {
    pub name: &'a str,
    pub field_len: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr<'a> {
    Lambda(Vec<FnArm<'a>>),
    Number(&'a str),
    StrLiteral(&'a str),
    Identifier(&'a str),
    Decl(Box<VariableDecl<'a>>),
    Paren(OpSequence<'a>),
}

pub type OpSequence<'a> = Vec<OpSequenceUnit<'a, Expr<'a>>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnArm<'a> {
    pub pattern: Vec<InfixConstructorSequence<'a>>,
    pub pattern_type: Vec<Option<InfixTypeSequence<'a>>>,
    pub exprs: Vec<OpSequence<'a>>,
}

pub type InfixConstructorSequence<'a> =
    Vec<OpSequenceUnit<'a, Pattern<'a>>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pattern<'a> {
    Number(&'a str),
    StrLiteral(&'a str),
    Constructor(&'a str, Vec<Pattern<'a>>),
    Binder(&'a str),
    Underscore,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OperatorPrecedence<'a> {
    pub name: &'a str,
    pub associativity: Associativity,
    pub precedence: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Associativity {
    Left,
    Right,
    UnaryLeft,
}

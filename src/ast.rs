// pub struct Function {}

#[derive(Debug)]
pub struct Program<'a>(Expr<'a>);

#[derive(Debug)]
pub enum Expr<'a> {
    Lit(LitExpr),
    Var(Ident<'a>),
}

#[derive(Debug)]
pub enum LitExpr {
    True,
    False,
}

#[derive(Debug)]
pub enum Ty<'a> {
    Unit,
    Named(Ident<'a>),
}

pub type Ident<'a> = &'a str;

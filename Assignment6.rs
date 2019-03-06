use std::collections::HashMap;

enum ExprC {
    NumC { n: f32 },
    IdC { s: char },
    StringC { s: String },
    IfC { cond: Box<ExprC>, thenn: Box<ExprC>, elsee: Box<ExprC> },
    LamC { args: Vec<char>, body: Box<ExprC> },
    AppC { func: Box<ExprC>, args: Vec<Box<ExprC>> },
}

enum Value {
    NumV { n: f32 },
    BoolV { b: bool },
    StringV { s: String }, 
    CloV { args: Vec<char>, body: Box<ExprC>, env: Environment },
    PrimV { op: char },
}

type Environment = HashMap<char, Value>;

fn main() {

}
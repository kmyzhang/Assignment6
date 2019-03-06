use std::collections::HashMap;

enum ExprC {
    NumC { n: f32 },
    IdC { s: String },
    StringC { s: String },
    IfC { cond: Box<ExprC>, thenn: Box<ExprC>, elsee: Box<ExprC> },
    LamC { args: Vec<String>, body: Box<ExprC> },
    AppC { func: Box<ExprC>, args: Vec<Box<ExprC>> },
}

enum Value {
    NumV { n: f32 },
    BoolV { b: bool },
    StringV { s: String }, 
    CloV { args: Vec<String>, body: Box<ExprC>, env: Environment },
    PrimV { op: String },
}

type Environment = HashMap<String, Value>;
fn make_environment() -> Environment {
    let mut env = Environment::new();
    env.insert(String::from("+"), Value::PrimV { op: "+".to_string()});
    env.insert(String::from("-"), Value::PrimV { op: "-".to_string()});
    env.insert(String::from("*"), Value::PrimV { op: "*".to_string()});
    env.insert(String::from("/"), Value::PrimV { op: "/".to_string()});
    env.insert(String::from("<="), Value::PrimV { op: "<=".to_string()});
    env.insert(String::from("equal?"), Value::PrimV { op: "equal?".to_string()});
    env.insert(String::from("true"), Value::BoolV { b: true});
    env.insert(String::from("true"), Value::BoolV { b: false});
    return env;
}

fn main() {
    let environment = make_environment();
}
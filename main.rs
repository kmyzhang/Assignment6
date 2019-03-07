use std::collections::HashMap;
use core::borrow::Borrow;
use crate as name;

//DATA STRUCTURES
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

//Environment
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

//returns a string type of a value
fn serialize(v: Value) -> String {
    match v {
        Value::NumV{ n } => return n.to_string(),
        Value::StringV{ s } => return s.to_string(),
        Value::BoolV{ b } => return b.to_string(),
        Value::CloV{ args, body, env} => return "#<procedure>".to_string(),
        Value::PrimV{ op } => return "#<primop>".to_string(),
    }
}

/*
    This function accepts an ExprC and an Environment and returns the mapped value
    of the String stored in the Environment if the ExprC is and IdC.
    Else, this function will raise an error.
*/
fn env_lookup(idc: ExprC, env: Environment) -> &Value{
    let mut val:Option<&Value> = hash_look(idc, env);
    match val{
        Option::Some{t:&Value} => return val.unwrap(),
    }
}

/*
    This is the helper function to env_lookup. It also accepts an ExprC and an Environment.
    If the ExprC is not an IdC, an error is raised. Else, it returns the Option<&Value> stored
    returned by Environment.
*/
fn hash_look(idc: ExprC, env: Environment) -> Option<&Value> {
    match idc{
        ExprC::IdC{s} => env.get(&s),
    }
}



fn main() {
    let environment = make_environment();
    env_lookup(ExprC::IdC{ s: "+".to_string()}, environment);
    //TEST CASES
    //Data structures
    assert_eq!(serialize(Value::NumV{n:1.5}), "1.5");
    assert_eq!(serialize(Value::StringV{s:"test".to_string()}), "test");
    assert_eq!(serialize(Value::BoolV{b:true}), "true");
    assert_eq!(serialize(Value::PrimV{op:"+".to_string()}), "#<primop>");


    assert_eq!(interp(ExprC::NumC {n:1.5}), Value::NumV{n:1.5});
    assert_eq!(interp(ExprC::StringC {s:"test".to_string()}), Value::StringV{s:"test".to_string()});
    assert_eq!(interp(ExprC::IfC{cond:Box::ExprC::IdC{s:"true".to_string()},
        thenn:Box::ExprC::NumC{n:1.5}, elsee:Box::ExprC::NumC{n:2}}, environment), Value::NumV{n:1.5});


}
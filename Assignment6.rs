use std::collections::HashMap;
use std::io;

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

fn interp(ex: ExprC, env: Environment) -> Value {
    match ex {
        ExprC::NumC{n} => return Value::NumV{n:n},
        ExprC::StringC{s} => return Value::StringV{s:s},
        ExprC::LamC{args, body} => return Value::CloV{args, body, env},
        ExprC::IfC{cond, thenn, elsee} => match interp(unbox(cond), env) {
            Value::BoolV{b} => if b {
                    interp(unbox(thenn), env)
                } else {
                    interp(unbox(elsee), env)
                },
            _ => panic!("invalid type"),
        }
        ExprC::LamC{args, body} => Value::StringV{s:"not done".to_string()},
        ExprC::AppC{func, args} => Value::StringV{s:"not done".to_string()},
        ExprC::IdC{s} => env_lookup(s,env),
        _ => panic!("invalid type")
    }
}

fn env_lookup(s: String, env: Environment) -> Value{
    let mut val:Option<&Value> = hash_look(s, env);
    match val{
        Option::Some{t:&Value} => return val.unwrap(),
    }
}

fn hash_look(s: String, env: Environment) -> Value {
    env.get(&s),
}

/*fn hash_look(s: String, env: Environment) -> Option<&Value> {
    match idc{
        ExprC::IdC{s} => env.get(&s),
    }
}*/

fn unbox<T>(value: Box<T>) -> T {
    *value
}

fn main() {
    let environment = make_environment();
    let mut arg_vec = Vec::new();
    arg_vec.push("a".to_string());

    //TEST CASES

    //Data structures
    assert_eq!(serialize(Value::NumV{n:1.5}), "1.5");
    assert_eq!(serialize(Value::StringV{s:"test".to_string()}), "test");
    assert_eq!(serialize(Value::BoolV{b:true}), "true");
    assert_eq!(serialize(Value::PrimV{op:"+".to_string()}), "#<primop>");
    assert_eq!(interp(ExprC::IdC{s:"+".to_string()}, environment), Value::PrimV{ op: "+".to_string()});


    //assert_eq!(serialize(Value::CloV{["a".to_string(), "b".to_string()], body: Box::new(Value::NumV{n:1.0}), env: make_environment()}), "#<procedure>");
}
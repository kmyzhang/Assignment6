use std::collections::HashMap;

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
// make_environment : (-> Environment)
// creates the initial environment
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

// serialize : (Value -> String)
// returns a string type of a value
fn serialize(v: Value) -> String {
    match v {
        Value::NumV{ n } => return n.to_string(),
        Value::StringV{ s } => return s.to_string(),
        Value::BoolV{ b } => return b.to_string(),
        Value::CloV{ args, body, env} => return "#<procedure>".to_string(),
        Value::PrimV{ op } => return "#<primop>".to_string(),
    }
}

// valid_id : (String -> Boolean)
// this function takes in an ID as a string and verifies that it is valid
fn valid_id(s : String) -> bool {
    match s.as_str() {
        "=" => false,
        "if" => false,
        "var" => false,
        "lam" => false,
        _ => true,
    }
}

// this is the main function which creates an initial environment and runs
// the test cases
fn main() {
    let environment = make_environment();

    //TEST CASES

    //Data structures
    assert_eq!(serialize(Value::NumV{n:1.5}), "1.5");
    assert_eq!(serialize(Value::StringV{s:"test".to_string()}), "test");
    assert_eq!(serialize(Value::BoolV{b:true}), "true");
    assert_eq!(serialize(Value::PrimV{op:"+".to_string()}), "#<primop>");
    //valid id test cases
    assert_eq!(valid_id("+".to_string()), true);
    assert_eq!(valid_id("=".to_string()), false);
    assert_eq!(valid_id("var".to_string()), false);
    assert_eq!(valid_id("if".to_string()), false);
    assert_eq!(valid_id("lam".to_string()), false);
}

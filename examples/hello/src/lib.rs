use wasm_minimal_protocol::*;

initiate_protocol!();

#[wasm_func]
pub fn hello() -> String {
    String::from("Hello from wasm!!!")
}

#[wasm_func]
pub fn double_it(arg: &str) -> String {
    format!("{}.{}", arg, arg)
}

#[wasm_func]
pub fn concatenate(arg1: &str, arg2: &str) -> String {
    format!("{}*{}", arg1, arg2)
}

#[wasm_func]
pub fn shuffle(arg1: &str, arg2: &str, arg3: &str) -> String {
    format!("{}-{}-{}", arg3, arg1, arg2)
}

#[wasm_func]
pub fn returns_ok() -> Result<String, String> {
    Ok(String::from("This is an `Ok`"))
}

#[wasm_func]
pub fn returns_err() -> Result<String, String> {
    Err(String::from("This is an `Err`"))
}

#[wasm_func]
pub fn will_panic() -> String {
    panic!("unconditional panic")
}

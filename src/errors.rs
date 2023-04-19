//Serialize的作用是将结构体转换为json字符串，Deserialize的作用是将json字符串转换为结构体
use serde::Serialize;

#[derive(Debug,Serialize)]
pub enum MyError{
    SomeError(String),
}

impl From<String> for MyError{
    fn from(err:String)->Self{
        MyError::SomeError(err)
    }
}

impl From<wasm_bindgen::JsValue> for MyError{
    fn from(err:wasm_bindgen::JsValue)->Self{
        MyError::SomeError(err.as_string().unwrap())
    }
}

use super::super::errors::MyError;
use serde::{Deserialize,Serialize};
use chrono::{NaiveDateTime};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request,RequestInit,RequestMode,Response};

#[derive(Debug,Serialize,Deserialize)]
pub struct Course{
    pub teacher_id:i32,
    pub id:u64,
    pub name:String,
    pub time:NaiveDateTime,

    pub description:Option<String>,
    pub format:Option<String>,
    pub structure:Option<String>,
    pub duration:Option<String>,
    pub price:Option<i32>,
    pub language:Option<String>,
    pub level:Option<String>,
}

pub async fn get_courses_by_teacher(teacher_id:i32)
    -> Result<Vec<Course>,MyError>{
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let url=format!("http://localhost:3001/courses/{}",teacher_id);

        let request=Request::new_with_str_and_init(&url,&opts)?;
        //表示浏览器能接受服务器返回的类型是json
        request.headers().set("Accept","application/json")?;
        //ok_or()方法，将option类型转换为Result类型,Some(v)->Ok(v),None->Err(e),e为传入的参数
        let window=web_sys::window().ok_or("No window exists!".to_string())?;
        //fetch_with_request()方法返回一个Promise对象，需要使用JsFuture::from()方法处理异步，将其转换为JsValue类型
        let resp_value=JsFuture::from(window.fetch_with_request(&request)).await?;

        assert!(resp_value.is_instance_of::<Response>());
        //dyn_into()方法将JsValue类型转换为Response类型
        let resp:Response=resp_value.dyn_into().unwrap();
        //json()方法返回一个Promise对象，需要使用JsFuture::from()方法处理异步，将其转换为JsValue类型
        //JsValue表示是一个js的值，可以是任何类型，比如字符串、数字、数组、对象等
        let json=JsFuture::from(resp.json()?).await?;

        let courses:Vec<Course>=json.into_serde().unwrap();

        Ok(courses)
    }

pub async fn delete_course(teacher_id:i32,course_id:u64)->(){
    let mut opts=RequestInit::new();
    opts.method("DELETE");
    opts.mode(RequestMode::Cors);

    let url=format!("http://localhost:3001/courses/{}/{}",teacher_id,course_id);
    let request=Request::new_with_str_and_init(&url,&opts).unwrap();
    request.headers().set("Accept","application/json").unwrap();

    let window=web_sys::window().unwrap();
    let _resp_value=JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

    assert!(_resp_value.is_instance_of::<Response>());

    let _resp:Response=_resp_value.dyn_into().unwrap();
    let json=JsFuture::from(_resp.json().unwrap()).await.unwrap();
    let _courses:Course=json.into_serde().unwrap();
}

use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn add_course(name:String,description:String)->Result<Promise,JsValue>{
    let mut opts=RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    let str_json=format!(
        r#"
        {{
            "teacher_id":1,
            "name":"{}",
            "description":"{}"
        }}
        "#,
        name,description
    );
    opts.body(Some(&JsValue::from_str(str_json.as_str())));

    let url="http://localhost:3001/courses/";

    let request=Request::new_with_str_and_init(&url,&opts)?;
    request.headers().set("Accept","application/json")?;
    request.headers().set("Content-Type","application/json")?;

    let window=web_sys::window().ok_or("No window exists!".to_string())?;
    let resp_value=JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());

    let resp:Response=resp_value.dyn_into().unwrap();
    //resp.json()返回一个Promise对象，这个Promise对象可以直接传给js进行处理
    Ok(resp.json()?)
}
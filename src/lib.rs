mod utils;
//引入内置内容
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
//C表示此处使用的是application binary interface，即二进制接口
extern "C"{
    //声明浏览器中的alert函数，此处写好alert函数签名
    fn alert(s: &str);
    //声明浏览器中的confirm函数，此处写好confirm函数签名
    fn confirm(s: &str) -> bool;

    //声明浏览器中console命名空间的console.log函数，此处写好console.log函数签名
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

//为了让下面写的greet函数可以被js调用，需要加上#[wasm_bindgen]注解
#[wasm_bindgen]
pub fn greet(s: &str) {
    //rust中调用alert函数
    alert("Hello, wasm-client!");
}

pub mod models;
pub mod errors;

use models::course::{Course, delete_course,get_courses_by_teacher};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::*;
use web_sys::{Document, HtmlButtonElement};

//表示下面的函数是主函数，在项目启动时会自动调用,可以是异步函数也可以是同步函数
#[wasm_bindgen(start)]
//返回值是Result类型，如果出错，返回JsValue类型的错误信息
pub async fn main()->Result<(),JsValue>{
    let window=web_sys::window().expect("No global window exists!");
    let document=window.document().expect("No global document exists!");

    let left_tbody=document.get_element_by_id("left-tbody").expect("left div not exists!");

    let course:Vec<Course>=models::course::get_courses_by_teacher(1).await.unwrap();
    // log(format!("hahaha:{:?}",course).to_string().as_str());
    for c in course.iter(){
        let tr=document.create_element("tr")?;
        tr.set_attribute("id", format!("tr-{}",c.id).as_str())?;
        let td=document.create_element("td")?;
        //set_text_content()传入的参数是Option类型，所以需要使用Some()包装一下
        td.set_text_content(Some(format!("{}",c.id).as_str()));
        tr.append_child(&td)?;

        let td=document.create_element("td")?;
        td.set_text_content(Some(c.name.as_str()));
        tr.append_child(&td)?;

        let td=document.create_element("td")?;
        td.set_text_content(Some(c.time.format("%Y-%m-%d").to_string().as_str()));
        tr.append_child(&td)?;

        let td=document.create_element("td")?;
        if let Some(description)=c.description.clone(){
            td.set_text_content(Some(description.as_str()));
        }
        tr.append_child(&td)?;

        let td=document.create_element("td")?;
        let cid=c.id;
        let btn:HtmlButtonElement=document
                                    .create_element("button")?
                                    .dyn_into::<HtmlButtonElement>()?;
        let click_closure=Closure::wrap(Box::new(move |_event:web_sys::MouseEvent|{
            let r=confirm(format!("确定删除ID为{}的课程吗？",cid).as_str());
            match r {
                true=>{
                //delete_course为异步函数，目前使用异步闭包不稳定，所以使用spawn_local
                //此处delete_course没有使用await，返回的是一个Future对象，用spawn_local包装之后这个Future对象就会在当前线程中执行
                //是用这种方式可以在普通闭包里执行异步函数
                    spawn_local(delete_course(1, cid));
                    alert("删除成功！");
                    web_sys::window().unwrap().location().reload().unwrap();
                }
                _=>{}
            }
        }) as Box<dyn Fn(_)>);

        btn.add_event_listener_with_callback("click",click_closure.as_ref().unchecked_ref())?;
        click_closure.forget();

        btn.set_attribute("class", "btn btn-danger btn-sm")?;
        btn.set_text_content(Some("删除"));
        td.append_child(&btn)?;
        tr.append_child(&td)?;

        left_tbody.append_child(&tr)?;
    }

    Ok(())
}

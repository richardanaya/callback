use std::collections::HashMap;

pub static mut CALLBACKS: Option<HashMap<i32, Box<Fn(i32)>>> = None;


pub fn get_callback_router() -> &'static mut std::collections::HashMap<i32, std::boxed::Box<(dyn std::ops::Fn(i32) + 'static)>> {
    unsafe {
        if CALLBACKS.is_none() {
            CALLBACKS = Some(HashMap::new());
        }
        CALLBACKS.as_mut().unwrap()
    }
}

pub fn route_callback(callback_id: i32, event: i32) {
    let h = get_callback_router().get(&callback_id);
    if h.is_some() {
        h.unwrap()(event);
    }
}


pub fn add_callback<F>(cb: i32,f: Box<F>)
where
    F: Fn(i32) + 'static,
{
    get_callback_router().insert(cb, f);
}

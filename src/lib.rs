#![no_std]
extern crate alloc;
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};
use spin::Mutex;
use web_common::*;

#[macro_use]
extern crate lazy_static;

pub enum CallbackHandler {
    Callback0(Box<dyn FnMut() -> () + Send + 'static>),
    Callback1(Box<dyn FnMut(JSValue) -> () + Send + 'static>),
    Callback2(Box<dyn FnMut(JSValue, JSValue) -> () + Send + 'static>),
    Callback3(Box<dyn FnMut(JSValue, JSValue, JSValue) -> () + Send + 'static>),
    Callback4(Box<dyn FnMut(JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static>),
    Callback5(Box<dyn FnMut(JSValue, JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static>),
    Callback6(
        Box<dyn FnMut(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static>,
    ),
    Callback7(
        Box<
            dyn FnMut(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> ()
                + Send
                + 'static,
        >,
    ),
    Callback8(
        Box<
            dyn FnMut(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> ()
                + Send
                + 'static,
        >,
    ),
    Callback9(
        Box<
            dyn FnMut(
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                ) -> ()
                + Send
                + 'static,
        >,
    ),
    Callback10(
        Box<
            dyn FnMut(
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                ) -> ()
                + Send
                + 'static,
        >,
    ),
}

type CallbackHandle = u32;

pub struct CallbackManager {
    cur_id: CallbackHandle,
    pub keys: Vec<CallbackHandle>,
    pub handlers: Vec<Arc<Mutex<CallbackHandler>>>,
}

lazy_static! {
    static ref INSTANCE: Mutex<CallbackManager> = {
        Mutex::new(CallbackManager {
            cur_id: 0,
            keys: Vec::new(),
            handlers: Vec::new(),
        })
    };
}

pub fn get_callbacks() -> &'static Mutex<CallbackManager> {
    &INSTANCE
}

pub fn get_callback(id: CallbackHandle) -> Option<Arc<Mutex<CallbackHandler>>> {
    let cbs = get_callbacks().lock();
    let index = cbs.keys.iter().position(|&r| r == id);
    if let Some(i) = index {
        let handler_ref = cbs.handlers.get(i).unwrap().clone();
        core::mem::drop(cbs);
        Some(handler_ref)
    } else {
        None
    }
}

pub fn remove_callback(id: CallbackHandle) {
    let mut cbs = get_callbacks().lock();
    let index = cbs.keys.iter().position(|&r| r == id);
    if let Some(i) = index {
        cbs.keys.remove(i);
        cbs.handlers.remove(i);
    }
}

fn create_callback(cb: CallbackHandler) -> JSValue {
    let mut h = get_callbacks().lock();
    h.cur_id += 1;
    let id = h.cur_id;
    h.keys.push(id);
    h.handlers.push(Arc::new(Mutex::new(cb)));
    return id as JSValue;
}

pub fn create_callback_0(cb: Box<dyn FnMut() -> () + Send + 'static>) -> JSValue {
    create_callback(CallbackHandler::Callback0(cb))
}

pub fn create_callback_1(cb: Box<dyn FnMut(JSValue) -> () + Send + 'static>) -> JSValue {
    create_callback(CallbackHandler::Callback1(cb))
}

pub fn create_callback_2(cb: Box<dyn FnMut(JSValue, JSValue) -> () + Send + 'static>) -> JSValue {
    create_callback(CallbackHandler::Callback2(cb))
}

pub fn create_callback_3(
    cb: Box<dyn FnMut(JSValue, JSValue, JSValue) -> () + Send + 'static>,
) -> JSValue {
    create_callback(CallbackHandler::Callback3(cb))
}

pub fn create_callback_4(
    cb: Box<dyn FnMut(JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static>,
) -> JSValue {
    create_callback(CallbackHandler::Callback4(cb))
}

pub fn create_callback_5(
    cb: Box<dyn FnMut(JSValue, JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static>,
) -> JSValue {
    create_callback(CallbackHandler::Callback5(cb))
}

pub fn create_callback_6(
    cb: Box<dyn FnMut(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static>,
) -> JSValue {
    create_callback(CallbackHandler::Callback6(cb))
}

pub fn create_callback_7(
    cb: Box<
        dyn FnMut(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> ()
            + Send
            + 'static,
    >,
) -> JSValue {
    create_callback(CallbackHandler::Callback7(cb))
}

pub fn create_callback_8(
    cb: Box<
        dyn FnMut(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> ()
            + Send
            + 'static,
    >,
) -> JSValue {
    create_callback(CallbackHandler::Callback8(cb))
}

pub fn create_callback_9(
    cb: Box<
        dyn FnMut(
                JSValue,
                JSValue,
                JSValue,
                JSValue,
                JSValue,
                JSValue,
                JSValue,
                JSValue,
                JSValue,
            ) -> ()
            + Send
            + 'static,
    >,
) -> JSValue {
    create_callback(CallbackHandler::Callback9(cb))
}

pub fn create_callback_10(
    cb: Box<
        dyn FnMut(
                JSValue,
                JSValue,
                JSValue,
                JSValue,
                JSValue,
                JSValue,
                JSValue,
                JSValue,
                JSValue,
                JSValue,
            ) -> ()
            + Send
            + 'static,
    >,
) -> JSValue {
    create_callback(CallbackHandler::Callback10(cb))
}

pub struct CallbackFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// Shared state between the future and the waiting thread
struct SharedState {
    completed: bool,
    waker: Option<Waker>,
    result: JSValue,
}

impl Future for CallbackFuture {
    type Output = JSValue;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture {
    pub fn new() -> (Self, JSValue) {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
            result: UNDEFINED,
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback1(Box::new(move |v: JSValue| {
            let mut shared_state = thread_shared_state.lock();
            shared_state.completed = true;
            shared_state.result = v;
            if let Some(waker) = shared_state.waker.take() {
                core::mem::drop(shared_state);
                waker.wake()
            }
        })));
        (CallbackFuture { shared_state }, id as JSValue)
    }
}

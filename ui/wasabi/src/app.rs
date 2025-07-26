use alloc::{rc::Rc, string::ToString};
use core::cell::RefCell;
use noli::window::Window;
use saba_core::{
    browser::Browser,
    constants::{WHITE, WINDOW_HEIGHT, WINDOW_INIT_X_POS, WINDOW_INIT_Y_POS, WINDOW_WIDTH},
};

#[derive(Debug)]
pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    window: Window,
}

impl WasabiUI {
    pub fn new(browser: Rc<RefCell<Browser>>) -> Self {
        Self {
            browser,
            window: Window::new(
                "saba".to_string(),
                WHITE,
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            )
            .unwrap(),
        }
    }
}

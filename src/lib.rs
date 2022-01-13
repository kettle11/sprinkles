pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space,
}

pub enum Event {
    KeyDown(Key),
    Draw,
}

extern "C" {
    fn js_clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32);
    fn js_draw_rectangle(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    );
}

pub struct Context {}

impl Context {
    pub fn clear_screen_to_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe { js_clear_screen_to_color(red, green, blue, alpha) }
    }

    pub fn draw_rectangle(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    ) {
        unsafe {
            js_draw_rectangle(x, y, width, height, red, green, blue, alpha);
        }
    }
}

thread_local! {
    pub static EVENT_HANDLER_AND_CONTEXT: std::cell::RefCell<(Box<dyn FnMut(&mut Context, Event)>, Context)> = std::cell::RefCell::new((Box::new(|_, _|{}), Context {}));
}

pub fn set_event_handler(function: impl FnMut(&mut Context, Event) + 'static) {
    EVENT_HANDLER_AND_CONTEXT.with(|event_handler| {
        event_handler.borrow_mut().0 = Box::new(function);
    });
}

#[no_mangle]
pub extern "C" fn key_pressed(value: usize) {
    let key = match value {
        1 => Key::Left,
        2 => Key::Right,
        3 => Key::Up,
        4 => Key::Down,
        5 => Key::Space,
        _ => return,
    };

    send_event(Event::KeyDown(key));
}

#[no_mangle]
pub extern "C" fn animate() {
    send_event(Event::Draw);
}

fn send_event(event: Event) {
    EVENT_HANDLER_AND_CONTEXT.with(|event_handler_and_context| {
        let mut borrow = event_handler_and_context.borrow_mut();
        let (event_handler, context) = &mut *borrow;
        (event_handler)(context, event)
    })
}

#[derive(Debug, Default)]
pub struct State {
    value: i32,
}

impl State {
    pub fn print(&self) {
        println!("Current state value: {}", self.value);
    }
}

#[no_mangle]
pub static mut s_field: i32 = 9001;

#[no_mangle]
pub fn initialize() -> State {
    State::default()
}

#[no_mangle]
pub fn update(state: &mut State) {
    state.value = state.value.wrapping_add(unsafe { s_field });
}

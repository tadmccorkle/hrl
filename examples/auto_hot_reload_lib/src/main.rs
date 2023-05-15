use std::{
    env::consts::{DLL_PREFIX, DLL_SUFFIX},
    io,
    path::PathBuf,
};

use example_lib::State;

#[cfg(feature = "hot_reload")]
use hot_reload::library::auto::AutoHotReloadLibrary;

struct App {
    state: State,

    #[cfg(feature = "hot_reload")]
    lib: AutoHotReloadLibrary,
}

impl App {
    fn initialize() -> Self {
        #[cfg(not(feature = "hot_reload"))]
        {
            let state = example_lib::initialize();
            Self { state }
        }

        #[cfg(feature = "hot_reload")]
        {
            use hot_reload::call_auto_hrl_symbol;

            let path: PathBuf = [
                "target",
                #[cfg(debug_assertions)]
                "debug",
                #[cfg(not(debug_assertions))]
                "release",
                format!("{}example_lib{}", DLL_PREFIX, DLL_SUFFIX).as_str(),
            ]
            .iter()
            .collect();

            let lib = AutoHotReloadLibrary::load(&path).expect("failed to load library");
            let state = call_auto_hrl_symbol!(lib, initialize(): unsafe extern "C" fn() -> State)
                .expect("failed to initialize lib");

            Self { state, lib }
        }
    }

    fn update(&mut self) {
        #[cfg(not(feature = "hot_reload"))]
        {
            example_lib::update(state);
        }

        #[cfg(feature = "hot_reload")]
        {
            use hot_reload::call_auto_hrl_symbol;

            call_auto_hrl_symbol!(
                self.lib,
                update(&mut self.state): unsafe extern "C" fn(&mut State)
            )
            .expect("failed to update");
        }
    }

    fn get(&self) -> i32 {
        #[cfg(not(feature = "hot_reload"))]
        {
            example_lib::s_field
        }

        #[cfg(feature = "hot_reload")]
        {
            use hot_reload::auto_hrl_symbol_value;

            auto_hrl_symbol_value!(self.lib, s_field: i32).expect("failed to get static field")
        }
    }

    fn set(&self, delta: i32) {
        #[cfg(not(feature = "hot_reload"))]
        {
            example_lib::s_field += delta;
        }

        #[cfg(feature = "hot_reload")]
        {
            use hot_reload::set_auto_hrl_symbol_value;

            set_auto_hrl_symbol_value!(self.lib, s_field: i32 = *s_field + delta)
                .expect("failed to set static field");
        }
    }
}

fn main() {
    let mut app = App::initialize();

    println!("input '+'/'-' to increment/decrement `s_field` and update, 'q' to quit, anything else to only update");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        match &input.trim().to_lowercase()[..] {
            "+" => app.set(1),
            "-" => app.set(-1),
            "q" => break,
            _ => (),
        };

        println!("Update with `s_field` = {}", app.get());
        app.update();
        app.state.print();
    }
}

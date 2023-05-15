use std::{
    env::consts::{DLL_PREFIX, DLL_SUFFIX},
    io,
    path::PathBuf,
};

use example_lib::State;

#[cfg(feature = "hot_reload")]
use hot_reload::library::manual::HotReloadLibrary;

struct App {
    state: State,

    #[cfg(feature = "hot_reload")]
    lib: HotReloadLibrary,
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

            let lib = HotReloadLibrary::load(&path).expect("failed to load library");
            let initialize = lib
                .symbol::<unsafe extern "C" fn() -> State>("initialize")
                .expect("failed to load initialize symbol");
            let state = unsafe { initialize() };

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
            let update = self
                .lib
                .symbol::<unsafe extern "C" fn(&mut State)>("update")
                .expect("failed to load update symbol");
            unsafe { update(&mut self.state) };
        }
    }

    fn get(&self) -> i32 {
        #[cfg(not(feature = "hot_reload"))]
        {
            example_lib::s_field
        }

        #[cfg(feature = "hot_reload")]
        {
            unsafe {
                **self
                    .lib
                    .symbol::<*const i32>("s_field")
                    .expect("failed to load static field")
            }
        }
    }

    fn set(&self, delta: i32) {
        #[cfg(not(feature = "hot_reload"))]
        {
            example_lib::s_field += delta;
        }

        #[cfg(feature = "hot_reload")]
        {
            let s = *self
                .lib
                .symbol::<*mut i32>("s_field")
                .expect("failed to load static field");
            unsafe { *s += delta };
        }
    }
}

fn main() {
    let mut app = App::initialize();

    println!("input '+'/'-' to increment/decrement `s_field` and update, 'q' to quit, anything else to only update");

    #[cfg(feature = "hot_reload")]
    println!("feature hot_reload: input 'r' to hot reload, 'f' to force hot reload");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        match &input.trim().to_lowercase()[..] {
            "+" => app.set(1),
            "-" => app.set(-1),
            "q" => break,
            #[cfg(feature = "hot_reload")]
            "r" => {
                app.lib.reload().expect("failed to reload lib");
                println!("lib reloaded");
                continue;
            }
            #[cfg(feature = "hot_reload")]
            "f" => {
                app.lib.force_reload().expect("failed to force reload lib");
                println!("lib force reloaded");
                continue;
            }
            _ => (),
        };

        println!("Update with `s_field` = {}", app.get());
        app.update();
        app.state.print();
    }
}

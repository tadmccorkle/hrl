use std::{
    fmt::Debug,
    fs,
    ops::Deref,
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver},
};

use libloading::{Library, Symbol as SymbolInner};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};

use super::{remove_hrl_files, Error, Result, HRL_EXT_A, HRL_EXT_B};

#[derive(Debug, Clone)]
pub struct Symbol<'a, T>(SymbolInner<'a, T>);

impl<'a, T> Deref for Symbol<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct HotReloadLibrary {
    lib: Option<Library>,
    loaded_path: PathBuf,
    watch_path: PathBuf,
    watcher: Receiver<notify::Result<Event>>,
}

impl HotReloadLibrary {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.is_file() {
            return Err(Error::PathNotFound);
        }

        let hrl_path = path.with_extension(HRL_EXT_A);
        fs::copy(path, &hrl_path).map_err(|e| Error::IoFailure(e))?;

        let lib = unsafe { Library::new(&hrl_path) }.map_err(|e| Error::LoadLibraryError(e))?;

        let (tx, rx) = mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())
            .map_err(|e| Error::FileWatcherError(e))?;
        watcher
            .watch(&path, RecursiveMode::NonRecursive)
            .map_err(|e| Error::FileWatcherError(e))?;

        Ok(Self {
            lib: Some(lib),
            loaded_path: hrl_path,
            watch_path: path.to_path_buf(),
            watcher: rx,
        })
    }

    pub fn reload(&mut self) -> Result<()> {
        let mut events = self.watcher.try_iter();
        let need_reload = events.any(|res| {
            if let Ok(event) = res {
                event.kind.is_create()
            } else {
                false
            }
        });
        events.for_each(drop);

        if need_reload {
            self.force_reload()
        } else {
            Ok(())
        }
    }

    pub fn force_reload(&mut self) -> Result<()> {
        let new_hrl_path = self.loaded_path.with_extension(
            match self.loaded_path.extension().map(|p| p.to_str()) {
                Some(Some(HRL_EXT_A)) => HRL_EXT_B,
                _ => HRL_EXT_A,
            },
        );

        if let Err(e) = fs::copy(&self.watch_path, &new_hrl_path) {
            Err(Error::IoFailure(e))
        } else {
            match unsafe { Library::new(&new_hrl_path) } {
                Ok(new_lib) => {
                    self.lib = Some(new_lib);
                    self.loaded_path = new_hrl_path;

                    Ok(())
                }
                Err(e) => Err(Error::LoadLibraryError(e)),
            }
        }
    }

    pub fn symbol<T>(&self, symbol_name: &str) -> Result<Symbol<T>> {
        let lib = self.lib.as_ref().expect("library unexpectedly unloaded");
        let symbol =
            unsafe { lib.get(symbol_name.as_bytes()) }.map_err(|e| Error::LoadSymbolError(e))?;

        Ok(Symbol(symbol))
    }

    pub fn close(mut self) -> Result<()> {
        self.close_ref()
    }

    fn close_ref(&mut self) -> Result<()> {
        if let Some(lib) = self.lib.take() {
            if let Err(e) = lib.close() {
                Err(Error::LoadLibraryError(e))
            } else {
                remove_hrl_files(&self.watch_path)
            }
        } else {
            remove_hrl_files(&self.watch_path)
        }
    }
}

impl Drop for HotReloadLibrary {
    fn drop(&mut self) {
        _ = self.close_ref();
    }
}

use std::{
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use libloading::Library;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};

use super::{remove_hrl_files, Error, Result, HRL_EXT_A, HRL_EXT_B};

#[derive(Debug)]
pub struct AutoHotReloadLibrary {
    lib: Arc<RwLock<Option<Library>>>,
    watch_path: PathBuf,
    watcher: Option<RecommendedWatcher>,
}

impl AutoHotReloadLibrary {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.is_file() {
            return Err(Error::PathNotFound);
        }

        let hrl_path = path.with_extension(HRL_EXT_A);
        fs::copy(path, &hrl_path).map_err(|e| Error::IoError(e))?;

        let lib = Arc::new(RwLock::new(Some(
            unsafe { Library::new(&hrl_path) }.map_err(|e| Error::LoadLibraryError(e))?,
        )));

        let mut watcher = Self::create_watcher(path.to_path_buf(), hrl_path, Arc::clone(&lib))
            .map_err(|e| Error::FileWatcherError(e))?;
        watcher
            .watch(path, RecursiveMode::NonRecursive)
            .map_err(|e| Error::FileWatcherError(e))?;

        Ok(Self {
            lib,
            watch_path: path.to_path_buf(),
            watcher: Some(watcher),
        })
    }

    fn create_watcher(
        watch_path: PathBuf,
        mut hrl_path: PathBuf,
        lib: Arc<RwLock<Option<Library>>>,
    ) -> notify::Result<RecommendedWatcher> {
        notify::recommended_watcher(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                if event.kind.is_create() {
                    let mut lib = lib.write().unwrap();

                    hrl_path.set_extension(match hrl_path.extension().map(|p| p.to_str()) {
                        Some(Some(HRL_EXT_A)) => HRL_EXT_B,
                        _ => HRL_EXT_A,
                    });

                    if fs::copy(&watch_path, &hrl_path).is_ok() {
                        if let Ok(new_lib) = unsafe { Library::new(&hrl_path) } {
                            lib.replace(new_lib);
                        } else {
                            lib.take();
                        }
                    } else {
                        lib.take();
                    }
                }
            }
        })
    }

    pub fn symbol_op<TSymbol, TReturn>(
        &self,
        symbol_name: &str,
        op: impl FnOnce(&TSymbol) -> TReturn,
    ) -> Result<TReturn> {
        if let Some(lib) = self.lib.read().unwrap().as_ref() {
            let symbol = unsafe { lib.get(symbol_name.as_bytes()) }
                .map_err(|e| Error::LoadSymbolError(e))?;
            Ok(op(&symbol))
        } else {
            Err(Error::LibraryUnloaded)
        }
    }

    pub fn close(mut self) -> Result<()> {
        self.close_ref()
    }

    fn stop_watcher(&mut self) -> Result<()> {
        if let Some(mut watcher) = self.watcher.take() {
            watcher
                .unwatch(&self.watch_path)
                .map_err(|e| Error::FileWatcherError(e))
        } else {
            Ok(())
        }
    }

    fn close_ref(&mut self) -> Result<()> {
        let stop_watcher_res = self.stop_watcher();
        let close_lib_res = if let Some(lib) = self.lib.write().unwrap().take() {
            if let Err(e) = lib.close() {
                Err(Error::LoadLibraryError(e))
            } else {
                remove_hrl_files(&self.watch_path)
            }
        } else {
            remove_hrl_files(&self.watch_path)
        };

        stop_watcher_res.and(close_lib_res)
    }
}

impl Drop for AutoHotReloadLibrary {
    fn drop(&mut self) {
        _ = self.close_ref();
    }
}

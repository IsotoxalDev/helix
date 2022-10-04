#[macro_use]
extern crate helix_view;

use anyhow::{Context, Error, Result};
use gdnative::prelude::*;
use gdnative::api::{ProjectSettings};
use std::path::PathBuf;

//pub mod application;
pub mod commands;
//pub mod compositor;
//pub mod config;
//pub mod health;
//pub mod job;
//pub mod keymap;
//pub mod ui;
//pub use keymap::macros::*;

fn setup_logging(logpath: PathBuf, verbosity: u64) -> Result<()> {
    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity {
        0 => base_config.level(log::LevelFilter::Warn),
        1 => base_config.level(log::LevelFilter::Info),
        2 => base_config.level(log::LevelFilter::Debug),
        _3_or_more => base_config.level(log::LevelFilter::Trace),
    };

    // Separate file config so we can include year, month and day in file logs
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {} [{}] {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(logpath)?);

    base_config.chain(file_config).apply()?;

    Ok(())
}

#[derive(NativeClass)]
#[inherit(Control)]
pub struct Helix;

#[methods]
impl Helix {
    fn new(_base: &Control) -> Self {
        Helix
    }
    
    #[method]
    fn _ready(&self, #[base] _base: &Control) {
        let logpath = helix_loader::log_file();
        let parent = logpath.parent().unwrap();
        if !parent.exists() {
            std::fs::create_dir_all(parent).ok();
        }
        setup_logging(logpath, 1).context("failed to initalize logging");

        let config_dir = helix_loader::config_dir();
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir).ok();
        }

        helix_loader::initialize_config_file(None);

        //let config = match std::fs::read_to_string(helix_loader::config_file()) {
        //    Ok(config) => toml::from_str(&config)
        //        .map(keymap::merge_keys)
        //        .unwrap_or_else(|err| {
        //            eprintln!("Bad config: {}", err);
        //            eprintln!("Press <ENTER> to continue with default config");
        //            use std::io::Read;
        //            let _ = std::io::stdin().read(&mut []);
        //            Config::default()
        //        }),
        //    Err(err) if err.kind() == std::io::ErrorKind::NotFound => Config::default(),
        //    Err(err) => godot_error!("{}", err),//Err(Error::new(err)),
        //};
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Helix>();
}

godot_init!(init);

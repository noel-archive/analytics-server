// 🐻‍❄️🐾 Noelware Analytics: Platform to build upon metrics ingested from any source, from your HTTP server to system-level metrics
// Copyright 2022 Noelware <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
    panic::set_hook,
    str::FromStr,
};

use ansi_term::Colour::RGB;
use anyhow::Result;
use chrono::Local;
use fern::Dispatch;
use log::{LevelFilter, Log};
use sentry::{init, types::Dsn, ClientOptions};
use sentry_log::{NoopLogger, SentryLogger};
use serde_json::json;

use crate::{config::Config, null_writer::NullWriter};

pub fn setup_sentry(config: &Config) -> Result<()> {
    if let Some(dsn) = &config.sentry_dsn {
        debug!("dsn was provided ({}), now enabling", dsn);
        let _ = init(ClientOptions {
            dsn: Some(Dsn::from_str(dsn.as_str())?),
            ..Default::default()
        });
    }

    Ok(())
}

pub fn setup_logging(config: &Config) -> Result<()> {
    let config = config.clone();
    let logging = &config.logging.unwrap_or_default();
    let info: &String = &"info".into();
    let level = logging.level.as_ref().unwrap_or(info);
    let log_filter = match level.as_str() {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    let console_dispatch = Dispatch::new()
        .format(move |out, message, record| {
            let thread = std::thread::current();
            let name = thread.name().unwrap_or("main");
            let pid = std::process::id();
            let disable_colours = std::env::var("ANALYTICS_SERVER_DISABLE_COLOURS").is_ok();

            if disable_colours {
                out.finish(format_args!(
                    "{} {:<5} [{} <{}> ({})] :: {}",
                    Local::now().format("[%B %d, %G | %H:%M:%S %p]"),
                    record.level(),
                    record.target(),
                    pid,
                    name,
                    message
                ));
            } else {
                let color = match record.level() {
                    log::Level::Error => RGB(153, 75, 104).bold(),
                    log::Level::Debug => RGB(163, 182, 138).bold(),
                    log::Level::Info => RGB(178, 157, 243).bold(),
                    log::Level::Trace => RGB(163, 182, 138).bold(),
                    log::Level::Warn => RGB(243, 243, 134).bold(),
                };

                let pid = std::process::id();
                let time = RGB(134, 134, 134).paint(format!(
                    "{}",
                    Local::now().format("[%B %d, %G | %H:%M:%S %p]")
                ));

                let level = color.paint(format!("{:<5}", record.level()));
                let (b1, b2) = (RGB(134, 134, 134).paint("["), RGB(134, 134, 134).paint("]"));
                let (p1, p2) = (RGB(134, 134, 134).paint("("), RGB(134, 134, 134).paint(")"));
                let (c1, c2) = (RGB(134, 134, 134).paint("<"), RGB(134, 134, 134).paint(">"));
                let target = RGB(120, 231, 255).paint(record.target());
                let thread_name = RGB(255, 105, 189).paint(name);
                let pid_colour = RGB(169, 147, 227).paint(pid.to_string());

                out.finish(format_args!(
                "{time} {level} {b1}{target} {c1}{thread_name}{c2} {p1}{pid_colour}{p2}{b2} :: {}",
                message
            ));
            }
        })
        .level(log_filter)
        .chain(std::io::stdout())
        .into_shared();

    let logstash_dispatch = Dispatch::new()
        .format(move |out, message, record| {
            let pid = std::process::id();
            let thread = std::thread::current();
            let thread_name = thread.name().unwrap_or("main");

            out.finish(format_args!(
                "{}",
                json!({
                    "@timestamp": Local::now().to_rfc3339(),
                    "@version": "1",
                    "message": format_args!("{}", message),
                    "log.level": record.level().as_str(),
                    "thread.name": thread_name,
                    "process.id": pid,
                    "metadata.file.path": record.file().unwrap_or("unknown.rs"),
                    "metadata.file.line": record.line().unwrap_or(0)
                })
            ));
        })
        .level(log_filter)
        .chain(if logging.logstash_url.is_some() {
            let logstash_host = logging
                .logstash_url
                .as_ref()
                .unwrap()
                .parse::<SocketAddr>()
                .expect("Unable to parse Logstash endpoint to SocketAddr!");

            Box::new(TcpStream::connect(logstash_host).expect("Unable to connect to Logstash"))
                as Box<dyn Write + Send>
        } else {
            Box::new(NullWriter) as Box<dyn Write + Send>
        })
        .into_shared();

    let dispatch = Dispatch::new()
        .chain(console_dispatch)
        .chain(logstash_dispatch)
        .chain(if config.sentry_dsn.is_some() {
            Box::new(SentryLogger::new())
        } else {
            Box::new(NoopLogger) as Box<dyn Log>
        });

    dispatch.apply()?;
    Ok(())
}

pub fn setup_panic_hook() {
    set_hook(Box::new(|info| {
        // let backtrace = Backtrace::force_capture();
        // println!("{}", backtrace);

        let message = if let Some(message) = info.payload().downcast_ref::<String>() {
            message.clone()
        } else if let Some(message) = info.payload().downcast_ref::<&str>() {
            (*message).to_string()
        } else {
            format!("{:?}", info)
        };

        let location = match info.location() {
            Some(loc) => format!("{}:{}:{}", loc.file(), loc.line(), loc.column()),
            None => "".to_string(),
        };

        let thread = std::thread::current();
        error!(
            "received panic in thread '{}' [{}] - {}",
            thread.name().unwrap_or("main"),
            location,
            message
        );

        std::process::exit(1);
    }));
}

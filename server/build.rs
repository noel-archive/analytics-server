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

use chrono::{DateTime, Utc};
use std::{error::Error, ffi::OsStr, fs::read, path::Path, process::Command, time::SystemTime};

#[allow(dead_code)]
fn read_file<P>(path: P) -> Result<String, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    Ok(read(path).map(String::from_utf8).unwrap()?)
}

fn execute<T>(command: T, args: &[&str]) -> Result<String, Box<dyn Error + 'static>>
where
    T: Into<String> + AsRef<OsStr>,
{
    let res = Command::new(command).args(args).output()?;
    Ok(String::from_utf8(res.stdout)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    // get stack version
    // read_file("../.stackversion").map(|data| {
    //     let splitted = data.split('\n');
    //     let iter = splitted.filter(|f| !f.starts_with('#')).collect::<Vec<_>>();

    //     match iter.get(0) {
    //         Some(s) => s,
    //         None => panic!("fuck"),
    //     }
    // })?;

    let commit_hash =
        execute("git", &["rev-parse", "--short=8", "HEAD"]).unwrap_or_else(|_| "noeluwu".into());

    let now = SystemTime::now();
    let utc: DateTime<Utc> = now.into();
    let build_date = utc.to_rfc3339();

    println!(
        "cargo:rustc-env=ANALYTICS_SERVER_COMMIT_HASH={}",
        commit_hash
    );

    println!("cargo:rustc-env=ANALYTICS_SERVER_BUILD_DATE={}", build_date);
    println!("cargo:rustc-env=ANALYTICS_SERVER_VERSION=v0.0.0-dev.0");

    Ok(())
}

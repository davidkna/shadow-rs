//! `shadow-rs` is a build-time information stored in your binary
//!
//! It's allows you to recall properties of the build process and environment at runtime, including:
//!
//! `Cargo.toml` project version
//! * Dependency information
//! * The Git commit that produced the build artifact (binary)
//! * What version of the rust toolchain was used in compilation
//! * The build variant, e.g. `debug` or `release`
//! * (And more)
//!
//! You can use this tool to check in production exactly where a binary came from and how it was built.
//!
//! # Full Examples
//! * Check out the [shadow_example](https://github.com/baoyachi/shadow-rs/tree/master/example_shadow) for a simple demonstration of how `shadow-rs` might be used to provide build-time information at run-time.
//!
//! # Example
//!
//! ```
//! pub const RUST_VERSION :&str = "rustc 1.45.0 (5c1f21c3b 2020-07-13)";
//! pub const BUILD_RUST_CHANNEL :&str = "debug";
//! pub const COMMIT_AUTHOR :&str = "baoyachi";
//! pub const BUILD_TIME :&str = "2020-08-16 13:48:52";
//! pub const COMMIT_DATE :&str = "2020-08-16 13:12:52";
//! pub const COMMIT_EMAIL :&str = "xxx@gmail.com";
//! pub const PROJECT_NAME :&str = "shadow-rs";
//! pub const RUST_CHANNEL :&str = "stable-x86_64-apple-darwin (default)";
//! pub const BRANCH :&str = "master";
//! pub const CARGO_LOCK :&str = r#"
//! ├── chrono v0.4.19
//! │   ├── libc v0.2.80
//! │   ├── num-integer v0.1.44
//! │   │   └── num-traits v0.2.14
//! │   │       [build-dependencies]
//! │   │       └── autocfg v1.0.1
//! │   ├── num-traits v0.2.14 (*)
//! │   └── time v0.1.44
//! │       └── libc v0.2.80
//! └── git2 v0.13.12
//! ├── log v0.4.11
//! │   └── cfg-if v0.1.10
//! └── url v2.2.0
//! ├── form_urlencoded v1.0.0
//! │   └── percent-encoding v2.1.0
//! └── percent-encoding v2.1.0"#;
//! pub const CARGO_VERSION :&str = "cargo 1.45.0 (744bd1fbb 2020-06-15)";
//! pub const BUILD_OS :&str = "macos-x86_64";
//! pub const COMMIT_HASH :&str = "386741540d73c194a3028b96b92fdeb53ca2788a";
//! pub const PKG_VERSION :&str = "0.3.13";
//! ```
//! # Setup Guide
//!
//! ### 1) Modify `Cargo.toml` fields
//! Modify your `Cargo.toml` like so:
//!
//! ```toml
//! [package]
//! build = "build.rs"
//!
//! [dependencies]
//! shadow-rs = "0.5"
//!
//! [build-dependencies]
//! shadow-rs = "0.5"
//! ```
//!
//! ### 2) Create `build.rs` file
//! Now in the root of your project (same directory as `Cargo.toml`) add a file `build.rs`:
//!
//! ```ignore
//! fn main() -> shadow_rs::SdResult<()> {
//!    shadow_rs::new()
//! }
//! ```
//!
//! ### 3) Integrate Shadow
//! In your root rust file (e.g. `main.rs`, or `lib.rs`):
//!
//! ```ignore
//! #[macro_use]
//! extern crate shadow_rs;
//!
//! shadow!(build);
//! ```
//! **Notice that the `shadow!` macro is provided the identifier `build`.  You can now use this identifier to access build-time information.**
//!
//! ### 4) Done. Use Shadow.
//! Then you can use const that's shadow build it(main.rs).
//!
//! The `build` mod just we use `shadow!(build)` generated.
//!
//! ```ignore
//! fn main(){
//!    println!("{}",shadow_rs::is_debug());//get build mode. cargo build --release return false.normally return true.
//!
//!    println!("{}",build::version()); //print version() method
//!    println!("{}",build::BRANCH); //master
//!    println!("{}",build::SHORT_COMMIT);//8405e28e
//!    println!("{}",build::COMMIT_HASH);//8405e28e64080a09525a6cf1b07c22fcaf71a5c5
//!    println!("{}",build::COMMIT_DATE);//2020-08-16T06:22:24+00:00
//!    println!("{}",build::COMMIT_AUTHOR);//baoyachi
//!    println!("{}",build::COMMIT_EMAIL);//xxx@gmail.com
//!
//!    println!("{}",build::BUILD_OS);//macos-x86_64
//!    println!("{}",build::RUST_VERSION);//rustc 1.45.0 (5c1f21c3b 2020-07-13)
//!    println!("{}",build::RUST_CHANNEL);//stable-x86_64-apple-darwin (default)
//!    println!("{}",build::CARGO_VERSION);//cargo 1.45.0 (744bd1fbb 2020-06-15)
//!    println!("{}",build::PKG_VERSION);//0.3.13
//!    println!("{}",build::CARGO_TREE); //like command:cargo tree
//!
//!    println!("{}",build::PROJECT_NAME);//shadow-rs
//!    println!("{}",build::BUILD_TIME);//2020-08-16 14:50:25
//!    println!("{}",build::BUILD_RUST_CHANNEL);//debug
//! }
//!```
//!
//! ## Clap example
//! And you can also use `shadow-rs` with [`clap`](https://github.com/baoyachi/shadow-rs/blob/master/example_shadow/src/main.rs).
//!
//! For the user guide and futher documentation, please read
//! [The shadow-rs document](https://github.com/baoyachi/shadow-rs).
//!

mod build;
mod channel;
mod ci;
mod env;
mod err;
mod git;

use build::*;
use env::*;

use git::*;

use crate::ci::CIType;
use std::collections::HashMap;
use std::env as std_env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub use channel::BuildRustChannel;
use chrono::Local;
pub use err::SdResult;

const SHADOW_RS: &str = "shadow.rs";

/// Add a mod in project with `$build_mod`.
///
/// You can use `shadow!(build_shadow)`. Then shadow-rs can help you add a mod with name `build_shadow`.
/// Next, use mod with name `build_shadow`,and also use const like:`build_shadow::BRANCH`.
///
/// Normally, you just config `shadow!(build)`.It looks more concise.
#[macro_export]
macro_rules! shadow {
    ($build_mod:ident) => {
        pub mod $build_mod {
            include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
        }
    };
}

/// It's shadow-rs Initialization entry.
///
/// In build.rs `main()` method call for this method.
///
/// # Examples
///
/// ```ignore
/// fn main() -> shadow_rs::SdResult<()> {
///    shadow_rs::new()
/// }
/// ```
pub fn new() -> SdResult<()> {
    let src_path = std::env::var("CARGO_MANIFEST_DIR")?;
    let out_path = std::env::var("OUT_DIR")?;
    Shadow::build(src_path, out_path)
}

/// Get current project build mode.
///
/// It's very useful. Debug mode is usually used for debugging information.
/// For example, log printing, environment variable switch.
///
/// The default value is `true`.
///
/// If we compile with `cargo build --release`. It's return value is `false`.
pub fn is_debug() -> bool {
    channel::build_channel() == BuildRustChannel::Debug
}

#[derive(Debug)]
pub(crate) struct Shadow {
    f: File,
    map: HashMap<ShadowConst, ConstVal>,
    std_env: HashMap<String, String>,
}

impl Shadow {
    fn get_env() -> HashMap<String, String> {
        let mut env_map = HashMap::new();
        for (k, v) in std_env::vars() {
            env_map.insert(k, v);
        }
        env_map
    }

    /// try get current ci env
    fn try_ci(&self) -> CIType {
        if let Some(c) = self.std_env.get("GITLAB_CI") {
            if c == "true" {
                return CIType::Gitlab;
            }
        }

        if let Some(c) = self.std_env.get("GITHUB_ACTIONS") {
            if c == "true" {
                return CIType::Github;
            }
        }

        //TODO completed [travis,jenkins] env

        CIType::None
    }

    fn build(src_path: String, out_path: String) -> SdResult<()> {
        let out = {
            let path = Path::new(out_path.as_str());
            if !out_path.ends_with('/') {
                path.join(format!("{}/{}", out_path, SHADOW_RS))
            } else {
                path.join(SHADOW_RS)
            }
        };

        let mut shadow = Shadow {
            f: File::create(out)?,
            map: Default::default(),
            std_env: Default::default(),
        };
        shadow.std_env = Self::get_env();

        let ci_type = shadow.try_ci();
        let src_path = Path::new(src_path.as_str());

        let mut map = new_git(&src_path, ci_type, &shadow.std_env);
        for (k, v) in new_project(&shadow.std_env) {
            map.insert(k, v);
        }
        for (k, v) in new_system_env(&shadow.std_env) {
            map.insert(k, v);
        }
        shadow.map = map;

        shadow.gen_const()?;

        //write version method
        shadow.write_version()?;

        Ok(())
    }

    fn gen_const(&mut self) -> SdResult<()> {
        self.write_header()?;
        for (k, v) in self.map.clone() {
            self.write_const(k, v)?;
        }
        Ok(())
    }

    fn write_header(&self) -> SdResult<()> {
        let desc = format!(
            r#"/// Code generated by shadow-rs generator. DO NOT EDIT.
/// Author by:https://www.github.com/baoyachi
/// The build script repository:https://github.com/baoyachi/shadow-rs
/// Create time by:{}"#,
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        );
        writeln!(&self.f, "{}\n\n", desc)?;
        Ok(())
    }

    fn write_const(&mut self, shadow_const: ShadowConst, val: ConstVal) -> SdResult<()> {
        let desc = format!("/// {}", val.desc);

        let (t, v) = match val.t {
            ConstType::OptStr => (ConstType::Str.to_string(), "".into()),
            ConstType::Str => (ConstType::Str.to_string(), val.v),
        };

        let define = format!(
            "#[allow(dead_code)]\n\
            pub const {} :{} = r#\"{}\"#;",
            shadow_const.to_ascii_uppercase(),
            t,
            v
        );
        writeln!(&self.f, "{}", desc)?;
        writeln!(&self.f, "{}\n", define)?;
        Ok(())
    }

    fn write_version(&mut self) -> SdResult<()> {
        let desc: &str = "/// The common version method. It's so easy to use this method";

        const VERSION_BRANCH_FN: &str = r##"#[allow(dead_code)]
pub fn version() -> String {
    format!(r#"
pkg_version:{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
    )
}"##;

        const VERSION_TAG_FN: &str = r##"#[allow(dead_code)]
pub fn version() -> String {
    format!(r#"
pkg_version:{}
tag:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, TAG, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
    )
}"##;

        let version_fn = match self.map.get(TAG) {
            None => VERSION_BRANCH_FN,
            Some(tag) => {
                if !tag.v.is_empty() {
                    VERSION_TAG_FN
                } else {
                    VERSION_BRANCH_FN
                }
            }
        };
        writeln!(&self.f, "{}", desc)?;
        writeln!(&self.f, "{}\n", version_fn)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_build() -> SdResult<()> {
        Shadow::build("./".into(), "./".into())?;
        let shadow = fs::read_to_string("./shadow.rs")?;
        println!("{}", shadow);
        Ok(())
    }

    #[test]
    fn test_env() {
        for (k, v) in std::env::vars() {
            println!("K:{},V:{}", k, v);
        }
    }
}

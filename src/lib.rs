//! Auto launch any application or executable at startup. Supports Windows, Mac (via AppleScript or Launch Agent), and Linux.
//!
//! ## Usage
//!
//! The parameters of `AutoLaunch::new` are different on each platform.
//! See the function definition or the demo below for details.
//!
//! Or you can construct the AutoLaunch by using `AutoLaunchBuilder`.
//!
//! ```rust
//! # #[cfg(target_os = "linux")]
//! # mod linux {
//! use auto_launch::AutoLaunch;
//!
//! fn main() {
//!     let app_name = "the-app";
//!     let app_path = "/path/to/the-app";
//!     let args = &["--minimized"];
//!     let auto = AutoLaunch::new(app_name, app_path, args);
//!
//!     // enable the auto launch
//!     auto.enable().is_ok();
//!     auto.is_enabled().unwrap();
//!
//!     // disable the auto launch
//!     auto.disable().is_ok();
//!     auto.is_enabled().unwrap();
//! }
//! # }
//! ```
//!
//! ### macOS
//!
//! macOS supports two ways to achieve auto launch (via AppleScript or Launch Agent).
//! When the `use_launch_agent` is true, it will achieve by Launch Agent, otherwise by AppleScript.
//!
//! **Note**:
//! - The `app_path` should be a absolute path and exists. Otherwise, it will cause an error when `enable`.
//! - In case using AppleScript, the `app_name` should be same as the basename of `app_path`, or it will be corrected automatically.
//!
//! ```rust
//! # #[cfg(target_os = "macos")]
//! # mod macos {
//! use auto_launch::AutoLaunch;
//!
//! fn main() {
//!     let app_name = "the-app";
//!     let app_path = "/path/to/the-app.app";
//!     let args = &["--minimized"];
//!     let bundle_identifiers = &["com.github.auto-launch-test"];
//!     let auto = AutoLaunch::new(app_name, app_path, false, args, bundle_identifiers, "");
//!
//!     // enable the auto launch
//!     auto.enable().is_ok();
//!     auto.is_enabled().unwrap();
//!
//!     // disable the auto launch
//!     auto.disable().is_ok();
//!     auto.is_enabled().unwrap();
//! }
//! # }
//! ```
//!
//! ### Windows
//!
//! On Windows, it will add a registry entry under either `\HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Run` (system-wide) or
//! `\HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Run` (current user only).
//!
//! By default we try to apply the auto launch to the system registry, which requires admin privileges and applies the auto launch to any user in the system.
//! If there's no permission to do so, we fallback to enabling it to the current user only.
//! To change this behavior, you can use [`AutoLaunch::set_windows_enable_mode`].
//!
//! ```rust
//! # #[cfg(target_os = "windows")]
//! # mod win {
//! use auto_launch::{AutoLaunch, WindowsEnableMode};
//!
//! fn main() {
//!     let app_name = "the-app";
//!     let app_path = "C:\\path\\to\\the-app.exe";
//!     let args = &["--minimized"];
//!     let enable_mode = WindowsEnableMode::CurrentUser;
//!     let auto = AutoLaunch::new(app_name, app_path, enable_mode, args);
//!
//!     // enable the auto launch
//!     auto.enable().is_ok();
//!     auto.is_enabled().unwrap();
//!
//!     // disable the auto launch
//!     auto.disable().is_ok();
//!     auto.is_enabled().unwrap();
//! }
//! # }
//! ```
//!
//! ### Builder
//!
//! AutoLaunch Builder helps to eliminate the constructor difference
//! on various platforms.
//!
//! ```rust
//! use auto_launch::*;
//!
//! fn main() {
//!     let auto = AutoLaunchBuilder::new()
//!         .set_app_name("the-app")
//!         .set_app_path("/path/to/the-app")
//!         .set_use_launch_agent(true)
//!         .set_args(&["--minimized"])
//!         .build()
//!         .unwrap();
//!
//!     auto.enable().is_ok();
//!     auto.is_enabled().unwrap();
//!
//!     auto.disable().is_ok();
//!     auto.is_enabled().unwrap();
//! }
//! ```
//!

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("app_name shouldn't be None")]
    AppNameNotSpecified,
    #[error("app_path shouldn't be None")]
    AppPathNotSpecified,
    #[error("app path doesn't exist: {0}")]
    AppPathDoesntExist(std::path::PathBuf),
    #[error("app path is not absolute: {0}")]
    AppPathIsNotAbsolute(std::path::PathBuf),
    #[error("Failed to execute apple script with status: {0}")]
    AppleScriptFailed(i32),
    #[error("Unsupported target os")]
    UnsupportedOS,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

/// The parameters of `AutoLaunch::new` are different on each platform.
///
/// ### Linux
///
/// ```rust
/// # #[cfg(target_os = "linux")]
/// # {
/// # use auto_launch::AutoLaunch;
/// # let app_name = "the-app";
/// # let app_path = "/path/to/the-app";
/// # let args = &["--minimized"];
/// AutoLaunch::new(app_name, app_path, args);
/// # }
/// ```
///
/// ### Macos
///
/// ```rust
/// # #[cfg(target_os = "macos")]
/// # {
/// # use auto_launch::AutoLaunch;
/// # let app_name = "the-app";
/// # let app_path = "/path/to/the-app";
/// # let use_launch_agent = false;
/// # let args = &["--minimized"];
/// # let bundle_identifiers = &["com.github.auto-launch-test"];
/// AutoLaunch::new(app_name, app_path, use_launch_agent, args, bundle_identifiers, "");
/// # }
/// ```
///
/// ### Windows
///
/// ```rust
/// # #[cfg(target_os = "windows")]
/// # {
/// # use auto_launch::{AutoLaunch, WindowsEnableMode};
/// # let app_name = "the-app";
/// # let app_path = "/path/to/the-app";
/// # let args = &["--minimized"];
/// # let enable_mode = WindowsEnableMode::CurrentUser;
/// AutoLaunch::new(app_name, app_path, enable_mode, args);
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutoLaunch {
    /// The application name
    pub(crate) app_name: String,

    /// The application executable path (absolute path will be better)
    pub(crate) app_path: String,

    /// Args passed to the binary on startup
    pub(crate) args: Vec<String>,

    #[cfg(target_os = "macos")]
    /// Whether use Launch Agent for implement or use AppleScript
    pub(crate) use_launch_agent: bool,

    #[cfg(target_os = "macos")]
    /// Bundle identifiers
    pub(crate) bundle_identifiers: Vec<String>,

    #[cfg(target_os = "macos")]
    /// Extra config in plist file for Launch Agent
    pub(crate) agent_extra_config: String,

    #[cfg(windows)]
    pub(crate) enable_mode: WindowsEnableMode,
}

impl AutoLaunch {
    /// check whether it is support the platform
    ///
    /// ## Usage
    ///
    /// ```rust
    /// use auto_launch::AutoLaunch;
    ///
    /// dbg!(AutoLaunch::is_support());
    /// ```
    pub fn is_support() -> bool {
        cfg!(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "windows",
        ))
    }

    /// get the application name
    pub fn get_app_name(&self) -> &str {
        &self.app_name
    }

    /// get the application path
    pub fn get_app_path(&self) -> &str {
        &self.app_path
    }

    /// get the args
    pub fn get_args(&self) -> &[String] {
        &self.args
    }
}

#[derive(Debug, Default, Clone)]
/// AutoLaunch Builder helps to eliminate the constructor difference
/// on various platforms.
///
/// ## Notes
///
/// The builder will not check whether the app_path matches the platform-specify file path.
///
/// ## Usage
///
/// ```rust
/// use auto_launch::*;
///
/// fn main() {
///     let auto = AutoLaunchBuilder::new()
///         .set_app_name("the-app")
///         .set_app_path("/path/to/the-app")
///         .set_use_launch_agent(true)
///         .set_args(&["--minimized"])
///         .build()
///         .unwrap();
///
///     auto.enable().is_ok();
///     auto.is_enabled().unwrap();
///
///     auto.disable().is_ok();
///     auto.is_enabled().unwrap();
/// }
/// ```
pub struct AutoLaunchBuilder {
    pub app_name: Option<String>,

    pub app_path: Option<String>,

    pub use_launch_agent: bool,

    pub bundle_identifiers: Option<Vec<String>>,

    pub agent_extra_config: Option<String>,

    pub windows_enable_mode: WindowsEnableMode,

    pub args: Option<Vec<String>>,
}

/// Determines how the auto launch is enabled on Windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowsEnableMode {
    /// Dynamically tries to enable the auto launch for the system (admin privileges required),
    /// fallbacks to the current user if there is no permission to modify the system registry.
    Dynamic,
    /// Enables the auto launch for the current user only. Does not require admin permissions.
    CurrentUser,
    /// Enables the auto launch for all users. Requires admin permissions.
    System,
}

impl Default for WindowsEnableMode {
    fn default() -> Self {
        Self::Dynamic
    }
}

impl AutoLaunchBuilder {
    pub fn new() -> AutoLaunchBuilder {
        AutoLaunchBuilder::default()
    }

    /// Set the `app_name`
    pub fn set_app_name(&mut self, name: &str) -> &mut Self {
        self.app_name = Some(name.into());
        self
    }

    /// Set the `app_path`
    pub fn set_app_path(&mut self, path: &str) -> &mut Self {
        self.app_path = Some(path.into());
        self
    }

    /// Set the `use_launch_agent`
    /// This setting only works on macOS
    pub fn set_use_launch_agent(&mut self, use_launch_agent: bool) -> &mut Self {
        self.use_launch_agent = use_launch_agent;
        self
    }

    /// Set the `bundle_identifiers`
    /// This setting only works on macOS
    pub fn set_bundle_identifiers(&mut self, bundle_identifiers: &[impl AsRef<str>]) -> &mut Self {
        self.bundle_identifiers = Some(
            bundle_identifiers
                .iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
        );
        self
    }

    /// Set the `agent_extra_config`
    /// This setting only works on macOS
    pub fn set_agent_extra_config(&mut self, config: &str) -> &mut Self {
        self.agent_extra_config = Some(config.into());
        self
    }

    /// Set the [`WindowsEnableMode`].
    /// This setting only works on Windows
    pub fn set_windows_enable_mode(&mut self, mode: WindowsEnableMode) -> &mut Self {
        self.windows_enable_mode = mode;
        self
    }

    /// Set the args
    pub fn set_args(&mut self, args: &[impl AsRef<str>]) -> &mut Self {
        self.args = Some(args.iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    /// Construct a AutoLaunch instance
    ///
    /// ## Errors
    ///
    /// - `app_name` is none
    /// - `app_path` is none
    /// - Unsupported target OS
    pub fn build(&self) -> Result<AutoLaunch> {
        let app_name = self.app_name.as_ref().ok_or(Error::AppNameNotSpecified)?;
        let app_path = self.app_path.as_ref().ok_or(Error::AppPathNotSpecified)?;
        let args = self.args.clone().unwrap_or_default();
        let bundle_identifiers = self.bundle_identifiers.clone().unwrap_or_default();
        let agent_extra_config = self.agent_extra_config.as_ref().map_or("", |v| v);

        #[cfg(target_os = "linux")]
        return Ok(AutoLaunch::new(app_name, app_path, &args));
        #[cfg(target_os = "macos")]
        return Ok(AutoLaunch::new(
            app_name,
            app_path,
            self.use_launch_agent,
            &args,
            &bundle_identifiers,
            agent_extra_config,
        ));
        #[cfg(target_os = "windows")]
        return Ok(AutoLaunch::new(
            app_name,
            app_path,
            self.windows_enable_mode,
            &args,
        ));

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        return Err(Error::UnsupportedOS);
    }
}

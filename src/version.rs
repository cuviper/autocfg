use std::cmp::Ordering;
use std::path::Path;
use std::process::Command;
use std::str;

use super::{error, Error};

/// The channel the current compiler was released on.
///
/// `Channel`s are orderable by their available features. The more features a channel supports, the
/// higher it is ordered. Specifically, channels are ordered as follows: `Stable < Beta < Nightly <
/// Dev`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    /// Stable channel.
    Stable,
    /// Beta channel.
    Beta,
    /// Nightly channel.
    Nightly,
    /// Dev channel.
    Dev,
}

impl PartialOrd<Channel> for Channel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (*self as u8).partial_cmp(&(*other as u8))
    }
}

impl Ord for Channel {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

/// A version structure for making relative comparisons.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: usize,
    minor: usize,
    patch: usize,
}

impl Version {
    /// Creates a `Version` instance for a specific `major.minor.patch` version.
    pub fn new(major: usize, minor: usize, patch: usize) -> Self {
        Version {
            major: major,
            minor: minor,
            patch: patch,
        }
    }
}

pub fn version_and_channel_from_rustc(rustc: &Path) -> Result<(Version, Channel), Error> {
    // Get rustc's verbose version
    let output = try!(Command::new(rustc)
        .args(&["--version", "--verbose"])
        .output()
        .map_err(error::from_io));
    if !output.status.success() {
        return Err(error::from_str("could not execute rustc"));
    }
    let output = try!(str::from_utf8(&output.stdout).map_err(error::from_utf8));

    // Find the release line in the verbose version output.
    let release = match output.lines().find(|line| line.starts_with("release: ")) {
        Some(line) => &line["release: ".len()..],
        None => return Err(error::from_str("could not find rustc release")),
    };

    let mut release_split = release.split('-');
    let version = match release_split.next() {
        Some(version) => version,
        None => return Err(error::from_str("could not parse rustc release")),
    };
    let channel = match release_split.next() {
        Some(channel) => {
            if channel.starts_with("beta") {
                Channel::Beta
            } else if channel.starts_with("nightly") {
                Channel::Nightly
            } else if channel.starts_with("dev") {
                Channel::Dev
            } else {
                return Err(error::from_str("could not parse rustc channel"));
            }
        }
        None => Channel::Stable,
    };

    // Split the version into semver components.
    let mut iter = version.splitn(3, '.');
    let major = try!(iter.next().ok_or(error::from_str("missing major version")));
    let minor = try!(iter.next().ok_or(error::from_str("missing minor version")));
    let patch = try!(iter.next().ok_or(error::from_str("missing patch version")));

    Ok((
        Version::new(
            try!(major.parse().map_err(error::from_num)),
            try!(minor.parse().map_err(error::from_num)),
            try!(patch.parse().map_err(error::from_num)),
        ),
        channel,
    ))
}

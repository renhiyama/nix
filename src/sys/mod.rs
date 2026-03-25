//! Mostly platform-specific functionality
#[cfg(any(target_os = "dragonfly",
          target_os = "freebsd",
          target_os = "ios",
          all(any(target_os = "linux", target_os = "runixos"), not(target_env = "uclibc")),
          target_os = "macos",
          target_os = "netbsd"))]
feature! {
    #![feature = "aio"]
    pub mod aio;
}

feature! {
    #![feature = "event"]

    #[cfg(any(target_os = "android", any(target_os = "linux", target_os = "runixos")))]
    #[allow(missing_docs)]
    pub mod epoll;

    #[cfg(any(target_os = "dragonfly",
              target_os = "freebsd",
              target_os = "ios",
              target_os = "macos",
              target_os = "netbsd",
              target_os = "openbsd"))]
    #[allow(missing_docs)]
    pub mod event;

    #[cfg(any(target_os = "android", any(target_os = "linux", target_os = "runixos")))]
    #[allow(missing_docs)]
    pub mod eventfd;
}

#[cfg(any(target_os = "android",
          target_os = "dragonfly",
          target_os = "freebsd",
          target_os = "ios",
          any(target_os = "linux", target_os = "runixos"),
          target_os = "redox",
          target_os = "macos",
          target_os = "netbsd",
          target_os = "illumos",
          target_os = "openbsd"))]
#[cfg(feature = "ioctl")]
#[cfg_attr(docsrs, doc(cfg(feature = "ioctl")))]
#[macro_use]
pub mod ioctl;

#[cfg(any(target_os = "linux", target_os = "runixos"))]
feature! {
    #![feature = "fs"]
    pub mod memfd;
}

#[cfg(not(target_os = "redox"))]
feature! {
    #![feature = "mman"]
    pub mod mman;
}

#[cfg(any(target_os = "linux", target_os = "runixos"))]
feature! {
    #![feature = "personality"]
    pub mod personality;
}

feature! {
    #![feature = "pthread"]
    pub mod pthread;
}

#[cfg(any(target_os = "android",
          target_os = "dragonfly",
          target_os = "freebsd",
          any(target_os = "linux", target_os = "runixos"),
          target_os = "macos",
          target_os = "netbsd",
          target_os = "openbsd"))]
feature! {
    #![feature = "ptrace"]
    #[allow(missing_docs)]
    pub mod ptrace;
}

#[cfg(any(target_os = "linux", target_os = "runixos"))]
feature! {
    #![feature = "quota"]
    pub mod quota;
}

#[cfg(any(target_os = "linux", target_os = "runixos"))]
feature! {
    #![feature = "reboot"]
    pub mod reboot;
}

#[cfg(not(any(target_os = "redox", target_os = "fuchsia", target_os = "illumos")))]
feature! {
    #![feature = "resource"]
    pub mod resource;
}

#[cfg(not(target_os = "redox"))]
feature! {
    #![feature = "poll"]
    pub mod select;
}

#[cfg(any(target_os = "android",
          target_os = "dragonfly",
          target_os = "freebsd",
          target_os = "ios",
          any(target_os = "linux", target_os = "runixos"),
          target_os = "macos"))]
feature! {
    #![feature = "zerocopy"]
    pub mod sendfile;
}

pub mod signal;

#[cfg(any(target_os = "android", any(target_os = "linux", target_os = "runixos")))]
feature! {
    #![feature = "signal"]
    #[allow(missing_docs)]
    pub mod signalfd;
}

#[cfg(not(target_os = "redox"))]
feature! {
    #![feature = "socket"]
    #[allow(missing_docs)]
    pub mod socket;
}

feature! {
    #![feature = "fs"]
    #[allow(missing_docs)]
    pub mod stat;
}

#[cfg(any(target_os = "android",
          target_os = "dragonfly",
          target_os = "freebsd",
          target_os = "ios",
          any(target_os = "linux", target_os = "runixos"),
          target_os = "macos",
          target_os = "openbsd"
))]
feature! {
    #![feature = "fs"]
    pub mod statfs;
}

feature! {
    #![feature = "fs"]
    pub mod statvfs;
}

#[cfg(any(target_os = "android", any(target_os = "linux", target_os = "runixos")))]
#[cfg_attr(docsrs, doc(cfg(all())))]
#[allow(missing_docs)]
pub mod sysinfo;

feature! {
    #![feature = "term"]
    #[allow(missing_docs)]
    pub mod termios;
}

#[allow(missing_docs)]
pub mod time;

feature! {
    #![feature = "uio"]
    pub mod uio;
}

feature! {
    #![feature = "feature"]
    pub mod utsname;
}

feature! {
    #![feature = "process"]
    pub mod wait;
}

#[cfg(any(target_os = "android", any(target_os = "linux", target_os = "runixos")))]
feature! {
    #![feature = "inotify"]
    pub mod inotify;
}

#[cfg(any(target_os = "android", any(target_os = "linux", target_os = "runixos")))]
feature! {
    #![feature = "time"]
    pub mod timerfd;
}

#[cfg(all(
    any(
        target_os = "freebsd",
        target_os = "illumos",
        any(target_os = "linux", target_os = "runixos"),
        target_os = "netbsd"
    ),
    feature = "time",
    feature = "signal"
))]
feature! {
    #![feature = "time"]
    pub mod timer;
}

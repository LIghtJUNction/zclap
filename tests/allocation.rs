use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::fmt::Write;
use zclap::{Arg, Command, Style};

thread_local! {
    static CALLS: Cell<Option<usize>> = const { Cell::new(None) };
}

struct Counting;

fn count() {
    let _ = CALLS.try_with(|value| {
        if let Some(n) = value.get() {
            value.set(Some(n + 1));
        }
    });
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count();
        // SAFETY: forwards the allocator caller's contract unchanged.
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count();
        // SAFETY: forwards the allocator caller's contract unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        count();
        // SAFETY: forwards the allocator caller's contract unchanged.
        unsafe { System.realloc(ptr, layout, size) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: forwards the allocator caller's contract unchanged.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

struct Buffer {
    bytes: [u8; 4096],
    len: usize,
}

impl Write for Buffer {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let end = self.len.checked_add(s.len()).ok_or(std::fmt::Error)?;
        let dst = self.bytes.get_mut(self.len..end).ok_or(std::fmt::Error)?;
        dst.copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}

fn measured(f: impl FnOnce()) -> usize {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            CALLS.with(|n| n.set(None));
        }
    }
    CALLS.with(|n| n.set(Some(0)));
    let _reset = Reset;
    f();
    CALLS.with(|n| n.get().unwrap())
}

const CLI: Command = Command::new("demo").args(&[
    Arg::flag("verbose")
        .short('v')
        .aliases(&["chatty", "debug"])
        .repeat()
        .requires_all(&["input", "mode"])
        .conflicts_with(&["quiet", "silent"]),
    Arg::value("mode").one_of(&["fast", "safe"]),
    Arg::positional("input").required(),
    Arg::flag("quiet"),
    Arg::flag("silent"),
]);

#[test]
fn parsing_validation_errors_and_rendering_allocate_nothing() {
    let count = measured(|| {
        let argv = ["-vv", "--debug", "--mode=fast", "input"];
        let mut parser = CLI.parse(&argv);
        let mut events = 0;
        while parser.next().unwrap().is_some() {
            events += 1;
        }
        parser.finish().unwrap();
        assert_eq!(events, 5);
        let mut out = Buffer {
            bytes: [0; 4096],
            len: 0,
        };
        Style::default().help(&CLI, &mut out).unwrap();
        let argv = ["--mode=invalid"];
        let error = CLI.parse(&argv).next().unwrap_err();
        Style::plain().error(&error, &mut out).unwrap();
        let empty: [&str; 0] = [];
        let error = CLI.parse(&empty).finish().unwrap_err();
        write!(&mut out, "{error}").unwrap();

        for argv in [
            &["--chatty", "--mode=fast", "--silent", "input"][..],
            &["--debug", "input"][..],
        ] {
            let mut parser = CLI.parse(argv);
            while parser.next().unwrap().is_some() {}
            let error = parser.finish().unwrap_err();
            Style::plain().error(&error, &mut out).unwrap();
        }
    });
    assert_eq!(count, 0);
}

#[cfg(feature = "std")]
#[test]
fn os_adapter_adds_no_allocations() {
    let args = [
        std::ffi::OsString::from("--mode=safe"),
        std::ffi::OsString::from("input"),
    ];
    assert_eq!(
        measured(|| {
            let mut parser = CLI.parse_os(&args);
            while parser.next().unwrap().is_some() {}
            parser.finish().unwrap();
        }),
        0
    );
}

#![cfg(feature = "std")]

use std::ffi::{OsStr, OsString};
use zclap::{Arg, Command, Error, Event};

const CLI: Command = Command::new("demo").args(&[
    Arg::value("file").short('f'),
    Arg::positional("path").repeat(),
]);

#[test]
fn owned_and_borrowed_os_inputs() {
    let args = [OsString::from("--file=hello"), OsString::from("世界")];
    let mut parser = CLI.parse_os(&args);
    let value = parser.next().unwrap().unwrap().value().unwrap();
    assert_eq!(value, OsStr::new("hello"));
    assert_eq!(
        value.as_encoded_bytes().as_ptr(),
        args[0].as_encoded_bytes()[7..].as_ptr()
    );
    assert_eq!(
        parser.next().unwrap(),
        Some(Event::Value {
            arg: "path",
            value: OsStr::new("世界")
        })
    );
    parser.finish().unwrap();
    let refs: Vec<&OsStr> = args.iter().map(OsString::as_os_str).collect();
    let mut parser = CLI.parse(&refs);
    assert_eq!(
        parser.next().unwrap().unwrap().value(),
        Some(OsStr::new("hello"))
    );
    let strings = [String::from("--file=x")];
    assert_eq!(
        CLI.parse(&strings).next().unwrap().unwrap().value(),
        Some("x")
    );
}

#[cfg(unix)]
#[test]
fn unix_raw_bytes_round_trip() {
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    for prefix in [b"--file=".as_slice(), b"-f", b"-f="] {
        let raw = [prefix, b"a\xff\xfe\x00z"].concat();
        let args = [OsString::from_vec(raw)];
        let mut parser = CLI.parse_os(&args);
        assert_eq!(
            parser.next().unwrap().unwrap().value().unwrap().as_bytes(),
            b"a\xff\xfe\x00z"
        );
        parser.finish().unwrap();
    }
    let args = [
        OsString::from("--file"),
        OsString::from_vec(vec![0xff, 0x80]),
    ];
    assert_eq!(
        CLI.parse_os(&args)
            .next()
            .unwrap()
            .unwrap()
            .value()
            .unwrap()
            .as_bytes(),
        &[0xff, 0x80]
    );
    let args = [OsString::from_vec(vec![b'-', 0xff])];
    assert_eq!(CLI.parse_os(&args).next(), Err(Error::Unknown { index: 0 }));
}

#[cfg(windows)]
#[test]
fn windows_unpaired_surrogates_round_trip() {
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    for prefix in ["--file=", "-f", "-f="] {
        let mut wide: Vec<u16> = prefix.encode_utf16().collect();
        wide.extend([0xd800, 0x61, 0xdc00]);
        let args = [OsString::from_wide(&wide)];
        let mut parser = CLI.parse_os(&args);
        let actual: Vec<_> = parser
            .next()
            .unwrap()
            .unwrap()
            .value()
            .unwrap()
            .encode_wide()
            .collect();
        assert_eq!(actual, [0xd800, 0x61, 0xdc00]);
        parser.finish().unwrap();
    }
    let args = [OsString::from("--file"), OsString::from_wide(&[0xdc00])];
    let actual: Vec<_> = CLI
        .parse_os(&args)
        .next()
        .unwrap()
        .unwrap()
        .value()
        .unwrap()
        .encode_wide()
        .collect();
    assert_eq!(actual, [0xdc00]);
    let args = [OsString::from_wide(&[0x2d, 0xd800])];
    assert_eq!(CLI.parse_os(&args).next(), Err(Error::Unknown { index: 0 }));
}

#[test]
fn unicode_short_prefix_keeps_os_suffix() {
    const CLI: Command = Command::new("demo").args(&[Arg::value("file").short('路')]);
    let args = [OsString::from("-路=hello")];
    assert_eq!(
        CLI.parse_os(&args).next().unwrap().unwrap().value(),
        Some(OsStr::new("hello"))
    );
}

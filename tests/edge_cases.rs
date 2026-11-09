use zclap::{Arg, Command, Error, Event};

const COUNT: Command = Command::new("count").args(&[
    Arg::flag("verbose").short('v').repeat(),
    Arg::flag("quiet").short('q').repeat(),
    Arg::value("number").short('n').repeat(),
    Arg::positional("file").repeat(),
]);

fn consume(command: &Command<'_>, argv: &[&str]) -> bool {
    let mut parser = command.parse(argv);
    loop {
        match parser.next() {
            Ok(Some(_)) => {}
            Ok(None) => return parser.finish().is_ok(),
            Err(_) => return false,
        }
    }
}

#[test]
fn empty_values_are_values_and_borrows_point_into_input() {
    const CLI: Command = Command::new("x").args(&[Arg::value("output").short('o').repeat()]);
    let owned = String::from("--output=héllo");
    let input = [owned.as_str(), "-o=", "--output", ""];
    let mut parser = CLI.parse(&input);
    match parser.next().unwrap().unwrap() {
        Event::Value { arg, value } => {
            assert_eq!(arg, "output");
            assert_eq!(value, "héllo");
            assert_eq!(value.as_ptr(), owned[9..].as_ptr());
        }
        event => panic!("expected value, got {event:?}"),
    }
    for _ in 0..2 {
        assert!(matches!(
            parser.next().unwrap(),
            Some(Event::Value { value: "", .. })
        ));
    }
    assert!(parser.next().unwrap().is_none());
    parser.finish().unwrap();
}

#[test]
fn end_of_options_preserves_all_remaining_tokens() {
    let input = ["-vvq", "--", "--number=42", "-v", "--", "", "-"];
    let mut parser = COUNT.parse(&input);
    for expected in ["verbose", "verbose", "quiet"] {
        assert!(matches!(parser.next().unwrap(), Some(Event::Flag { arg }) if arg == expected));
    }
    for expected in &input[2..] {
        assert!(
            matches!(parser.next().unwrap(), Some(Event::Value { arg: "file", value }) if value == *expected)
        );
    }
    assert!(parser.next().unwrap().is_none());
    assert!(parser.next().unwrap().is_none());
    parser.finish().unwrap();
}

#[test]
fn conflicts_and_requires_are_independent_of_argument_order() {
    const CLI: Command = Command::new("x").args(&[
        Arg::flag("red").conflicts("blue"),
        Arg::flag("blue"),
        Arg::flag("save").requires("output"),
        Arg::value("output"),
    ]);
    assert!(!consume(&CLI, &["--red", "--blue"]));
    assert!(!consume(&CLI, &["--blue", "--red"]));
    assert!(!consume(&CLI, &["--save"]));
    assert!(consume(&CLI, &["--save", "--output=result"]));
    assert!(consume(&CLI, &["--output=result", "--save"]));
    assert!(consume(&CLI, &["--output=result"]));
}

#[test]
fn aliases_share_duplicate_and_required_state() {
    const CLI: Command =
        Command::new("x").args(&[Arg::value("output").short('o').alias("out").required()]);
    assert!(consume(&CLI, &["--out=a"]));
    assert!(consume(&CLI, &["-oa"]));
    assert!(!consume(&CLI, &["--out=a", "--output=b"]));
    assert!(!consume(&CLI, &["-oa", "--out=b"]));
    assert!(!consume(&CLI, &[]));
}

#[test]
fn long_tokens_and_many_repetitions_do_not_overflow() {
    let huge = "é".repeat(64 * 1024);
    let assigned = format!("--number={huge}");
    let input = [assigned.as_str()];
    let mut parser = COUNT.parse(&input);
    assert!(
        matches!(parser.next().unwrap(), Some(Event::Value { arg: "number", value }) if value == huge)
    );
    assert!(parser.next().unwrap().is_none());
    parser.finish().unwrap();

    let flags = format!("-{}", "v".repeat(65_537));
    let input = [flags.as_str()];
    let mut parser = COUNT.parse(&input);
    let mut count = 0;
    while let Some(event) = parser.next().unwrap() {
        assert!(matches!(event, Event::Flag { arg: "verbose" }));
        count += 1;
    }
    assert_eq!(count, 65_537);
    parser.finish().unwrap();
}

#[test]
fn byte_values_are_lossless_in_both_joined_and_separate_forms() {
    const CLI: Command = Command::new("x").args(&[
        Arg::value("path").short('p').repeat(),
        Arg::positional("file").repeat(),
    ]);
    let input: &[&[u8]] = &[b"--path=\xff\x80", b"-p\xfe", b"-p", b"\x00\xff", b"\xff"];
    let mut parser = CLI.parse_bytes(input);
    for expected in [
        &b"\xff\x80"[..],
        &b"\xfe"[..],
        &b"\x00\xff"[..],
        &b"\xff"[..],
    ] {
        assert!(
            matches!(parser.next().unwrap(), Some(Event::Value { value, .. }) if value == expected)
        );
    }
    assert!(parser.next().unwrap().is_none());
    parser.finish().unwrap();
}

#[test]
fn deterministic_random_valid_syntax_matches_an_independent_count() {
    let mut seed = 0x2a1f_9547_b6cc_9183_u64;
    for _case in 0..512 {
        let mut owned = Vec::new();
        let mut verbose = 0;
        let mut quiet = 0;
        let mut sum = 0;
        for _ in 0..32 {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let number = (seed >> 32) as u16;
            match seed % 6 {
                0 => {
                    owned.push("--verbose".to_owned());
                    verbose += 1;
                }
                1 => {
                    owned.push("-vvq".to_owned());
                    verbose += 2;
                    quiet += 1;
                }
                2 => {
                    owned.push(format!("-n{number}"));
                    sum += u64::from(number);
                }
                3 => {
                    owned.push(format!("--number={number}"));
                    sum += u64::from(number);
                }
                4 => {
                    owned.push("--number".to_owned());
                    owned.push(number.to_string());
                    sum += u64::from(number);
                }
                _ => {
                    owned.push("-q".to_owned());
                    quiet += 1;
                }
            }
        }
        let input: Vec<_> = owned.iter().map(String::as_str).collect();
        let mut parser = COUNT.parse(&input);
        let (mut actual_verbose, mut actual_quiet, mut actual_sum) = (0, 0, 0_u64);
        while let Some(event) = parser.next().unwrap() {
            match event {
                Event::Flag { arg: "verbose" } => actual_verbose += 1,
                Event::Flag { arg: "quiet" } => actual_quiet += 1,
                Event::Value {
                    arg: "number",
                    value,
                } => actual_sum += value.parse::<u64>().unwrap(),
                event => panic!("unexpected event {event:?}"),
            }
        }
        parser.finish().unwrap();
        assert_eq!(
            (actual_verbose, actual_quiet, actual_sum),
            (verbose, quiet, sum)
        );
    }
}

#[test]
fn deterministic_arbitrary_bytes_terminate_without_panicking() {
    let mut seed = 0x95d3_1a07_u32;
    for _ in 0..4096 {
        let mut bytes = [0_u8; 48];
        for byte in &mut bytes {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            *byte = seed as u8;
        }
        if seed & 1 != 0 {
            bytes[0] = b'-';
        }
        if seed & 2 != 0 {
            bytes[1] = b'-';
        }
        let input = [&bytes[..]];
        let mut parser = COUNT.parse_bytes(&input);
        let mut events = 0;
        while let Ok(Some(_)) = parser.next() {
            events += 1;
            assert!(events <= bytes.len());
        }
        let _ = parser.finish();
    }
}

#[test]
fn flag_assignments_fail_before_emitting_the_flag() {
    const CLI: Command = Command::new("x").args(&[Arg::flag("verbose").short('v')]);
    for token in ["--verbose=yes", "--verbose=", "-v=yes", "-v="] {
        let input = [token];
        let mut parser = CLI.parse(&input);
        assert!(
            matches!(
                parser.next(),
                Err(Error::UnexpectedValue {
                    arg: "verbose",
                    index: 0
                })
            ),
            "{token}"
        );
    }
}

#[test]
fn errors_and_successful_completion_are_terminal() {
    let input = ["-vv", "--unknown", "-q"];
    let mut parser = COUNT.parse(&input);
    assert!(parser.next().unwrap().is_some());
    assert!(parser.next().unwrap().is_some());
    let error = parser.next().unwrap_err();
    assert_eq!(error.index(), Some(1));
    assert_eq!(parser.next().unwrap_err(), error);
    assert_eq!(parser.finish().unwrap_err(), error);

    let mut parser = COUNT.parse(&["-v"]);
    let error = parser.finish().unwrap_err();
    assert!(matches!(error, Error::Unfinished { index: 0 }));
    assert_eq!(parser.next().unwrap_err(), error);

    let mut parser = COUNT.parse(&[] as &[&str]);
    parser.finish().unwrap();
    parser.finish().unwrap();
    assert!(parser.next().unwrap().is_none());
}

#[test]
fn multivalue_groups_stop_at_options_and_commands() {
    const CLI: Command = Command::new("x")
        .args(&[
            Arg::value("pair").short('p').values(2, 3).repeat(),
            Arg::flag("verbose"),
        ])
        .subcommands(&[Command::new("run")]);
    assert!(consume(&CLI, &["--pair=a", "b", "--verbose"]));
    assert!(consume(&CLI, &["-pa", "b", "c", "--pair", "d", "e"]));
    assert!(!consume(&CLI, &["--pair", "a", "--verbose"]));
    assert!(!consume(&CLI, &["-pa"]));
    let input = ["--pair", "a", "b", "run"];
    let mut parser = CLI.parse(&input);
    assert!(matches!(
        parser.next().unwrap(),
        Some(Event::Value { value: "a", .. })
    ));
    assert!(matches!(
        parser.next().unwrap(),
        Some(Event::Value { value: "b", .. })
    ));
    assert!(matches!(
        parser.next().unwrap(),
        Some(Event::Command { name: "run" })
    ));
    assert_eq!(parser.command().name(), "run");
    assert!(parser.next().unwrap().is_none());
    parser.finish().unwrap();
}

#[test]
fn nested_commands_validate_each_scope_and_do_not_inherit_parent_options() {
    const CLI: Command = Command::new("x")
        .args(&[Arg::flag("root").required()])
        .subcommands(&[Command::new("remote")
            .args(&[Arg::value("name").required()])
            .subcommands(&[Command::new("add").args(&[Arg::positional("url").required()])])]);
    assert!(consume(
        &CLI,
        &[
            "--root",
            "remote",
            "--name=origin",
            "add",
            "https://example.invalid"
        ]
    ));
    assert!(!consume(&CLI, &["remote", "--name=origin", "add", "url"]));
    assert!(!consume(&CLI, &["--root", "remote", "add", "url"]));
    assert!(!consume(
        &CLI,
        &["--root", "remote", "--name=origin", "add"]
    ));
    assert!(!consume(
        &CLI,
        &["--root", "remote", "--name=origin", "--root"]
    ));
}

#[test]
fn unicode_option_names_and_clusters_use_character_boundaries() {
    const CLI: Command = Command::new("x").args(&[
        Arg::flag("詳細").short('é').repeat(),
        Arg::value("出力").short('ø').repeat(),
    ]);
    let input = ["-ééø=値", "--出力=資料", "--詳細"];
    let mut parser = CLI.parse(&input);
    for _ in 0..2 {
        assert!(matches!(
            parser.next().unwrap(),
            Some(Event::Flag { arg: "詳細" })
        ));
    }
    for expected in ["値", "資料"] {
        assert!(
            matches!(parser.next().unwrap(), Some(Event::Value { arg: "出力", value }) if value == expected)
        );
    }
    assert!(matches!(
        parser.next().unwrap(),
        Some(Event::Flag { arg: "詳細" })
    ));
    assert!(parser.next().unwrap().is_none());
    parser.finish().unwrap();
}

#[cfg(all(feature = "std", unix))]
#[test]
fn unix_non_utf8_os_values_remain_borrowed_and_lossless() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    const CLI: Command = Command::new("x").args(&[
        Arg::value("path").short('p').repeat(),
        Arg::positional("file"),
    ]);
    let input = [
        OsString::from_vec(b"--path=\xff".to_vec()),
        OsString::from_vec(b"-p\xfe".to_vec()),
        OsString::from("-p"),
        OsString::from_vec(b"\xfd".to_vec()),
        OsString::from_vec(b"\xfc".to_vec()),
    ];
    let mut parser = CLI.parse_os(&input);
    for (token, offset, expected) in [(0, 7, 0xff), (1, 2, 0xfe), (3, 0, 0xfd), (4, 0, 0xfc)] {
        let event = parser.next().unwrap().unwrap();
        let value = event.value().unwrap().as_encoded_bytes();
        assert_eq!(value, [expected]);
        assert_eq!(
            value.as_ptr(),
            input[token].as_encoded_bytes()[offset..].as_ptr()
        );
    }
    assert!(parser.next().unwrap().is_none());
    parser.finish().unwrap();
}

#[cfg(all(feature = "std", windows))]
#[test]
fn windows_unpaired_surrogates_are_preserved_in_all_value_forms() {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    const CLI: Command = Command::new("x").args(&[
        Arg::value("path").short('ø').repeat(),
        Arg::positional("file"),
    ]);
    let joined: Vec<_> = "--path=".encode_utf16().chain([0xd800, 0x61]).collect();
    let short: Vec<_> = "-ø=".encode_utf16().chain([0xdc00]).collect();
    let input = [
        OsString::from_wide(&joined),
        OsString::from_wide(&short),
        OsString::from("--path"),
        OsString::from_wide(&[0xd801]),
        OsString::from_wide(&[0xdc01]),
    ];
    let mut parser = CLI.parse_os(&input);
    for expected in [
        &[0xd800_u16, 0x61][..],
        &[0xdc00][..],
        &[0xd801][..],
        &[0xdc01][..],
    ] {
        let event = parser.next().unwrap().unwrap();
        let actual: Vec<_> = event.value().unwrap().encode_wide().collect();
        assert_eq!(actual, expected);
    }
    assert!(parser.next().unwrap().is_none());
    parser.finish().unwrap();
}

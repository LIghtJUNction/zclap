use zclap::{Arg, Command, Error, Event};

fn run<'s>(command: &'s Command<'s>, input: &[&str]) -> Result<(), Error<'s>> {
    run_with::<4>(command, input)
}

fn run_with<'s, const WORDS: usize>(
    command: &'s Command<'s>,
    input: &[&str],
) -> Result<(), Error<'s>> {
    let mut parser = command.parse_with::<WORDS, _>(input);
    while parser.next()?.is_some() {}
    parser.finish()
}

#[test]
fn aliases_return_the_same_ids_and_borrowed_values() {
    const CLI: Command = Command::new("copy").args(&[
        Arg::flag("verbose")
            .short('v')
            .aliases(&["chatty", "詳細"])
            .repeat(),
        Arg::value("output")
            .long("destination")
            .aliases(&["out", "target"])
            .required(),
    ]);
    CLI.check().unwrap();
    for input in [
        ["--verbose", "--destination=héllo"],
        ["--chatty", "--out=héllo"],
        ["--詳細", "--target=héllo"],
    ] {
        let mut parser = CLI.parse(&input);
        assert_eq!(parser.next().unwrap(), Some(Event::Flag { arg: "verbose" }));
        let offset = input[1].find('=').unwrap() + 1;
        match parser.next().unwrap().unwrap() {
            Event::Value { arg, value } => {
                assert_eq!(arg, "output");
                assert_eq!(value, "héllo");
                assert_eq!(value.as_ptr(), input[1][offset..].as_ptr());
            }
            event => panic!("unexpected event: {event:?}"),
        }
        assert_eq!(parser.next().unwrap(), None);
        parser.finish().unwrap();
    }
    run(&CLI, &["-v", "--chatty", "--詳細", "--out", "file"]).unwrap();
}

#[test]
fn all_aliases_share_duplicate_and_required_state() {
    const CLI: Command =
        Command::new("demo").args(&[Arg::flag("main").aliases(&["first", "second"]).required()]);
    let spellings = ["--main", "--first", "--second"];
    for first in spellings {
        run(&CLI, &[first]).unwrap();
        for second in spellings {
            assert_eq!(
                run(&CLI, &[first, second]),
                Err(Error::Duplicate {
                    arg: "main",
                    index: 1
                })
            );
        }
    }
    assert_eq!(run(&CLI, &[]), Err(Error::MissingRequired { arg: "main" }));
}

#[test]
fn alias_collisions_are_definition_errors_in_every_direction() {
    for args in [
        [
            Arg::flag("first").aliases(&["same", "same"]),
            Arg::flag("second"),
        ],
        [
            Arg::flag("first").aliases(&["first", "another"]),
            Arg::flag("second"),
        ],
        [
            Arg::flag("first").aliases(&["unused", "second"]),
            Arg::flag("second"),
        ],
        [
            Arg::flag("first"),
            Arg::flag("second").aliases(&["unused", "first"]),
        ],
        [
            Arg::flag("first").aliases(&["unused", "shared"]),
            Arg::flag("second").aliases(&["shared", "other"]),
        ],
        [
            Arg::flag("first").alias("shared"),
            Arg::flag("second").aliases(&["other", "shared"]),
        ],
    ] {
        let command = Command::new("demo").args(&args);
        let error = command.check().unwrap_err();
        assert!(matches!(error, Error::Definition { .. }));
        let mut parser = command.parse(&[] as &[&str]);
        assert_eq!(parser.next().unwrap_err(), error);
        assert_eq!(parser.finish().unwrap_err(), error);
    }
}

#[test]
fn invalid_aliases_are_checked_beyond_the_first_entry() {
    for invalid in [
        "",
        "--double",
        "-single",
        "with space",
        "name=value",
        "\0",
        "\t",
        "\x7f",
    ] {
        let names = ["valid", invalid];
        let args = [Arg::flag("main").aliases(&names)];
        let command = Command::new("demo").args(&args);
        assert!(
            matches!(command.check(), Err(Error::Definition { arg: "main", .. })),
            "{invalid:?}"
        );
    }
    const POSITIONAL: Command =
        Command::new("demo").args(&[Arg::positional("file").aliases(&["path", "input"])]);
    assert!(matches!(
        POSITIONAL.check(),
        Err(Error::Definition { arg: "file", .. })
    ));
}

#[test]
fn const_builders_replace_alias_lists_and_getters_expose_the_current_list() {
    const PLURAL: Arg = Arg::flag("main")
        .alias("obsolete")
        .aliases(&["first", "second"]);
    const SINGULAR: Arg = Arg::flag("main")
        .aliases(&["obsolete", "old"])
        .alias("current");
    const EMPTY: Arg = Arg::flag("main").alias("obsolete").aliases(&[]);
    assert_eq!(PLURAL.alias_names(), &["first", "second"]);
    assert_eq!(PLURAL.alias_name(), Some("first"));
    assert_eq!(SINGULAR.alias_names(), &["current"]);
    assert_eq!(SINGULAR.alias_name(), Some("current"));
    assert!(EMPTY.alias_names().is_empty());
    assert_eq!(EMPTY.alias_name(), None);
    const CLI: Command = Command::new("demo").args(&[PLURAL]);
    run(&CLI, &["--first"]).unwrap();
    assert_eq!(run(&CLI, &["--obsolete"]), Err(Error::Unknown { index: 0 }));
    const CLEARED: Command = Command::new("demo").args(&[EMPTY]);
    run(&CLEARED, &["--main"]).unwrap();
    assert_eq!(
        run(&CLEARED, &["--obsolete"]),
        Err(Error::Unknown { index: 0 })
    );
}

#[test]
fn every_dependency_is_required_and_its_declared_order_controls_the_error() {
    const CLI: Command = Command::new("demo").args(&[
        Arg::flag("save").requires_all(&["second", "first"]),
        Arg::flag("first"),
        Arg::flag("second"),
    ]);
    assert_eq!(
        run(&CLI, &["--save"]),
        Err(Error::Requires {
            arg: "save",
            other: "second"
        })
    );
    assert_eq!(
        run(&CLI, &["--save", "--second"]),
        Err(Error::Requires {
            arg: "save",
            other: "first"
        })
    );
    assert_eq!(
        run(&CLI, &["--save", "--first"]),
        Err(Error::Requires {
            arg: "save",
            other: "second"
        })
    );
    run(&CLI, &["--save", "--first", "--second"]).unwrap();
    run(&CLI, &["--second", "--first", "--save"]).unwrap();
    run(&CLI, &[]).unwrap();
    run(&CLI, &["--first"]).unwrap();
}

#[test]
fn every_conflict_is_symmetric_with_respect_to_input_order() {
    const CLI: Command = Command::new("demo").args(&[
        Arg::flag("source").conflicts_with(&["second", "first"]),
        Arg::flag("first"),
        Arg::flag("second"),
    ]);
    for (other, spelling) in [("first", "--first"), ("second", "--second")] {
        for input in [["--source", spelling], [spelling, "--source"]] {
            assert_eq!(
                run(&CLI, &input),
                Err(Error::Conflict {
                    arg: "source",
                    other
                })
            );
        }
    }
    for input in [
        ["--first", "--second", "--source"],
        ["--source", "--second", "--first"],
    ] {
        assert_eq!(
            run(&CLI, &input),
            Err(Error::Conflict {
                arg: "source",
                other: "second"
            })
        );
    }
    run(&CLI, &["--first", "--second"]).unwrap();
}

#[test]
fn error_precedence_follows_definition_order_then_conflicts_then_requires() {
    const CLI: Command = Command::new("demo").args(&[
        Arg::flag("early")
            .conflicts_with(&["blocker"])
            .requires_all(&["first", "second"]),
        Arg::flag("later").requires_all(&["second", "first"]),
        Arg::flag("first"),
        Arg::flag("second"),
        Arg::flag("blocker"),
    ]);
    assert_eq!(
        run(&CLI, &["--later", "--early"]),
        Err(Error::Requires {
            arg: "early",
            other: "first"
        })
    );
    assert_eq!(
        run(&CLI, &["--later", "--early", "--blocker"]),
        Err(Error::Conflict {
            arg: "early",
            other: "blocker"
        })
    );
    assert_eq!(
        run(&CLI, &["--later"]),
        Err(Error::Requires {
            arg: "later",
            other: "second"
        })
    );
    run(&CLI, &["--later", "--early", "--first", "--second"]).unwrap();
}

#[test]
fn relationships_use_definition_ids_and_work_through_all_aliases() {
    const CLI: Command = Command::new("demo").args(&[
        Arg::flag("save")
            .aliases(&["write", "persist"])
            .requires_all(&["output", "format"]),
        Arg::value("output")
            .long("destination")
            .aliases(&["out", "target"]),
        Arg::flag("format").aliases(&["raw", "binary"]),
    ]);
    run(&CLI, &["--persist", "--target=file", "--binary"]).unwrap();
    assert_eq!(
        run(&CLI, &["--write", "--out=file"]),
        Err(Error::Requires {
            arg: "save",
            other: "format"
        })
    );
    const INVALID: Command = Command::new("demo").args(&[
        Arg::flag("save").requires_all(&["out"]),
        Arg::value("output").aliases(&["out", "target"]),
    ]);
    assert!(matches!(
        INVALID.check(),
        Err(Error::Definition { arg: "save", .. })
    ));
}

#[test]
fn singular_and_plural_setters_replace_relationship_lists() {
    const ARG: Arg = Arg::flag("main")
        .conflicts_with(&["first", "second"])
        .conflicts("third")
        .requires_all(&["first", "second"])
        .requires("third");
    assert_eq!(ARG.conflict_targets(), &["third"]);
    assert_eq!(ARG.conflict_target(), Some("third"));
    assert_eq!(ARG.requires_targets(), &["third"]);
    assert_eq!(ARG.requires_target(), Some("third"));
    const LIST: Arg = Arg::flag("main")
        .conflicts("third")
        .conflicts_with(&["second", "first"])
        .requires("third")
        .requires_all(&["first", "second"]);
    assert_eq!(LIST.conflict_targets(), &["second", "first"]);
    assert_eq!(LIST.conflict_target(), Some("second"));
    assert_eq!(LIST.requires_targets(), &["first", "second"]);
    assert_eq!(LIST.requires_target(), Some("first"));
    const CLEARED: Command = Command::new("demo").args(&[Arg::flag("main")
        .conflicts_with(&["missing", "main"])
        .conflicts_with(&[])
        .requires_all(&["main", "missing"])
        .requires_all(&[])]);
    CLEARED.check().unwrap();
    run(&CLEARED, &["--main"]).unwrap();
    let cleared = &CLEARED.arguments()[0];
    assert!(cleared.conflict_targets().is_empty());
    assert!(cleared.requires_targets().is_empty());
    assert_eq!(cleared.conflict_target(), None);
    assert_eq!(cleared.requires_target(), None);
}

#[test]
fn relationship_targets_reject_unknown_and_self_but_allow_redundant_targets() {
    for arg in [
        Arg::flag("main").conflicts_with(&["other", "missing"]),
        Arg::flag("main").conflicts_with(&["other", "main"]),
        Arg::flag("main").requires_all(&["other", "missing"]),
        Arg::flag("main").requires_all(&["other", "main"]),
    ] {
        let args = [arg, Arg::flag("other")];
        let command = Command::new("demo").args(&args);
        assert!(matches!(
            command.check(),
            Err(Error::Definition { arg: "main", .. })
        ));
    }
    const CLI: Command = Command::new("demo").args(&[
        Arg::flag("main")
            .requires_all(&["first", "first"])
            .conflicts_with(&["second", "second"]),
        Arg::flag("first"),
        Arg::flag("second"),
    ]);
    CLI.check().unwrap();
    run(&CLI, &["--main", "--first"]).unwrap();
    assert_eq!(
        run(&CLI, &["--main", "--first", "--second"]),
        Err(Error::Conflict {
            arg: "main",
            other: "second"
        })
    );
}

#[test]
fn parent_relationships_are_validated_before_subcommand_transition() {
    const CLI: Command = Command::new("demo")
        .args(&[
            Arg::flag("save").requires_all(&["output", "format"]),
            Arg::value("output").aliases(&["out", "target"]),
            Arg::flag("format"),
        ])
        .subcommands(&[Command::new("run").args(&[
            Arg::flag("save").requires_all(&["format", "output"]),
            Arg::value("output").aliases(&["out", "target"]),
            Arg::flag("format"),
        ])]);
    assert_eq!(
        run(&CLI, &["--save", "--out=root", "run"]),
        Err(Error::Requires {
            arg: "save",
            other: "format"
        })
    );
    assert_eq!(
        run(&CLI, &["--save", "--out=root", "--format", "run", "--save"]),
        Err(Error::Requires {
            arg: "save",
            other: "format"
        })
    );
    run(
        &CLI,
        &[
            "--save",
            "--out=root",
            "--format",
            "run",
            "--save",
            "--format",
            "--target=child",
        ],
    )
    .unwrap();
}

#[test]
fn aliases_and_relationships_cover_the_last_index_and_large_command_fallback() {
    for count in [256, 257] {
        let names: Vec<_> = (0..count)
            .map(|index| format!("option{index:03}"))
            .collect();
        let targets = [
            names[254].as_str(),
            names[128].as_str(),
            names[64].as_str(),
            names[0].as_str(),
        ];
        let mut args: Vec<_> = names.iter().map(|name| Arg::flag(name)).collect();
        args[count - 1] = args[count - 1]
            .aliases(&["last", "tail"])
            .requires_all(&targets);
        let command = Command::new("demo").args(&args);
        command.check().unwrap();
        if count == 257 {
            assert!(matches!(
                run(&command, &["--tail"]),
                Err(Error::Capacity {
                    needed: 257,
                    available: 256
                })
            ));
        }
        for alias in ["--last", "--tail"] {
            assert_eq!(
                run_with::<5>(&command, &[alias]),
                Err(Error::Requires {
                    arg: names[count - 1].as_str(),
                    other: targets[0]
                })
            );
            run_with::<5>(
                &command,
                &[
                    alias,
                    "--option254",
                    "--option128",
                    "--option064",
                    "--option000",
                ],
            )
            .unwrap();
        }
        let primary = format!("--{}", names[count - 1]);
        run_with::<5>(
            &command,
            &[
                &primary,
                "--option000",
                "--option064",
                "--option128",
                "--option254",
            ],
        )
        .unwrap();
    }
}

#[test]
fn alias_count_does_not_consume_argument_capacity() {
    let names: Vec<_> = (0..300).map(|index| format!("alias{index}")).collect();
    let aliases: Vec<_> = names.iter().map(String::as_str).collect();
    let args = [Arg::flag("main").aliases(&aliases)];
    let command = Command::new("demo").args(&args);
    command.check().unwrap();
    for alias in ["--main", "--alias0", "--alias255", "--alias299"] {
        run_with::<1>(&command, &[alias]).unwrap();
    }
}

#[test]
fn byte_input_preserves_non_utf8_values_after_unicode_aliases() {
    const CLI: Command =
        Command::new("demo").args(&[Arg::value("path").aliases(&["file", "é"]).repeat()]);
    let tokens: &[&[u8]] = &[b"--file=\xff", b"--\xc3\xa9=\xfe", b"--path", b"\xfd"];
    let mut parser = CLI.parse_bytes(tokens);
    for expected in [0xff, 0xfe, 0xfd] {
        let event = parser.next().unwrap().unwrap();
        assert_eq!(event.id(), "path");
        assert_eq!(event.value().unwrap(), &[expected]);
    }
    assert!(parser.next().unwrap().is_none());
    parser.finish().unwrap();
}

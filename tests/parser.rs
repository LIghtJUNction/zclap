use zclap::{Arg, Command, Error, Event};

fn run<'s, 'a>(cli: &'s Command<'s>, argv: &'a [&'a str]) -> Result<Vec<Event<'s, 'a>>, Error<'s>> {
    let mut parser = cli.parse(argv);
    let mut result = Vec::new();
    while let Some(event) = parser.next()? {
        result.push(event);
    }
    parser.finish()?;
    Ok(result)
}

const BASIC: Command = Command::new("demo").args(&[
    Arg::flag("verbose").short('v').repeat(),
    Arg::flag("quiet").short('q'),
    Arg::value("port").short('p'),
    Arg::positional("input").required(),
]);

#[test]
fn mixed_spellings_and_order() {
    let argv = ["file", "-vvq", "--port=8080"];
    assert_eq!(
        run(&BASIC, &argv).unwrap(),
        vec![
            Event::Value {
                arg: "input",
                value: "file"
            },
            Event::Flag { arg: "verbose" },
            Event::Flag { arg: "verbose" },
            Event::Flag { arg: "quiet" },
            Event::Value {
                arg: "port",
                value: "8080"
            },
        ]
    );
}

#[test]
fn option_values_all_forms() {
    for argv in [
        vec!["-p12", "x"],
        vec!["-p=12", "x"],
        vec!["-p", "12", "x"],
        vec!["--port", "12", "x"],
        vec!["--port=12", "x"],
    ] {
        assert_eq!(
            run(&BASIC, &argv).unwrap()[0],
            Event::Value {
                arg: "port",
                value: "12"
            }
        );
    }
}

#[test]
fn short_cluster_ends_at_value() {
    assert_eq!(
        run(&BASIC, &["-vqpvalue", "x"]).unwrap()[2],
        Event::Value {
            arg: "port",
            value: "value"
        }
    );
    assert_eq!(
        run(&BASIC, &["-vp", "value", "x"]).unwrap()[1],
        Event::Value {
            arg: "port",
            value: "value"
        }
    );
}

#[test]
fn empty_dash_and_terminator() {
    for value in ["", "-", "--", "--quiet", "--port=9"] {
        let argv = ["--", value];
        assert_eq!(
            run(&BASIC, &argv).unwrap(),
            vec![Event::Value {
                arg: "input",
                value
            }]
        );
    }
    assert_eq!(
        run(&BASIC, &["-"]).unwrap(),
        vec![Event::Value {
            arg: "input",
            value: "-"
        }]
    );
}

#[test]
fn empty_and_negative_attached_values() {
    for option in ["--port=", "-p="] {
        assert_eq!(
            run(&BASIC, &[option, "x"]).unwrap()[0],
            Event::Value {
                arg: "port",
                value: ""
            }
        );
    }
    for option in ["--port=-1", "-p-1"] {
        assert_eq!(
            run(&BASIC, &[option, "x"]).unwrap()[0],
            Event::Value {
                arg: "port",
                value: "-1"
            }
        );
    }
    assert!(matches!(
        run(&BASIC, &["--port", "--quiet", "x"]),
        Err(Error::MissingValue {
            arg: "port",
            index: 0
        })
    ));
}

#[test]
fn explicit_long_name_and_alias() {
    const CLI: Command =
        Command::new("demo").args(&[Arg::value("id").long("primary").alias("alternative")]);
    assert_eq!(
        run(&CLI, &["--alternative=ok"]).unwrap(),
        vec![Event::Value {
            arg: "id",
            value: "ok"
        }]
    );
    assert!(matches!(
        run(&CLI, &["--id=ok"]),
        Err(Error::Unknown { index: 0 })
    ));
}

#[test]
fn structured_errors_and_terminal_state() {
    for argv in [
        vec!["--wat"],
        vec!["-z"],
        vec!["--port"],
        vec!["--port", "--"],
        vec!["--quiet=yes"],
        vec!["-q=yes"],
        vec!["-qq", "x"],
        vec!["x", "y"],
    ] {
        let mut parser = BASIC.parse(&argv);
        let error = loop {
            match parser.next() {
                Err(e) => break e,
                Ok(None) => break parser.finish().unwrap_err(),
                Ok(Some(_)) => {}
            }
        };
        assert_eq!(parser.next().unwrap_err(), error);
        assert_eq!(parser.finish().unwrap_err(), error);
        assert_eq!(error.exit_code(), 2);
        assert!(error.index().is_some());
        assert!(!error.to_string().is_empty());
    }
}

#[test]
fn finish_is_required_and_idempotent() {
    let empty: [&str; 0] = [];
    let mut parser = BASIC.parse(&empty);
    assert_eq!(parser.next().unwrap(), None);
    assert_eq!(parser.next().unwrap(), None);
    assert_eq!(
        parser.finish(),
        Err(Error::MissingRequired { arg: "input" })
    );
    let mut parser = BASIC.parse(&["x"]);
    assert!(parser.next().unwrap().is_some());
    parser.finish().unwrap();
    parser.finish().unwrap();
    assert_eq!(parser.next().unwrap(), None);
    let mut parser = BASIC.parse(&["-vv", "x"]);
    parser.next().unwrap();
    assert_eq!(parser.finish(), Err(Error::Unfinished { index: 0 }));
    let mut parser = BASIC.parse(&["x", "-v"]);
    assert_eq!(parser.finish(), Err(Error::Unfinished { index: 0 }));
}

#[test]
fn duplicate_values_and_flags() {
    for argv in [vec!["--port=1", "-p2", "x"], vec!["--quiet", "-q", "x"]] {
        assert!(matches!(
            run(&BASIC, &argv),
            Err(Error::Duplicate { index: 1, .. })
        ));
    }
}

#[test]
fn value_groups_and_repeat() {
    const CLI: Command = Command::new("demo").args(&[
        Arg::value("pair").short('p').values(2, 3).repeat(),
        Arg::flag("flag"),
    ]);
    let events = run(&CLI, &["--pair=a", "b", "c", "--flag", "-pd", "e"]).unwrap();
    assert_eq!(events.len(), 6);
    assert_eq!(
        events[4],
        Event::Value {
            arg: "pair",
            value: "d"
        }
    );
    assert_eq!(
        run(&CLI, &["--pair=a"]),
        Err(Error::MissingValue {
            arg: "pair",
            index: 0
        })
    );
    assert_eq!(
        run(&CLI, &["--pair=a", "--flag"]),
        Err(Error::MissingValue {
            arg: "pair",
            index: 0
        })
    );
    let mut parser = CLI.parse(&["--pair=a"]);
    parser.next().unwrap();
    assert_eq!(
        parser.finish(),
        Err(Error::MissingValue {
            arg: "pair",
            index: 0
        })
    );
}

#[test]
fn groups_stop_and_leave_positionals() {
    const CLI: Command = Command::new("demo").args(&[
        Arg::value("pair").values(2, 2),
        Arg::positional("tail").required(),
    ]);
    assert_eq!(
        run(&CLI, &["--pair", "a", "b", "c"]).unwrap()[2],
        Event::Value {
            arg: "tail",
            value: "c"
        }
    );
}

#[test]
fn positional_groups() {
    const CLI: Command = Command::new("demo").args(&[
        Arg::positional("pair").values(2, 2).required(),
        Arg::positional("tail").values(1, 4),
    ]);
    assert_eq!(
        run(&CLI, &["a"]),
        Err(Error::MissingValue {
            arg: "pair",
            index: 0
        })
    );
    assert_eq!(run(&CLI, &["a", "b"]).unwrap().len(), 2);
    assert_eq!(run(&CLI, &["a", "b", "c", "d"]).unwrap().len(), 4);
    assert_eq!(
        run(&CLI, &["a", "b", "c", "d", "e", "f", "g"]),
        Err(Error::UnexpectedArgument { index: 6 })
    );
}

#[test]
fn choices_are_exact() {
    const CLI: Command = Command::new("demo").args(&[
        Arg::value("mode").one_of(&["", "debug", "release"]),
        Arg::positional("target").one_of(&["a", "b"]),
    ]);
    assert_eq!(run(&CLI, &["--mode=", "a"]).unwrap().len(), 2);
    assert_eq!(
        run(&CLI, &["--mode=Debug"]),
        Err(Error::InvalidValue {
            arg: "mode",
            index: 0
        })
    );
    assert_eq!(
        run(&CLI, &["--mode", "other"]),
        Err(Error::InvalidValue {
            arg: "mode",
            index: 1
        })
    );
    assert_eq!(
        run(&CLI, &["c"]),
        Err(Error::InvalidValue {
            arg: "target",
            index: 0
        })
    );
}

#[test]
fn nested_commands_validate_each_scope() {
    const CLI: Command = Command::new("demo")
        .args(&[Arg::flag("root").required()])
        .subcommands(&[Command::new("build")
            .args(&[Arg::flag("child").required()])
            .subcommands(&[Command::new("release").args(&[Arg::positional("path").required()])])]);
    assert_eq!(
        run(&CLI, &["build"]),
        Err(Error::MissingRequired { arg: "root" })
    );
    assert_eq!(
        run(&CLI, &["--root", "build", "release"]),
        Err(Error::MissingRequired { arg: "child" })
    );
    assert_eq!(
        run(&CLI, &["--root", "build", "--child", "release"]),
        Err(Error::MissingRequired { arg: "path" })
    );
    assert_eq!(
        run(&CLI, &["--root", "build", "--child", "release", "x"])
            .unwrap()
            .len(),
        5
    );
    assert_eq!(
        run(&CLI, &["--root", "build", "--root"]),
        Err(Error::Unknown { index: 2 })
    );
}

#[test]
fn repeated_names_in_independent_scopes() {
    const CLI: Command = Command::new("demo")
        .args(&[Arg::flag("v")])
        .subcommands(&[Command::new("child").args(&[Arg::flag("v").required()])]);
    assert_eq!(run(&CLI, &["--v", "child", "--v"]).unwrap().len(), 3);
    assert_eq!(
        run(&CLI, &["--v", "child"]),
        Err(Error::MissingRequired { arg: "v" })
    );
}

#[test]
fn subcommand_and_value_group_boundary() {
    const CLI: Command = Command::new("demo")
        .args(&[Arg::value("list").values(1, usize::MAX)])
        .subcommands(&[Command::new("run")]);
    let events = run(&CLI, &["--list", "a", "run"]).unwrap();
    assert_eq!(events[1], Event::Command { name: "run" });
    assert_eq!(
        run(&CLI, &["--list", "run"]).unwrap(),
        vec![Event::Value {
            arg: "list",
            value: "run"
        }]
    );
    assert_eq!(
        run(&CLI, &["--", "run"]),
        Err(Error::UnexpectedArgument { index: 1 })
    );
}

#[test]
fn subcommand_name_completes_a_started_positional_group() {
    const CLI: Command = Command::new("demo")
        .args(&[Arg::positional("pair").values(2, 3)])
        .subcommands(&[Command::new("run")]);
    assert_eq!(
        run(&CLI, &["a", "run"]).unwrap(),
        vec![
            Event::Value {
                arg: "pair",
                value: "a"
            },
            Event::Value {
                arg: "pair",
                value: "run"
            },
        ]
    );
    assert_eq!(
        run(&CLI, &["a", "b", "run"]).unwrap()[2],
        Event::Command { name: "run" }
    );
}

#[test]
fn unicode_option_names_and_values() {
    const CLI: Command = Command::new("测试").args(&[
        Arg::flag("详细").short('详').repeat(),
        Arg::value("路径").short('路'),
    ]);
    assert_eq!(
        run(&CLI, &["-详详路=文件😀"]).unwrap(),
        vec![
            Event::Flag { arg: "详细" },
            Event::Flag { arg: "详细" },
            Event::Value {
                arg: "路径",
                value: "文件😀"
            },
        ]
    );
    assert_eq!(
        run(&CLI, &["--路径=文件"]).unwrap()[0],
        Event::Value {
            arg: "路径",
            value: "文件"
        }
    );
}

#[test]
fn definition_errors_are_structured() {
    for args in [
        vec![Arg::flag("x"), Arg::value("x")],
        vec![Arg::flag("x").short('x'), Arg::flag("y").short('x')],
        vec![Arg::flag("x").alias("y"), Arg::flag("y")],
        vec![Arg::flag("x").alias("x")],
        vec![Arg::flag("x").short('=')],
        vec![Arg::flag("")],
        vec![Arg::flag("x").long("--x")],
        vec![Arg::flag("x").values(1, 2)],
        vec![Arg::flag("x").one_of(&["x"])],
        vec![Arg::value("x").values(0, 0)],
        vec![Arg::value("x").values(2, 1)],
        vec![Arg::positional("x").short('x')],
        vec![Arg::positional("x").repeat(), Arg::positional("y")],
        vec![Arg::positional("x"), Arg::positional("y").required()],
        vec![Arg::flag("x").requires("y")],
        vec![Arg::flag("x").conflicts("x")],
    ] {
        let cli = Command::new("demo").args(&args);
        assert!(matches!(cli.check(), Err(Error::Definition { .. })));
        let argv: [&str; 0] = [];
        assert!(matches!(
            cli.parse(&argv).next(),
            Err(Error::Definition { .. })
        ));
    }
    assert!(Command::new("").check().is_err());
    assert!(Command::new("demo")
        .subcommands(&[Command::new("x"), Command::new("x")])
        .check()
        .is_err());
}

#[test]
fn large_definitions_and_custom_capacity() {
    let names: Vec<String> = (0..513).map(|i| format!("option{i}")).collect();
    let args: Vec<Arg> = names.iter().map(|s| Arg::flag(s)).collect();
    let empty: [&str; 0] = [];
    for n in [4, 8, 16, 32, 64, 128, 256] {
        let cli = Command::new("demo").args(&args[..n]);
        let token = format!("--option{}", n - 1);
        assert_eq!(run(&cli, &[token.as_str()]).unwrap().len(), 1);
    }
    let cli = Command::new("demo").args(&args[..257]);
    assert_eq!(
        cli.parse(&empty).finish(),
        Err(Error::Capacity {
            needed: 257,
            available: 256
        })
    );
    cli.parse_with::<8, _>(&empty).finish().unwrap();
    let cli = Command::new("demo").args(&args[..513]);
    assert_eq!(
        cli.parse_with::<8, _>(&empty).finish(),
        Err(Error::Capacity {
            needed: 513,
            available: 512
        })
    );
    cli.parse_with::<9, _>(&empty).finish().unwrap();
    Command::new("empty")
        .parse_with::<0, _>(&empty)
        .finish()
        .unwrap();
}

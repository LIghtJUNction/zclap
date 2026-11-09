use zclap::{Arg, Command, Error, Event};

#[test]
fn indexed_lookup_matches_every_unsorted_definition_and_alias() {
    let names: Vec<String> = (0..256)
        .map(|i| format!("key{:03}", (i * 73) % 256))
        .collect();
    for count in [4, 8, 16, 32, 64, 128, 256] {
        let mut args: Vec<Arg> = names[..count].iter().map(|s| Arg::flag(s)).collect();
        args[count - 1] = args[count - 1].alias("替代");
        let cli = Command::new("indexed").args(&args);
        for name in &names[..count] {
            let token = format!("--{name}");
            let argv = [token.as_str()];
            let mut parser = cli.parse(&argv);
            assert_eq!(parser.next().unwrap(), Some(Event::Flag { arg: name }));
            parser.finish().unwrap();
        }
        let mut parser = cli.parse(&["--替代"]);
        assert_eq!(
            parser.next().unwrap(),
            Some(Event::Flag {
                arg: &names[count - 1]
            })
        );
        parser.finish().unwrap();
        for token in ["--", "--unknown", "--key", "--key999", "--key001-more"] {
            let argv = [token];
            let mut parser = cli.parse(&argv);
            if token == "--" {
                assert_eq!(parser.next().unwrap(), None);
            } else {
                assert_eq!(parser.next(), Err(Error::Unknown { index: 0 }));
            }
        }
    }
}

#[test]
fn required_and_relationship_masks_cross_word_boundaries() {
    let names: Vec<String> = (0..256).map(|i| format!("key{i}")).collect();
    let mut args: Vec<Arg> = names.iter().map(|s| Arg::flag(s)).collect();
    let boundaries = [0, 63, 64, 127, 128, 255];
    for &i in &boundaries {
        args[i] = args[i].required();
    }
    args[200] = args[200].requires("key201");
    args[210] = args[210].conflicts("key211");
    let cli = Command::new("masks").args(&args);
    for omitted in boundaries {
        let tokens: Vec<String> = boundaries
            .iter()
            .filter(|&&i| i != omitted)
            .map(|i| format!("--key{i}"))
            .collect();
        let refs: Vec<&str> = tokens.iter().map(String::as_str).collect();
        let mut parser = cli.parse(&refs);
        while parser.next().unwrap().is_some() {}
        assert_eq!(
            parser.finish(),
            Err(Error::MissingRequired {
                arg: &names[omitted]
            })
        );
    }
    let tokens: Vec<String> = boundaries.iter().map(|i| format!("--key{i}")).collect();
    let mut refs: Vec<&str> = tokens.iter().map(String::as_str).collect();
    let mut parser = cli.parse(&refs);
    while parser.next().unwrap().is_some() {}
    parser.finish().unwrap();
    refs.push("--key200");
    let mut parser = cli.parse(&refs);
    while parser.next().unwrap().is_some() {}
    assert_eq!(
        parser.finish(),
        Err(Error::Requires {
            arg: "key200",
            other: "key201"
        })
    );
    refs.push("--key201");
    refs.extend(["--key210", "--key211"]);
    let mut parser = cli.parse(&refs);
    while parser.next().unwrap().is_some() {}
    assert_eq!(
        parser.finish(),
        Err(Error::Conflict {
            arg: "key210",
            other: "key211"
        })
    );
}

#[test]
fn redefining_arguments_rebuilds_metadata() {
    const CLI: Command = Command::new("rebuilt")
        .args(&[Arg::flag("old").required().alias("alias")])
        .args(&[Arg::flag("new")]);
    let argv: [&str; 0] = [];
    CLI.parse(&argv).finish().unwrap();
    assert_eq!(
        CLI.parse(&["--old"]).next(),
        Err(Error::Unknown { index: 0 })
    );
    assert_eq!(
        CLI.parse(&["--alias"]).next(),
        Err(Error::Unknown { index: 0 })
    );
    assert_eq!(
        CLI.parse(&["--new"]).next().unwrap(),
        Some(Event::Flag { arg: "new" })
    );
}

#[test]
fn larger_commands_keep_validation_and_lookup() {
    let names: Vec<String> = (0..513).map(|i| format!("key{i}")).collect();
    let mut args: Vec<Arg> = names.iter().map(|s| Arg::flag(s)).collect();
    args[512] = args[512].required().requires("key511").alias("last");
    let cli = Command::new("large").args(&args);
    let argv: [&str; 0] = [];
    assert_eq!(
        cli.parse_with::<9, _>(&argv).finish(),
        Err(Error::MissingRequired { arg: "key512" })
    );
    let mut parser = cli.parse_with::<9, _>(&["--last"]);
    parser.next().unwrap();
    assert_eq!(
        parser.finish(),
        Err(Error::Requires {
            arg: "key512",
            other: "key511"
        })
    );
    let mut parser = cli.parse_with::<9, _>(&["--last", "--key511"]);
    while parser.next().unwrap().is_some() {}
    parser.finish().unwrap();
}

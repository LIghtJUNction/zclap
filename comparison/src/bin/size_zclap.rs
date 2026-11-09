use zclap::{Arg, Command, Event};
use zclap_comparison::{contribution, name_index, report};

const CLI: Command = Command::new("bench").args(&[
    Arg::value("opt000").required(),
    Arg::flag("opt001"),
    Arg::value("opt002"),
    Arg::flag("opt003"),
]);

fn main() {
    let owned: Vec<_> = std::env::args().skip(1).collect();
    let input: Vec<_> = owned.iter().map(String::as_str).collect();
    report((|| {
        let mut parser = CLI.parse(&input);
        let mut checksum = 0u64;
        while let Some(event) = parser.next().map_err(|_| "parse error")? {
            let (arg, value) = match event {
                Event::Flag { arg } => (arg, None),
                Event::Value { arg, value } => (arg, Some(value)),
                _ => return Err("unexpected command"),
            };
            checksum = checksum.wrapping_add(contribution(name_index(arg)?, value)?);
        }
        parser.finish().map_err(|_| "validation error")?;
        Ok(checksum)
    })());
}

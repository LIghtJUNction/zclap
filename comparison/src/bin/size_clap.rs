use zclap_comparison::{contribution_number, name_index, report};

fn main() {
    let input: Vec<_> = std::env::args().skip(1).collect();
    report((|| {
        let mut command = clap::Command::new("bench")
            .no_binary_name(true)
            .disable_help_flag(true)
            .disable_version_flag(true);
        for (i, name) in ["opt000", "opt001", "opt002", "opt003"]
            .into_iter()
            .enumerate()
        {
            let arg = clap::Arg::new(name).long(name).required(i == 0);
            command = command.arg(if i % 2 == 0 {
                arg.action(clap::ArgAction::Set)
                    .value_parser(clap::value_parser!(u64))
            } else {
                arg.action(clap::ArgAction::SetTrue)
            });
        }
        let matches = command
            .try_get_matches_from(input)
            .map_err(|_| "parse error")?;
        let mut checksum = 0u64;
        for id in matches.ids() {
            if matches.value_source(id.as_str()) != Some(clap::parser::ValueSource::CommandLine) {
                continue;
            }
            let i = name_index(id.as_str())?;
            let numeric = if i % 2 == 0 {
                *matches.get_one::<u64>(id.as_str()).ok_or("missing value")?
            } else {
                1
            };
            checksum = checksum.wrapping_add(contribution_number(i, numeric));
        }
        Ok(checksum)
    })());
}

use zclap_comparison::{Output, report};

fn main() {
    let input: Vec<_> = std::env::args().skip(1).collect();
    report((|| {
        let mut parser = lexopt::Parser::from_args(input);
        let mut output = Output::default();
        while let Some(token) = parser.next().map_err(|_| "parse error")? {
            let i = match token {
                lexopt::Arg::Long("opt000") => 0,
                lexopt::Arg::Long("opt001") => 1,
                lexopt::Arg::Long("opt002") => 2,
                lexopt::Arg::Long("opt003") => 3,
                _ => return Err("unknown option"),
            };
            if i % 2 == 0 {
                let value = parser.value().map_err(|_| "missing value")?;
                output.add(i, Some(value.to_str().ok_or("non-UTF-8 integer")?))?;
            } else {
                output.add(i, None)?;
            }
        }
        output.finish()
    })());
}

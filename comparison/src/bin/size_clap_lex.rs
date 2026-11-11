use zclap_comparison::{Output, report};

fn main() {
    let input: Vec<_> = std::env::args().skip(1).collect();
    report((|| {
        let raw = clap_lex::RawArgs::new(input);
        let mut cursor = raw.cursor();
        let mut output = Output::default();
        while let Some(token) = raw.next(&mut cursor) {
            let (name, attached) = token.to_long().ok_or("unexpected token")?;
            let i = match name {
                Ok("opt000") => 0,
                Ok("opt001") => 1,
                Ok("opt002") => 2,
                Ok("opt003") => 3,
                _ => return Err("unknown option"),
            };
            if i % 2 == 0 {
                let value = attached
                    .or_else(|| raw.next_os(&mut cursor))
                    .ok_or("missing value")?;
                output.add(i, Some(value.to_str().ok_or("non-UTF-8 integer")?))?;
            } else {
                if attached.is_some() {
                    return Err("flag has a value");
                }
                output.add(i, None)?;
            }
        }
        output.finish()
    })());
}

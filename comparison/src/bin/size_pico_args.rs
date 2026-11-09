use zclap_comparison::{report, Output};

fn main() {
    let input: Vec<_> = std::env::args()
        .skip(1)
        .map(std::ffi::OsString::from)
        .collect();
    report((|| {
        let mut parser = pico_args::Arguments::from_vec(input);
        let mut output = Output::default();
        for (i, key) in [(0, "--opt000"), (2, "--opt002")] {
            let value: Option<u64> = parser.opt_value_from_str(key).map_err(|_| "value error")?;
            if let Some(value) = value {
                output.add_number(i, value)?;
            }
        }
        for (i, key) in [(1, "--opt001"), (3, "--opt003")] {
            if parser.contains(key) {
                output.add(i, None)?;
            }
        }
        if !parser.finish().is_empty() {
            return Err("unknown or duplicate option");
        }
        output.finish()
    })());
}

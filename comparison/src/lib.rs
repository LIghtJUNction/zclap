//! Shared output semantics for the standalone comparison programs.

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Output {
    seen: [u64; 4],
    pub checksum: u64,
}

impl Output {
    pub fn add(&mut self, index: usize, value: Option<&str>) -> Result<(), &'static str> {
        let numeric = match value {
            Some(text) => text.parse::<u64>().map_err(|_| "invalid integer")?,
            None => 1,
        };
        self.add_number(index, numeric)
    }

    pub fn add_number(&mut self, index: usize, numeric: u64) -> Result<(), &'static str> {
        let mask = 1u64 << (index % 64);
        let word = &mut self.seen[index / 64];
        if *word & mask != 0 {
            return Err("duplicate option");
        }
        *word |= mask;
        self.checksum = self
            .checksum
            .wrapping_add(contribution_number(index, numeric));
        Ok(())
    }

    pub fn finish(self) -> Result<u64, &'static str> {
        if self.seen[0] & 1 == 0 {
            Err("missing required option")
        } else {
            Ok(self.checksum)
        }
    }
}

pub fn contribution(index: usize, value: Option<&str>) -> Result<u64, &'static str> {
    let numeric = match value {
        Some(text) => text.parse::<u64>().map_err(|_| "invalid integer")?,
        None => 1,
    };
    Ok(contribution_number(index, numeric))
}

pub fn contribution_number(index: usize, numeric: u64) -> u64 {
    (index as u64 + 1)
        .wrapping_mul(0x9e37_79b9)
        .wrapping_add(numeric)
}

pub fn name_index(name: &str) -> Result<usize, &'static str> {
    // This only decodes a library-validated argument ID for result consumption.
    // Lexical adapters use their prebuilt full-name lookup to validate input.
    name.strip_prefix("opt")
        .ok_or("invalid id")?
        .parse()
        .map_err(|_| "invalid id")
}

pub fn report(result: Result<u64, &'static str>) {
    match result {
        Ok(value) => println!("{value}"),
        Err(message) => {
            eprintln!("error: {message}");
            std::process::exit(2);
        }
    }
}

struct Sizes {
    bytes: u64,
    kilobytes: f64,
    megabytes: f64,
    gigabytes: f64,
}

impl Sizes {
    fn convert(units: u64, unit_type: &str) -> Self {
        match unit_type {
            "kb" => Self {
                bytes: units * 1000,
                kilobytes: units as f64,
                megabytes: units as f64 / 1000.0,
                gigabytes: units as f64 / 1_000_000.0,
            },
            "mb" => Self {
                bytes: units * 1_000_000,
                kilobytes: units as f64 * 1000.0,
                megabytes: units as f64,
                gigabytes: units as f64 / 1000.0,
            },
            "gb" => Self {
                bytes: units * 1_000_000_000,
                kilobytes: units as f64 * 1_000_000.0,
                megabytes: units as f64 * 1000.0,
                gigabytes: units as f64,
            },
            _ => panic!("Unknown unit type"),
        }
    }

    fn format(&self) -> String {
        format!(
            "Sizes {{ bytes: {} bytes, kilobytes: {:.0} kilobytes, megabytes: {:.0} megabytes, gigabytes: {:.0} gigabytes }}",
            self.bytes, self.kilobytes, self.megabytes, self.gigabytes
        )
    }
}

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let first_arg = &args[1];
    
    //split first_arg by spaces and print the first part
    let parts: Vec<&str> = first_arg.split_whitespace().collect();
    match parts.as_slice() {
        // Match the case where exactly two values are present.
        [units, unit_type] => {
            match units.parse::<u64>() {
                Ok(units) => {
                    println!("units: {}, unit_type: {}", units, unit_type);
                    let unit_type = unit_type.to_lowercase();
                    let sizes = Sizes::convert(units, &unit_type).format();
                    println!("{:?}", sizes)
                }                   
                Err(_) => println!("Error: units '{}' is not a valid number", units),
            }
        }
        // Match any other case (e.g., too few or too many arguments).
        _ => println!("Error: Invalid number of arguments. Expected 'units unit_type'"),
    }
    
    let result = format_size(6888837399);
    println!("{}", result)
}

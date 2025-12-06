use hashbrown::HashMap;
use hashbrown::hash_map::RawEntryMut;
use memmap2::Mmap;
use rustc_hash::FxBuildHasher;
use std::fs::File;

struct StationStats {
    min: i32,
    max: i32,
    total: i64,
    count: usize,
}

fn aggregate(filename: &str) -> String {
    let file = File::open(filename).expect("Need 'measurements.txt' in the current directory.");
    let mmap = unsafe { Mmap::map(&file).expect("Failed to mmap file") };

    let mut stats = HashMap::<String, StationStats, FxBuildHasher>::default();

    let mut start = 0;
    for (i, &byte) in mmap.iter().enumerate() {
        if byte == b'\n' {
            let line_bytes = &mmap[start..i];
            start = i + 1;

            // Find semicolon position directly in bytes
            let semicolon_pos = line_bytes
                .iter()
                .position(|&b| b == b';')
                .expect("Bad line structure");

            // Station name - skip UTF-8 validation with unchecked
            let station = unsafe { std::str::from_utf8_unchecked(&line_bytes[..semicolon_pos]) };

            // Parse as integer (12.3 becomes 123)
            let reading_bytes = &line_bytes[semicolon_pos + 1..];
            let reading = parse_int(reading_bytes);

            // Update tracking - use raw_entry to avoid allocating String on lookup
            match stats.raw_entry_mut().from_key(station) {
                RawEntryMut::Occupied(mut entry) => {
                    let stats = entry.get_mut();
                    if reading < stats.min {
                        stats.min = reading;
                    } else if reading > stats.max {
                        stats.max = reading;
                    }
                    stats.total += reading as i64;
                    stats.count += 1;
                }
                RawEntryMut::Vacant(entry) => {
                    entry.insert(
                        station.to_owned(),
                        StationStats {
                            min: reading,
                            max: reading,
                            total: reading as i64,
                            count: 1,
                        },
                    );
                }
            }
        }
    }

    format_output(stats)
}

// Fast integer parser - parses "12.3" as 123 (ignoring decimal point)
#[inline]
fn parse_int(bytes: &[u8]) -> i32 {
    let mut result = 0i32;
    let mut negative_toggle = 1;

    for &byte in bytes {
        match byte {
            b'-' => negative_toggle = -1,
            b'.' => continue, // Just skip the decimal point
            b'0'..=b'9' => {
                result = result * 10 + (byte - b'0') as i32;
            }
            _ => break,
        }
    }

    result * negative_toggle
}

fn format_output(stats: HashMap<String, StationStats, FxBuildHasher>) -> String {
    let mut names: Vec<&String> = stats.keys().collect();
    names.sort();

    let mut output = String::from("{");
    for (count, station) in names.into_iter().enumerate() {
        if count != 0 {
            output.push_str(", ");
        }
        let station_stats = stats
            .get(station)
            .expect("Station should exist in the hashmap");

        // Values are already multiplied by 10 (12.3 stored as 123)
        // Apply IEEE 754 roundTowardPositive (ceiling)
        let min = (station_stats.min as f64).ceil() / 10.0;
        let max = (station_stats.max as f64).ceil() / 10.0;
        let mean_times_10 = (station_stats.total as f64 / station_stats.count as f64).ceil();
        let mean = mean_times_10 / 10.0;

        output.push_str(&format!("{station}={:.1}/{:.1}/{:.1}", min, mean, max));
    }
    output.push_str("}\n");

    output
}

fn main() {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "measurements.txt".to_string());
    let output = aggregate(&filename);
    println!("{}", output);
}

#[cfg(test)]
mod test {
    use super::aggregate;

    #[inline(always)]
    fn check_measurements(basename: &str) {
        let reference_name = format!("samples/{}.out", basename);
        let actual_name = format!("samples/{}.txt", basename);

        let reference = String::from_utf8(std::fs::read(reference_name).unwrap()).unwrap();
        let actual = aggregate(&actual_name);

        assert_eq!(reference, actual);
    }

    #[test]
    fn measurements_1() {
        check_measurements("measurements-1");
    }

    #[test]
    fn measurements_10() {
        check_measurements("measurements-10");
    }

    #[test]
    fn measurements_10000_unique_keys() {
        check_measurements("measurements-10000-unique-keys");
    }

    #[test]
    fn measurements_2() {
        check_measurements("measurements-2");
    }

    #[test]
    fn measurements_20() {
        check_measurements("measurements-20");
    }

    #[test]
    fn measurements_3() {
        check_measurements("measurements-3");
    }

    #[test]
    fn measurements_boundaries() {
        check_measurements("measurements-boundaries");
    }

    #[test]
    fn measurements_complex_utf8() {
        check_measurements("measurements-complex-utf8");
    }

    #[test]
    fn measurements_dot() {
        check_measurements("measurements-dot");
    }

    #[test]
    fn measurements_rounding() {
        check_measurements("measurements-rounding");
    }

    #[test]
    fn measurements_short() {
        check_measurements("measurements-short");
    }

    #[test]
    fn measurements_shortest() {
        check_measurements("measurements-shortest");
    }
}

use hashbrown::HashMap;
use hashbrown::hash_map::RawEntryMut;
use rustc_hash::FxBuildHasher;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct StationStats {
    min: f64,
    max: f64,
    total: f64,
    count: usize
}

fn aggregate(filename: &str) -> String {
    let measurements = BufReader::new(File::open(filename).expect("Need 'measurements.txt' in the current directory."));

    let mut stats = HashMap::<String, StationStats, FxBuildHasher>::default();
    for line in measurements.lines() {
        let line = line.expect("Failed to read next line...");

        // Parse line
        let (station, reading) = line.split_once(';').expect("Bad line structure");

        // Parse reading
        let reading = reading.parse::<f64>().expect("Not a floating point number");

        // Update tracking - use raw_entry to avoid allocating String on lookup
        match stats.raw_entry_mut().from_key(station) {
            RawEntryMut::Occupied(mut entry) => {
                let stats = entry.get_mut();
                if reading < stats.min {
                    stats.min = reading;
                } else if reading > stats.max {
                    stats.max = reading;
                }
                stats.total += reading;
                stats.count += 1;
            }
            RawEntryMut::Vacant(entry) => {
                entry.insert(
                    station.to_owned(),
                    StationStats {
                        min: reading,
                        max: reading,
                        total: reading,
                        count: 1,
                    },
                );
            }
        }
    }

    let mut names: Vec<&String> = stats.keys().collect();
    names.sort();

    let mut output = String::from("{");
    for (count, station) in names.into_iter().enumerate() {
        if count != 0 {
            output.push_str(", ");
        }
        let station_stats = stats.get(station).expect("Station should exist in the hashmap");
        let mean = station_stats.total / station_stats.count as f64;

        // Apply IEEE 754 roundTowardPositive (ceiling) to 1 decimal place
        let min = (station_stats.min * 10.0).ceil() / 10.0;
        let max = (station_stats.max * 10.0).ceil() / 10.0;
        let mean = (mean * 10.0).ceil() / 10.0;

        output.push_str(&format!("{station}={:.1}/{:.1}/{:.1}", min, mean, max));
    }
    output.push_str("}\n");

    output
}

fn main() {
    let filename = std::env::args().nth(1).unwrap_or_else(|| "measurements.txt".to_string());
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
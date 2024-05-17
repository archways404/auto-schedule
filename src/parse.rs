use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

const ICS_FILE: &str = "calendar.ics";
const OUTPUT_FILE: &str = "filtered_calendar.ics";

pub fn filter_and_modify_ics() -> io::Result<()> {
    let input = File::open(ICS_FILE)?;
    let reader = io::BufReader::new(input);

    let output = File::create(OUTPUT_FILE)?;
    let mut writer = io::BufWriter::new(output);

    let mut inside_event = false;
    let mut keep_event = true;
    let mut event_lines = Vec::new();
    let mut location_line: Option<String> = None;

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("BEGIN:VEVENT") {
            inside_event = true;
            keep_event = true;
            event_lines.clear();
            location_line = None;
        }

        if inside_event {
            if line.starts_with("LOCATION:") {
                location_line = Some(line.clone());
                if line == "LOCATION:DigimA" {
                    keep_event = false;
                }
            } else if line.starts_with("SUMMARY:") {
                if let Some(location) = &location_line {
                    event_lines.push(location.replace("LOCATION:", "SUMMARY:"));
                    continue;
                }
            }
            event_lines.push(line.clone());
        } else {
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?;
        }

        if line.starts_with("END:VEVENT") {
            inside_event = false;
            if keep_event {
                for event_line in &event_lines {
                    writer.write_all(event_line.as_bytes())?;
                    writer.write_all(b"\n")?;
                }
            }
        }
    }

    Ok(())
}

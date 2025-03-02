use std::error::Error;
use std::path::Path;

use csv::{ByteRecord, ReaderBuilder};
use time::{Date, Month};

/// A named struct representing one row's timestamps and values.
#[derive(Debug)]
pub struct RowData {
    pub timestamps: Vec<i64>,
    pub values: Vec<f64>,
}

/// Parses a CSV file at `path` into a `Vec<RowData>`
/// where each row of the CSV yields one `RowData`.
pub fn parse_csv_file<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<RowData>, Box<dyn Error>> {
    // Build a CSV reader without headers
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(path)?;

    let mut record = ByteRecord::new();

    // Final collection of all rows in the CSV
    let mut dataset = Vec::new();

    // Read each CSV row as raw bytes
    while rdr.read_byte_record(&mut record)? {
        if record.is_empty() {
            continue; // skip empty lines
        }

        // The first column is our base date in MM/DD/YYYY
        let base_date_bytes = record
            .get(0)
            .ok_or("Missing date column")?;
        let base_date_str = std::str::from_utf8(base_date_bytes)?;
        let base_date = parse_mm_dd_yyyy(base_date_str)?;

        // Prepare row-specific vectors
        let mut row_timestamps = Vec::new();
        let mut row_values = Vec::new();

        // For columns [1..], each is a numeric value at "base_date + offset_months"
        // offset_months = 0 for record[1], 1 for record[2], etc.
        for (i, field_bytes) in record.iter().enumerate().skip(1) {
            // Stop reading this row as soon as we find an empty field.
            if field_bytes.is_empty() {
                break;
            }

            let offset_months = (i - 1) as i32;

            // Parse the numeric value
            let val_str = std::str::from_utf8(field_bytes)?;
            let val: f64 = val_str.parse().map_err(|e| {
                format!("Could not parse numeric field '{val_str}' as f64: {e}")
            })?;

            // Compute date = base_date + offset_months
            if let Some(date_i) = add_months(base_date, offset_months) {
                let datetime = date_i
                    .with_hms(0, 0, 0)
                    .expect("Midnight should be valid time");
                // Convert to Unix timestamp
                let ts = datetime.assume_utc().unix_timestamp();

                row_timestamps.push(ts);
                row_values.push(val);
            }
        }

        // Store row's data in the dataset
        dataset.push(RowData {
            timestamps: row_timestamps,
            values: row_values,
        });
    }

    Ok(dataset)
}

/// Parses a "MM/DD/YYYY" string into a `time::Date`.
fn parse_mm_dd_yyyy(s: &str) -> Result<Date, Box<dyn Error>> {
    let parts: Vec<_> = s.split('/').collect();
    if parts.len() != 3 {
        return Err(format!("Invalid date format '{s}'; expected MM/DD/YYYY").into());
    }
    let month_num = parts[0].parse::<u8>()?;
    let day = parts[1].parse::<u8>()?;
    let year = parts[2].parse::<i32>()?;

    let month_enum = Month::try_from(month_num)
        .map_err(|_| format!("Invalid month: {month_num}"))?;

    let date = Date::from_calendar_date(year, month_enum, day)
        .map_err(|e| format!("Invalid date {s}: {e}"))?;

    Ok(date)
}

/// Naive function to add `months` to a `Date`, returning `None` if the resulting
/// day/month combo is invalid (e.g., 31 + February).
fn add_months(base: Date, months: i32) -> Option<Date> {
    let orig_year = base.year();
    let orig_month = base.month() as i32;
    let day = base.day();

    let total_months = orig_year * 12 + (orig_month - 1) + months;
    let new_year = total_months / 12;
    let new_month = (total_months % 12) + 1; // 1..=12

    let new_month_enum = Month::try_from(new_month as u8).ok()?;
    Date::from_calendar_date(new_year, new_month_enum, day).ok()
}

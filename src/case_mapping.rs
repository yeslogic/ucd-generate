use std::collections::BTreeMap;

use ucd_parse::{self, SpecialCaseMapping, UnicodeData, UnicodeDataExpander};

use args::ArgMatches;
use error::Result;

pub fn command(args: ArgMatches) -> Result<()> {
    let dir = args.ucd_dir()?;
    let unicode_data: Vec<UnicodeData> = ucd_parse::parse(dir)?;
    let special_casing: Vec<SpecialCaseMapping> = ucd_parse::parse(dir)?;
    let expanded: Vec<_> = UnicodeDataExpander::new(unicode_data).collect();

    // Collect the mappings
    let mut lowercase = BTreeMap::new();
    let mut uppercase = BTreeMap::new();
    let mut titlecase = BTreeMap::new();
    for row in expanded {
        if let Some(lower) = row.simple_lowercase_mapping {
            if row.codepoint.value() != lower.value() {
                lowercase.insert(row.codepoint.value(), vec![lower.value()]);
            }
        }
        if let Some(upper) = row.simple_uppercase_mapping {
            if row.codepoint.value() != upper.value() {
                uppercase.insert(row.codepoint.value(), vec![upper.value()]);
            }
        }
        if let Some(title) = row.simple_titlecase_mapping {
            if row.codepoint.value() != title.value() {
                titlecase.insert(row.codepoint.value(), vec![title.value()]);
            }
        }
    }

    // Add unconditional mappings from SpecialCasing.txt
    for row in special_casing {
        if !row.conditions.is_empty() {
            // Skip conditional mappings
            continue;
        }

        // Only add if the mapping is not empty and does not map to itself
        if !row.lowercase.is_empty() && row.lowercase != &[row.codepoint.value()] {
            lowercase.insert(
                row.codepoint.value(),
                row.lowercase.iter().map(|cp| cp.value()).collect(),
            );
        }
        if !row.uppercase.is_empty() && row.uppercase != &[row.codepoint.value()] {
            uppercase.insert(
                row.codepoint.value(),
                row.uppercase.iter().map(|cp| cp.value()).collect(),
            );
        }
        if !row.titlecase.is_empty() && row.titlecase != &[row.codepoint.value()] {
            titlecase.insert(
                row.codepoint.value(),
                row.titlecase.iter().map(|cp| cp.value()).collect(),
            );
        }
    }

    let mut wtr = args.writer("case_mapping")?;
    wtr.codepoint_to_codepoints(&(args.name().to_owned() + "_LOWERCASE"), &lowercase)?;
    wtr.codepoint_to_codepoints(&(args.name().to_owned() + "_UPPERCASE"), &uppercase)?;
    wtr.codepoint_to_codepoints(&(args.name().to_owned() + "_TITLECASE"), &titlecase)?;

    Ok(())
}

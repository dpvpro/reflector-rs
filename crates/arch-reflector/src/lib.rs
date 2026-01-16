mod cli;
mod mirrors;

use arch_mirrors_rs::{Mirror, Status};
use chrono::{TimeDelta, Utc};
pub use cli::{Cli, Filters};
use comfy_table::{Cell, CellAlignment, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
use mirrors::{count_countries, get_cache_file, get_mirror_status};

pub async fn run(options: &Cli) {
    if options.list_countries {
        list_countries(&options.url).await;
    }
}

pub fn filter_status<'a>(
    filters: &'a Filters,
    status: &'a Status,
) -> impl Iterator<Item = &'a Mirror> {
    let now = Utc::now();
    let min_completion_pct = filters.completion_percent as f64 / 100.0;
    status.urls.iter().filter(move |mirror| {
        if let Some(last_sync) = mirror.last_sync {
            // Filter by age. The age is given in hours and converted to seconds. Servers
            // with a last refresh older than the age are omitted.
            if let Some(age) = filters.age {
                let max_age =
                    TimeDelta::new((age * 3600.0) as i64, 0).expect("invalid age parameter");
                if age > 0.0 && last_sync + max_age < now {
                    return false;
                }
            }
        } else {
            // Filter unsynced mirrors.
            return false;
        }

        if mirror.last_sync.is_none() {}

        // Filter by completion "percent" [0-1].
        if let Some(completion_pct) = mirror.completion_pct {
            if completion_pct < min_completion_pct {
                return false;
            }
        }

        // Filter by protocols.
        if !filters.protocol.is_empty() {
            if !filters.protocol.contains(&mirror.protocol) {
                return false;
            }
        }

        // Filter by delay. The delay is given as a float of hours and must be
        // converted to seconds.
        if let Some(delay) = filters.delay {
            let max_delay = (delay * 3600.0) as u32;
            if mirror.delay > max_delay {
                return false;
            }
        }

        // Filter by ISO hosing.
        if filters.isos && !mirror.isos {
            return false;
        }

        // Filter by IPv4 support.
        if filters.ipv4 && !mirror.ipv4 {
            return false;
        }

        // Filter by IPv6 support.
        if filters.ipv6 && !mirror.ipv6 {
            return false;
        }

        true
    })
}

pub async fn list_countries(url: &str) {
    let cache_file = get_cache_file(None);
    let status = get_mirror_status(10, 10, url, &cache_file).await.unwrap();
    let counts = count_countries(&status.urls).await;
    let mut sorted = vec![];
    for (country, count) in counts {
        sorted.push((country, count));
    }
    sorted.sort_by(|c1, c2| c1.0.code.cmp(&c2.0.code));

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Country", "Code", "Count"]);

    for (country, count) in sorted {
        table.add_row(vec![
            Cell::new(country.country.to_string()),
            Cell::new(country.code.to_string()),
            Cell::new(count.to_string()).set_alignment(CellAlignment::Right),
        ]);
    }
    println!("{}", table);
}

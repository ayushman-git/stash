use chrono::{DateTime, Local, Utc};

pub fn datetime_humanize(dt: DateTime<Utc>) -> String {
    let local_dt = dt.with_timezone(&Local);
    let diff = Local::now() - local_dt;

    if diff.num_minutes() < 1 {
        "just now".into()
    } else if diff.num_minutes() < 60 {
        format!("{}m ago", diff.num_minutes())
    } else if diff.num_hours() < 24 {
        format!("{}h ago", diff.num_hours())
    } else if diff.num_days() == 1 {
        "yesterday".into()
    } else if diff.num_days() < 7 {
        format!("{}d ago", diff.num_days())
    } else if diff.num_days() < 30 {
        format!("{}w ago", diff.num_days() / 7)
    } else if diff.num_days() < 365 {
        format!("{}mo ago", diff.num_days() / 30)
    } else {
        format!("{}y ago", diff.num_days() / 365)
    }
}
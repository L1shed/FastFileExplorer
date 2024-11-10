use std::time::{SystemTime,UNIX_EPOCH};

const KB: u64 = 1024;
const MB: u64 = KB * 1024;
const GB: u64 = MB * 1024;

// Format file size to string
pub fn format_size(size: u64) -> String {
    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

// Format string to file size
pub fn format_size_from_string(size_str: &str) -> Option<u64> {
    let size_str = size_str.trim();
    let (num_str, unit) = size_str.split_at(size_str.len() - 2);
    
    let number: u64 = num_str.parse().ok()?;
    let multiplier = match unit.to_uppercase().as_str() {
        "KB" => KB,
        "MB" => MB,
        "GB" => GB,
        _ => return None,
    };

    Some(number * multiplier)
}

// pub fn format_size_from_string(size: &str) -> Option<u64> {

//     let size_ext = size[size.len()-2..].to_string().to_uppercase();

//     match size_ext.as_str() {
//         "GB" => {
//             size.replace("GB", "").parse::<u64>()
//                 .ok()
//                 .map(|size| (size * GB ))
//         },
//         "MB" => {
//             size.replace("MB", "").trim().parse::<u64>()
//                 .ok()
//                 .map(|size| (size * MB))
//         },
//         "KB" => {
//             size.replace("KB", "").trim().parse::<u64>()
//                 .ok()
//                 .map(|size| (size * KB))
//         },
//         _ => {
//             // size.replace("B", "").trim().parse::<u64>().ok()
//             None
//         }
//     }
// }

pub fn format_time(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).unwrap();
    let datetime = chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
        .unwrap()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    return datetime;
}
use chrono::{DateTime, Local, TimeZone};
use std::{os::unix::fs::MetadataExt, time::SystemTime};
use walkdir::WalkDir;

// https://users.rust-lang.org/t/convert-std-time-systemtime-to-chrono-datetime-datetime/7684/4
fn _system_time_to_date_time(t: SystemTime) -> DateTime<Local> {
    let (sec, nsec) = match t.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => {
            // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        }
    };
    Local.timestamp(sec, nsec)
}

fn main() {
    for entry in WalkDir::new("src")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let meta = entry.metadata().unwrap();
        let btime = meta.created().unwrap(); // birth time

        // NOTE: To get ctime (last status change time), we need
        // `std::os::unix::fs::MetadataExt`.

        let mtime = meta.modified().unwrap();
        let btime_unix = btime.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let mtime_unix = mtime.duration_since(SystemTime::UNIX_EPOCH).unwrap();

        let btime_dt = Local.timestamp(btime_unix.as_secs() as _, btime_unix.subsec_nanos());
        let mtime_dt = Local.timestamp(mtime_unix.as_secs() as _, mtime_unix.subsec_nanos());

        println!("path={}", entry.path().display());
        for (t, u, dt, label) in [
            (btime, btime_unix, btime_dt, "btime"),
            (mtime, mtime_unix, mtime_dt, "mtime"),
        ] {
            println!(
                "{}={} {}_system={:?} {}_unix={:?} {}_elapsed={:?}",
                label,
                dt.to_rfc3339(),
                label,
                t,
                label,
                u,
                label,
                t.elapsed().unwrap(),
            );
        }
        assert!(btime < mtime);

        let ctime_dt = Local.timestamp(meta.ctime(), meta.ctime_nsec() as _);
        println!("ctime={}", ctime_dt.to_rfc3339());
        let mtime_dt_ext = Local.timestamp(meta.mtime(), meta.mtime_nsec() as _);
        assert_eq!(mtime_dt, mtime_dt_ext);
    }
}

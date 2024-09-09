use std::{
    ffi::OsStr,
    fs,
    io,
    time::Instant,
    process::{
        Command,
        Stdio,
    },
};

fn main() -> io::Result<()> {
    #![expect(clippy::const_is_empty)]

    const TIME_LIMIT_MS: u128 = 200;
    const SKIP: &[&str] = &[
        "p264",
        "p564",
    ];

    let mut success = 0;
    let mut failure = Vec::new();
    let mut timeout = Vec::new();
    let entries =
        fs::read_dir("src/bin")?
            .filter_map(Result::ok);
    for entry in entries {
        let path = entry.path();
        let stem = path
            .file_stem()
            .unwrap_or_default();
        let ext = path
            .extension()
            .unwrap_or_default();
        if  entry.metadata()?.is_file() &&
            ext == "rs"  &&
            stem.as_encoded_bytes().starts_with(b"p") &&
            !SKIP.iter().any(|s| {
                stem.as_encoded_bytes() == s.as_bytes()
            }){
            print!("testing {stem:?} ... ");
            let start_time = Instant::now();
            let is_success = Command::new("cargo")
                .args([
                    OsStr::new("run"),
                    OsStr::new("--bin"),
                    stem,
                    OsStr::new("--release"),
                ])
                .stdin (Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .is_ok_and(|status| status.success());
            let time_elapsed_ms = start_time
                .elapsed()
                .as_millis();
            if is_success {
                print!("ok ");
                success += 1;
            } else {
                print!("failed ");
                failure.push(stem.to_owned());
            }
            print!("in {time_elapsed_ms}ms");
            if time_elapsed_ms >= TIME_LIMIT_MS {
                print!(" (!)");
                timeout.push((stem.to_owned(), time_elapsed_ms));
            }
            println!();
        }
    }

    failure.sort_unstable_by_key(|name| (
        name.len(),
        name.clone()));
    timeout.sort_unstable_by_key(|&(ref name, _)| (
        name.len(),
        name.clone()));

    println!();
    print!("{} skipped (", SKIP.len());
    if !SKIP.is_empty() {
        print!("{:?}", SKIP[0]);
    }
    for name in &SKIP[1..] {
        print!(", {name:?}");
    }
    print!("), ");
    print!("{success} passed, ");
    print!("{} failed:", failure.len());
    println!();
    for name in &failure {
        println!("  - {name:?}");
    }
    if !timeout.is_empty() {
        println!("also:");
        for &(ref name, time_elapsed_ms) in &timeout {
            println!("  - {name:?} took {time_elapsed_ms}ms");
        }
    }

    Ok(())
}

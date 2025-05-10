use ahash::AHashMap;
use rayon::prelude::*;
use std::env;
use std::io;
use std::sync::Mutex;
use std::time::Instant;

fn main() -> io::Result<()> {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Expected two args, got {args.len()}",
        ));
    }
    let content = std::fs::read_to_string(&args[1])?;
    let v1 = Mutex::new(Vec::new());
    let v2 = Mutex::new(Vec::new());
    content.par_lines().for_each(|line| {
        let mut parts = line.split_whitespace().map(|s| s.parse::<u32>().unwrap());
        if let (Some(f), Some(s), None) = (parts.next(), parts.next(), parts.next()) {
            v1.lock().unwrap().push(f);
            v2.lock().unwrap().push(s);
        };
    });
    let mut v1 = v1.into_inner().unwrap();
    let mut v2 = v2.into_inner().unwrap();
    v1.sort_unstable();
    v2.sort_unstable();

    let d: u32 = v1.iter().zip(&v2).map(|(a, &b)| a.abs_diff(b)).sum();
    println!("{d:?}");
    let mut m = AHashMap::new();
    v2.iter().for_each(|val| {
        *m.entry(val).or_insert(0) += 1;
    });
    let d2: u32 = v1.iter().map(|val| m.get(val).unwrap_or(&0) * val).sum();

    println!("{d2:?}");
    println!("total duration {}", now.elapsed().as_micros());
    Ok(())
}

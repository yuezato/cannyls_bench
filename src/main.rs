use cannyls_bench::*;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cannyls_bench")]
struct Opt {
    #[structopt(short = "m", long = "module")]
    bench_module: String, // Storage or Device

    #[structopt(short = "b", long = "bench_name")]
    bench_name: String,

    #[structopt(short = "f", long = "lusf_file")]
    lusf_path: PathBuf,

    #[structopt(short = "j", parse(try_from_str = "parse_with_unit"))]
    journal_size: u64,

    #[structopt(short = "d", parse(try_from_str = "parse_with_unit"))]
    data_region_size: u64,
}

fn unit_to_num(src: &str, suffix: &str, scale: u64) -> Result<u64, std::num::ParseIntError> {
    let v = src.split(suffix).collect::<Vec<&str>>();
    if v.len() == 2 {
        let num: u64 = v[0].parse()?;
        Ok(num * scale)
    } else {
        src.parse()
    }
}

fn parse_with_unit(src: &str) -> Result<u64, std::num::ParseIntError> {
    let unit_with_scale = vec![
        ("K", 1_000),
        ("kb", 1_000),
        ("Kb", 1_000),
        ("M", 1_000_000),
        ("mb", 1_000_000),
        ("Mb", 1_000_000),
        ("G", 1_000_000_000),
        ("gb", 1_000_000_000),
        ("Gb", 1_000_000_000),
    ];

    for (u, s) in unit_with_scale {
        if src.ends_with(u) {
            return unit_to_num(src, u, s);
        }
    }

    src.parse()
}

fn main() {
    let opt = Opt::from_args();
    match &*opt.bench_module {
        "storage" => match &*opt.bench_name {
            "bench1" => {
                storage::bench1(opt.lusf_path, opt.journal_size, opt.data_region_size);
            }
            _ => {
                panic!("not implemented");
            }
        },
        _ => {
            panic!("not implemented");
        }
    }
}

use super::timer;
use cannyls::lump::LumpId;
use cannyls::nvm::FileNvm;
use cannyls::storage::{Storage, StorageBuilder};
use std::path::Path;

pub fn create_storage<P: AsRef<Path>>(
    path: P,
    journal_size: u64,
    data_region_size: u64,
) -> Result<Storage<FileNvm>, cannyls::Error> {
    let capacity: u64 = journal_size + data_region_size;
    let ratio: f64 = journal_size as f64 / capacity as f64;
    let nvm = track!(FileNvm::create(path, capacity))?;
    StorageBuilder::new()
        .journal_region_ratio(ratio)
        .create(nvm)
}

/// PUTした直後にDELETEを行う。
pub fn bench1<P: AsRef<Path>>(path: P, journal_size: u64, data_region_size: u64, r: usize) {
    let storage = create_storage(path, journal_size, data_region_size).unwrap();
    println!("{:?}", storage.header());
    bench1_inner(storage, super::man!(10), 512, r);
}

fn bench1_inner(mut storage: Storage<FileNvm>, iter: usize, data_size: usize, r: usize) {
    let mut percentile = Percentile::new();
    let _timer = timer::Timer::new(&format!(
        "bench1 with iter={:?}, data_size={:?}, r={:?}",
        iter, data_size, r
    ));
    let lump_data = storage
        .allocate_lump_data_with_bytes(vec![42; data_size].as_slice())
        .unwrap();
    for i in 0..iter {
        let lump_id = LumpId::new(i as u128);

        percentile.record(|| {
            storage.put(&lump_id, &lump_data).unwrap();
            storage.delete(&lump_id).unwrap();
        });

        if i % r == 0 {
            storage.release_all_releasable_portions();
        }
    }
    percentile.result();
}

struct Percentile {
    micros: Vec<u128>,
}

impl Percentile {
    pub fn new() -> Self {
        Percentile { micros: Vec::new() }
    }

    pub fn record<F>(&mut self, f: F)
    where
        F: FnOnce() -> (),
    {
        let now = std::time::Instant::now();
        f();
        self.micros.push(now.elapsed().as_micros());
    }

    pub fn result(&mut self) {
        use std::time::Duration;

        self.micros.sort_unstable();

        let pos_9 = (0.09 * self.micros.len() as f64) as usize;
        let pos_50 = (0.50 * self.micros.len() as f64) as usize;
        let pos_99 = (0.90 * self.micros.len() as f64) as usize;
        let pos_999 = (0.99 * self.micros.len() as f64) as usize;

        println!(
            "9% = {:?}",
            Duration::from_micros(self.micros[pos_9] as u64)
        );
        println!(
            "50% = {:?}",
            Duration::from_micros(self.micros[pos_50] as u64)
        );
        println!(
            "99% = {:?}",
            Duration::from_micros(self.micros[pos_99] as u64)
        );
        println!(
            "99.9% = {:?}",
            Duration::from_micros(self.micros[pos_999] as u64)
        );
    }
}

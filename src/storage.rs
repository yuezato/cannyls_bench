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
pub fn bench1<P: AsRef<Path>>(path: P, journal_size: u64, data_region_size: u64) {
    let storage = create_storage(path, journal_size, data_region_size).unwrap();
    println!("{:?}", storage.header());
    bench1_inner(storage, super::man!(10), 512);
}

fn bench1_inner(mut storage: Storage<FileNvm>, iter: usize, data_size: usize) {
    let _timer = timer::Timer::new(&format!("bench1 with {:?} and {:?}", iter, data_size));
    let lump_data = storage
        .allocate_lump_data_with_bytes(vec![42; data_size].as_slice())
        .unwrap();
    for i in 0..iter {
        let lump_id = LumpId::new(i as u128);
        storage.put(&lump_id, &lump_data).unwrap();
        storage.delete(&lump_id).unwrap();

        if i % 100 == 0 {
            storage.run_side_job_once().unwrap();
        }
    }
}

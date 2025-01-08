use libparted::Disk;

pub mod disk_select;
pub mod helpers;
pub mod partition_manager;

fn main() {
    let mut device = disk_select::select_disk();
    let sector_size = device.sector_size();
    println!("Selected device: {}", device.model());
    println!("{:?}", device.path());
    let disk = Disk::new(&mut device).expect("Failed to read disk from device");
    println!("{:?}", unsafe { disk.get_device().path() });
    partition_manager::select_partitions(disk, sector_size);
}

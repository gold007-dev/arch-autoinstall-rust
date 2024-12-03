use libparted::Disk;

pub mod helpers;
pub mod disk_select;
pub mod partition_manager;

fn main(){
    let mut device = disk_select::select_disk();
    let sector_size=device.sector_size();
    let disk = Disk::new(&mut device).expect("Failed to read disk from device");
    partition_manager::select_partitions(disk, sector_size);
}


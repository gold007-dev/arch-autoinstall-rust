use libparted::Disk;
use crate::helpers;

pub fn select_partitions(disk: Disk, sector_size: u64) {
    println!("Disk of type {}:", disk.get_disk_type_name().unwrap());
    let parts: Vec<_> = disk.parts().collect();
    println!("This disk already has {} partitions", parts.len());
    for partition in parts {
        println!(
            "Partition: Name=\"{}\" Start={}, Ende={}, Grösse={}",
            partition.name().unwrap_or("unnamed partition".to_string()),
            partition.geom_start(),
            partition.geom_end(),
            helpers::get_formatted_size(partition.geom_length() as f64*sector_size as f64, 0)
        );
    }
}

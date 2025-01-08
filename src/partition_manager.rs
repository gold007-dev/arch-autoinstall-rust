use crate::helpers::{self};
use inquire::Confirm;
use libparted::{Disk, Partition};

pub fn select_partitions(mut disk: Disk, sector_size: u64) {
    println!("Disk of type {}:", disk.get_disk_type_name().unwrap());
    let all_parts: Vec<_> = disk.parts().collect();
    let valid_parts: Vec<&Partition> = all_parts
        .iter()
        .filter(|partition| {
            let path = partition.get_path();
            path.is_some() && path.unwrap() != std::path::Path::new("/dev/null")
        })
        .collect::<Vec<&Partition>>();
    println!("This disk already has {} partitions", valid_parts.len());
    for partition in valid_parts {
        println!(
            "Partition: Name=\"{}\" Start={}, End={}, Size={}",
            partition.name().unwrap_or("unnamed partition".to_string()),
            partition.geom_start(),
            partition.geom_end(),
            helpers::get_formatted_size(partition.geom_length() as f64 * sector_size as f64, 0)
        );
        // println!("{:?}",partition.get_path().unwrap_or(std::path::Path::new("/dev/null")));
    }

    // let device = unsafe { disk.get_device() };
    let enough_free_space = helpers::check_free_space(&disk, &mut [32,1,4]);

    if enough_free_space {
        println!("the disk has enough space.");
    }else {
        println!("the disk doesn't have enough space.");
    }

    let clear_disk = Confirm::new("Do you want to remove all partitions?")
        .with_default(false)
        .with_help_message("This action can not be undone")
        .prompt()
        .unwrap();

    std::mem::drop(all_parts);
    if clear_disk {
        let res = disk.delete_all();
        match res {
            Ok(_) => println!("All partitions removed successfully"),
            Err(e) => println!("Error removing partitions: {}", e),
        }
    } else {
    }
}

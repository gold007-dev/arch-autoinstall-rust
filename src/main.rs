use inquire::{InquireError, Select};
use libparted::{self, Device, Disk};
fn main() {
    let devices = Device::devices(true);
    let mut devices_list: Vec<Device<'_>> = vec![];
    let mut device_names: Vec<&str> = vec![];
    let mut model_string_list: Vec<String> = vec![];

    for mut device in devices {
        let model = device.model().to_string();
        let disk = Disk::new(&mut device).expect("Cant open disk");
        if disk.get_disk_type_name().unwrap() == "loop" {
            continue;
        }
        std::mem::drop(disk);

        println!("{} GB", device.length()*device.sector_size()/1024/1024/1024);
        model_string_list.push((device.length()*device.sector_size()/1024/1024/1024).to_string()+" GB - "+&model);
        devices_list.push(device);
    }

    //convert Vec<String> to Vec<&str>
    for model in &model_string_list {
        device_names.push(model);
    }

    let ans: Result<&str, InquireError> =
        Select::new("Which device do you want?", device_names.clone()).prompt();

    match ans {
        Ok(choice) => {
            if let Some(index) = device_names.clone().iter().position(|&name| name == choice) {
                let device = &mut devices_list[index];
                let sector_size = device.sector_size();
                let result = Disk::new(device);
                let disk: Disk<'_> = result.expect("error");
                println!("Disk of type {}:", disk.get_disk_type_name().unwrap());
                for partition in disk.parts() {
                    println!(
                        "Partition: Start={}, Ende={}, Grösse={}",
                        partition.geom_start(),
                        partition.geom_end(),
                        get_formatted_size(partition.geom_length() as f64*sector_size as f64, 0)
                    );
                }
            } else {
                panic!("Device not found! exiting");
            }
        }
        Err(_) => println!("There was an error, please try again"),
    }
}

fn get_formatted_size(size: f64, upgrades:u64) -> String {
    if size/1024.0>10.0 {
        return get_formatted_size(size/1024 as f64,upgrades+1);
    }else {
        return size.to_string()+" "+&upgrades.to_string();
    }
}
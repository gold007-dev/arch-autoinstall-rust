use inquire::{InquireError, Select};
use libparted::{self, Device, Disk};
use crate::helpers;
pub fn select_disk()-> libparted::Device<'static>{
    let devices = Device::devices(true);
    let mut model_string_list: Vec<String> = vec![];
    let mut devices_list: Vec<Box<Device<'_>>> = vec![];
    let mut device_names: Vec<&str> = vec![];

    for mut device in devices {
        let model = device.model().to_string();
        let disk = Disk::new(&mut device).expect("Cant open disk");
        if disk.get_disk_type_name().unwrap() == "loop" {
            continue;
        }
        std::mem::drop(disk);

        println!("{} GB", device.length()*device.sector_size()/1024/1024/1024);
        model_string_list.push(helpers::get_formatted_size((device.length()*device.sector_size()) as f64,0)+" - "+&model);
        devices_list.push(Box::new(device));
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
                println!("{:?}",devices_list[index].path());
                let device =Device::new(devices_list[index].path()).expect("Error opening device");
                println!("Selected device: {}", device.model());
                println!("{:?}",device.path());
                return device;
            } else {
                panic!("Device not found! exiting");
            }
        }
        Err(_) => panic!("There was an error, please try again"),
    }
}
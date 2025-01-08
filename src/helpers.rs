use libparted::{Disk, Partition};

pub fn get_formatted_size(size: f64, upgrades: u64) -> String {
    let mut sizes: Vec<&str> = Vec::new();
    sizes.push("B");
    sizes.push("KB");
    sizes.push("MB");
    sizes.push("GB");
    sizes.push("TB");
    if size / 1024.0 > 10.0 {
        return get_formatted_size(size / 1024 as f64, upgrades + 1);
    } else {
        return ((size * 100.0).round() / 100.0).to_string() + " " + sizes[upgrades as usize];
    }
}

// pub fn find_free_space(disk: &Disk, sector_size: u64) {
//     let device = unsafe { disk.get_device() };
//     let total_sectors = device.length();
//     let mut occupied_ranges: Vec<(u64, u64)> = vec![];

//     // Alle Partitionen durchlaufen und Start-/End-Bereiche sammeln
//     for partition in disk.parts() {
//         occupied_ranges.push((partition.geom_start().try_into().unwrap(), partition.geom_end().try_into().unwrap()));
//     }

//     // Sortiere die Bereiche nach Startsektor
//     occupied_ranges.sort_by_key(|range| range.0);

//     println!("Occupied ranges: {:?}", occupied_ranges);

//     // Freie Bereiche zwischen den Partitionen finden
//     let mut free_ranges = vec![];
//     let mut last_end = 0;

//     for &(start, end) in &occupied_ranges {
//         if start > last_end {
//             // Freier Bereich gefunden
//             free_ranges.push((last_end, start - 1));
//         }
//         last_end = end + 1;
//     }

//     // Freien Bereich am Ende des Geräts prüfen
//     if last_end < total_sectors {
//         free_ranges.push((last_end, total_sectors - 1));
//     }

//     // Freie Bereiche ausgeben
//     for (start, end) in free_ranges {
//         let size_in_bytes = (end - start + 1) * sector_size;
//         println!(
//             "Free space: Start={} End={} Size={}",
//             start,
//             end,
//             get_formatted_size(size_in_bytes as f64, 0)
//         );
//     }
// }

// pub fn check_free_space(disk: &Disk, sector_size: u64, required_sizes: &[u64]) {
//     let device = unsafe { disk.get_device() };
//     let total_sectors = device.length();
//     let mut occupied_ranges: Vec<(u64, u64)> = vec![];

//     // Alle gültigen Partitionen durchlaufen und Start-/End-Bereiche sammeln
//     for partition in disk.parts() {
//         if let Some(path) = partition.get_path() {
//             if path != std::path::Path::new("/dev/null") {
//                 occupied_ranges.push((
//                     partition.geom_start().try_into().unwrap(),
//                     partition.geom_end().try_into().unwrap(),
//                 ));
//             }
//         }
//     }

//     // Sortiere die Bereiche nach Startsektor
//     occupied_ranges.sort_by_key(|range| range.0);

//     // Freie Bereiche finden
//     let mut free_ranges = vec![];
//     let mut last_end = 0;

//     for &(start, end) in &occupied_ranges {
//         if start > last_end {
//             // Freier Bereich gefunden
//             free_ranges.push((last_end, start - 1));
//         }
//         last_end = end + 1;
//     }

//     // Freien Bereich am Ende des Geräts prüfen
//     if last_end < total_sectors {
//         free_ranges.push((last_end, total_sectors - 1));
//     }

//     // Größen in Bytes prüfen
//     let required_bytes: Vec<u64> = required_sizes.iter().map(|&gb| gb * 1024 * 1024 * 1024).collect();

//     // Prüfen, ob alle erforderlichen Größen durch zusammenhängende Bereiche erfüllt werden können
//     let mut remaining_bytes: Vec<u64> = required_bytes.clone();

//     for &(start, end) in &free_ranges {
//         let free_size = (end - start + 1) * sector_size;

//         // Überprüfen, ob dieser freie Bereich für eine der verbleibenden Größen geeignet ist
//         remaining_bytes.retain(|&size| size > free_size);

//         // Wenn keine Größen mehr übrig sind, können wir abbrechen
//         if remaining_bytes.is_empty() {
//             println!(
//                 "Freier Bereich vorhanden, um alle Anforderungen ({:?} GB) zu erfüllen.",
//                 required_sizes
//             );
//             return;
//         }
//     }

//     // Falls Anforderungen übrig bleiben
//     if !remaining_bytes.is_empty() {
//         println!(
//             "Nicht genügend zusammenhängender freier Bereich vorhanden, um folgende Größen zu erfüllen: {:?} GB.",
//             remaining_bytes.iter().map(|&b| b / 1024 / 1024 / 1024).collect::<Vec<_>>()
//         );
//     }
// }

pub fn check_free_space(disk: &Disk, required_sizes: &mut [i64])->bool {
    let all_parts: Vec<_> = disk.parts().collect();
    let mut valid_parts: Vec<&Partition> = all_parts
        .iter()
        .filter(|partition| {
            let path = partition.get_path();
            path.is_some() && path.unwrap() != std::path::Path::new("/dev/null")
        })
        .collect::<Vec<&Partition>>();
    let device = unsafe { disk.get_device() };

    let binding = Partition::new(
        disk,
        libparted::PartitionType::PED_PARTITION_NORMAL,
        None,
        device.length().try_into().unwrap(),
        device.length().try_into().unwrap(),
    )
    .unwrap();
    valid_parts.push(&binding);
    println!("This disk already has {} partitions", valid_parts.len());
    let sector_size: i64 = device.sector_size().try_into().unwrap();

    required_sizes.sort_by(|a, b| b.cmp(a));

    let mut required_sectors: Vec<i64> = required_sizes
        .iter()
        .map(|size| size * 1024 * 1024 * 1024 / sector_size)
        .collect();

    let mut previous_end = 0;
    for partition in valid_parts {
        let mut to_remove = vec![];
        let mut removed = true;
        while removed {
            removed = false;
            for size in &*required_sectors {
                if partition.geom_start() - previous_end >= *size {
                    to_remove.push(*size);
                    removed = true;
                    previous_end += size;

                    println!(
                        "found space for a {}GB partition",
                        size * sector_size / 1024 / 1024 / 1024
                    );

                    break;
                }
            }
            required_sectors.retain(|x| !to_remove.contains(x));
        }
        previous_end = partition.geom_end();
    }

    required_sectors.len()==0
}

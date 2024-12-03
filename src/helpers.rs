pub fn get_formatted_size(size: f64, upgrades:u64) -> String {
    let mut sizes:Vec<&str> = Vec::new();
    sizes.push("B");
    sizes.push("KB");
    sizes.push("MB");
    sizes.push("GB");
    sizes.push("TB");
    if size/1024.0>10.0 {
        return get_formatted_size(size/1024 as f64,upgrades+1);
    }else {
        return ((size * 100.0).round() / 100.0).to_string()+" "+sizes[upgrades as usize];
    }
}
use std::{io::Write, path::PathBuf};

fn main() {
    let config = std::fs::read_to_string("config.cfg").unwrap();
    let lines = config.lines()
        .map(|line| line.split_once(":").unwrap().1.trim())
        .collect::<Vec<_>>();

    let src = lines[0];

    if !std::fs::metadata(src).is_ok() {
        println!("Die SD-Karte wurde nicht gefunden.");
        println!("Drücke ENTER um das Programm zu beenden.");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }

    let dst = lines[1];
    let dst_name = get_dst_name();
    
    move_images(src, &dst, &dst_name);
}

fn move_images(src: &str, dst: &str, dst_name: &str) {
    let full_dst = format!("{}\\{}", dst, dst_name);
    println!("Kopiere Dateien von {} nach {}.", src, full_dst);
    
    if !std::fs::metadata(&full_dst).is_ok() {
        std::fs::create_dir(&full_dst).unwrap();
    }

    let total_num_files = get_total_num_files(src);
    let progress_bar = indicatif::ProgressBar::new(total_num_files as u64);
    
    let full_dst = format!("{}\\{}", dst, dst_name);
    let folders_at_src: Vec<_> = get_subdirectories(src).collect();

    std::thread::scope(|s| {
        for folder in folders_at_src {
            s.spawn(|| {
                let folder = folder;  
                let files_at_folder = std::fs::read_dir(&folder).unwrap();
                for file in files_at_folder.filter_map(|entry| entry.ok()) {
                    let file_name = file.file_name();
                    let (file_name,file_end) = file_name.to_str().unwrap().split_once('.').unwrap();
                    let full_path = format!("{}\\{}", &full_dst, file_end);
                    let full_file_name = format!("{}\\{}-{}.{}", &full_path, &file_name, folder.file_name().unwrap().to_str().unwrap(), file_end);
                    if !std::fs::metadata(&full_path).is_ok() {
                        std::fs::create_dir(&full_path).unwrap();
                    }
                    std::fs::copy(file.path(), &full_file_name).unwrap();
                    progress_bar.inc(1);
                }
            });
        }
    });

    progress_bar.finish_and_clear();
    println!("Kopieren abgeschlossen.");
    println!("Drücke ENTER um das Programm zu beenden.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn get_dst_name() -> String {
    println!("Gib den Namen des Ordners ein in den die Bilder kopiert werden sollen.");
    print!("Name: ");
    std::io::stdout().flush().unwrap();
    let mut dst_name = String::new();
    std::io::stdin().read_line(&mut dst_name).unwrap();
    dst_name.trim().to_string()
}

fn get_subdirectories(path: &str) -> impl Iterator<Item = PathBuf> {
    std::fs::read_dir(path).unwrap().filter_map(|entry| {
        match entry {
            Ok(entry) if entry.path().is_dir() => {
                Some(entry.path())
            }
            _ => None
        }
    })
}

fn get_total_num_files(path: &str) -> usize {
    let mut total = 0;
    let folders = get_subdirectories(path);
    for folder in folders {
        let files = std::fs::read_dir(&folder).unwrap();
        total += files.count();
    }
    total
}

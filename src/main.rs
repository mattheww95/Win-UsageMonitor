/*
 Currently this is not handling reasizing of the window
*/


use sysinfo::{
    System, RefreshKind, CpuRefreshKind
};
use std::{thread, time::Duration};
use windows::Win32::{Foundation::{GENERIC_READ, GENERIC_WRITE, HANDLE}, System::Console::{self, CreateConsoleScreenBuffer, SetConsoleActiveScreenBuffer, SetCurrentConsoleFontEx, WriteConsoleOutputCharacterW, CONSOLE_TEXTMODE_BUFFER}};

fn get_status_chr(input_val: f32, comp_val: f32) -> char {
    let mut status_chr = '✅';
    if input_val > comp_val{
        status_chr = '❌';
    }
    status_chr

}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let n_screen_width = 120;
    let n_screen_height = 480;
    //let mut screen: Vec<u16> = std::vec::Vec::with_capacity(n_screen_height * n_screen_width);
    let mut screen: Vec<u16> = vec![' ' as u16; n_screen_height * n_screen_width];
    let mut chars_written: u32 = 0;
    unsafe {
        let access_writes: windows::Win32::Foundation::GENERIC_ACCESS_RIGHTS = GENERIC_READ | GENERIC_WRITE;
        let h_console: HANDLE = CreateConsoleScreenBuffer(access_writes.0, 0, None, CONSOLE_TEXTMODE_BUFFER, None)?;
        let _ = SetConsoleActiveScreenBuffer(h_console);
        //let _ = SetCurrentConsoleFontEx(h_console, false, lpconsolecurrentfontex)?;

        let mut s = System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),);
        loop {
            let mut idx: usize = 0;
            let mut row: usize = 0;
            thread::sleep(Duration::from_secs(1));
            s.refresh_all();



            for cpu in s.cpus() {
                let cpu_usage = cpu.cpu_usage();
                let status_chr = get_status_chr(cpu_usage, 80.0);
                let cpu_usage = format!("{:<6} - % {:<6.2} {}", cpu.name(), cpu_usage, status_chr);
                for chr in cpu_usage.chars() {
                    screen[row + idx] = chr as u16;
                    idx+=1;
                }
                row += n_screen_width;
                idx = 0; // advance to next line
            }

            let free_memory: u64 = s.free_memory();
            let total_memory: u64 = s.total_memory();
            let memory: f32 = (free_memory as f32 / total_memory as f32) * 100.0;
            let status_chr = get_status_chr(memory, 75.0);
            let memory_usage = format!("Memory Usage: % {:.2} {}", memory, status_chr);
            for chr in memory_usage.chars() {
                screen[row + idx] = chr as u16;
                idx+=1;
            }
            let _ = WriteConsoleOutputCharacterW(h_console, &screen, Console::COORD { X: 0, Y: 0 }, &mut chars_written);
            screen.fill(' ' as u16);
        }
    }

}

#![no_main]
#![no_std]

extern crate alloc;

use uefi::prelude::*;
use uefi::table::boot::OutputBuffer;
use uefi::proto::console::text::{Key, TextInputProtocol};
use uefi::Status;
use alloc::string::String;
use alloc::format;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Инициализируем сервисы UEFI
    uefi_services::init(&system_table).expect_success("Failed to initialize UEFI services");

    // Выводим приветствие
    system_table.stdout().output_string(cstr16!("Welcome to our custom OS (Vibe Code Edition)!")).unwrap();
    system_table.stdout().output_string(cstr16!("\r\n")).unwrap();
    system_table.stdout().output_string(cstr16!("Type 'help' for available commands\r\n")).unwrap();

    // Главный цикл обработки команд
    loop {
        system_table.stdout().output_string(cstr16!("> ")).unwrap();
        
        let command = read_line(&mut system_table);
        
        match command.trim().to_lowercase().as_str() {
            "help" => {
                print_help(&mut system_table.stdout());
            },
            "restart" => {
                restart_system(system_table.boot_services());
            },
            "mandelbrot" => {
                draw_mandelbrot(&mut system_table.stdout());
            },
            "calculator" => {
                run_calculator(&mut system_table.stdout());
            },
            "timedatectl" => {
                show_time_date(&mut system_table.stdout());
            },
            "clear" => {
                // Очистка экрана (ограниченно поддерживается в UEFI)
                for _ in 0..50 {
                    system_table.stdout().output_string(cstr16!("\r\n")).unwrap();
                }
            },
            "exit" | "quit" => {
                system_table.stdout().output_string(cstr16!("Goodbye!\r\n")).unwrap();
                break;
            },
            "" => continue, // Пустая команда, продолжаем
            _ => {
                system_table.stdout().output_string(cstr16!("Unknown command: ")).unwrap();
                system_table.stdout().output_string(unsafe { &cstr16!(command.as_str()) }).unwrap();
                system_table.stdout().output_string(cstr16!("\r\n")).unwrap();
                system_table.stdout().output_string(cstr16!("Type 'help' for available commands\r\n")).unwrap();
            }
        }
    }

    Status::SUCCESS
}

// Функция для чтения строки ввода
fn read_line(system_table: &mut SystemTable<Boot>) -> String {
    let mut input = String::new();
    let stdin = system_table.stdin_mut();
    
    loop {
        match stdin.read_key_stroke() {
            Ok(key) => {
                match key {
                    Key::Printable(ch) => {
                        // Выводим символ
                        let ch_str = format!("{}", ch);
                        system_table.stdout().output_string(unsafe { &cstr16!(ch_str.as_str()) }).unwrap();
                        
                        // Добавляем в буфер ввода
                        input.push(ch);
                    },
                    Key::Special(8) | Key::Special(127) => { // Backspace
                        if !input.is_empty() {
                            input.pop();
                            // Визуальное удаление символа
                            system_table.stdout().output_string(cstr16!("\x08 \x08")).unwrap();
                        }
                    },
                    Key::Special(13) => { // Enter
                        system_table.stdout().output_string(cstr16!("\r\n")).unwrap();
                        break;
                    },
                    _ => {} // Игнорируем другие клавиши
                }
            },
            Err(_) => continue, // Продолжаем пока не получим клавишу
        }
    }
    
    input
}

// Функция вывода справки
fn print_help(stdout: &mut OutputBuffer) {
    stdout.output_string(cstr16!("Available commands:\r\n")).unwrap();
    stdout.output_string(cstr16!("  help         - Show this help message\r\n")).unwrap();
    stdout.output_string(cstr16!("  restart      - Restart the system\r\n")).unwrap();
    stdout.output_string(cstr16!("  mandelbrot   - Draw Mandelbrot fractal\r\n")).unwrap();
    stdout.output_string(cstr16!("  calculator   - Simple calculator\r\n")).unwrap();
    stdout.output_string(cstr16!("  timedatectl  - Show system time and date\r\n")).unwrap();
    stdout.output_string(cstr16!("  clear        - Clear the screen\r\n")).unwrap();
    stdout.output_string(cstr16!("  exit/quit    - Exit the OS\r\n")).unwrap();
}

// Функция перезагрузки системы
fn restart_system(boot_services: &uefi::table::boot::BootServices) -> ! {
    use uefi::table::runtime::ResetType;
    
    boot_services.stall(1000000); // Ждем 1 секунду
    boot_services.reset_system(ResetType::Cold, Status::SUCCESS, None);
    
    // Должно быть невозможно достичь этой точки
    loop {
        unsafe { core::arch::asm!("nop") };
    }
}

// Функция отрисовки фрактала Мандельброта
fn draw_mandelbrot(stdout: &mut OutputBuffer) {
    stdout.output_string(cstr16!("Drawing Mandelbrot fractal...\r\n")).unwrap();
    
    // Простое текстовое представление множества Мандельброта
    let width = 60;
    let height = 20;
    
    for y in 0..height {
        for x in 0..width {
            let cx = (x as f64 - width as f64 / 2.0) * 4.0 / width as f64;
            let cy = (y as f64 - height as f64 / 2.0) * 4.0 / height as f64;
            
            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut iter = 0;
            let max_iter = 50;
            
            while iter < max_iter && zx * zx + zy * zy <= 4.0 {
                let tmp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = tmp;
                iter += 1;
            }
            
            let ch = if iter == max_iter { '#' } else if iter > 30 { '*' } else if iter > 15 { '+' } else if iter > 5 { '.' } else { ' ' };
            let ch_str = format!("{}", ch);
            stdout.output_string(unsafe { &cstr16!(ch_str.as_str()) }).unwrap();
        }
        stdout.output_string(cstr16!("\r\n")).unwrap();
    }
}

// Функция простого калькулятора
fn run_calculator(stdout: &mut OutputBuffer) {
    stdout.output_string(cstr16!("Simple Calculator (type 'exit' to quit)\r\n")).unwrap();
    stdout.output_string(cstr16!("Enter expressions like: 5 + 3, 10 - 2, etc.\r\n")).unwrap();
    
    // Для простоты, симулируем вычисления
    stdout.output_string(cstr16!("Example calculations:\r\n")).unwrap();
    stdout.output_string(cstr16!("2 + 2 = 4\r\n")).unwrap();
    stdout.output_string(cstr16!("5 * 6 = 30\r\n")).unwrap();
    stdout.output_string(cstr16!("10 - 3 = 7\r\n")).unwrap();
    stdout.output_string(cstr16!("Calculator simulation complete.\r\n")).unwrap();
}

// Функция отображения даты и времени
fn show_time_date(stdout: &mut OutputBuffer) {
    // В реальной реализации мы бы получали время из сервисов UEFI runtime
    stdout.output_string(cstr16!("Current date and time: [UEFI Runtime Services Needed]\r\n")).unwrap();
    stdout.output_string(cstr16!("System uptime: [Not implemented]\r\n")).unwrap();
    stdout.output_string(cstr16!("Time and date display simulated.\r\n")).unwrap();
}

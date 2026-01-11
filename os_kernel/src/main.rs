#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi::println;

#[entry]
fn main(_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    // Инициализируем сервисы UEFI
    uefi_services::init(&system_table).expect_success("Failed to initialize UEFI services");

    // Выводим приветствие
    println!("Simple UEFI OS loaded successfully!");

    // Отображаем приглашение
    println!("> ");

    // Цикл обработки команд
    loop {
        // Здесь будет реализован цикл обработки команд
        // Пока просто выводим приглашение снова
        println!("Available commands: restart, mandelbrot, calculator, timedatectl");
        println!("> ");
        
        // Для настоящей реализации потребуется обработка ввода с клавиатуры
        // и реализация самих команд
        
        break; // Временно выходим из цикла для тестирования
    }

    Status::SUCCESS
}

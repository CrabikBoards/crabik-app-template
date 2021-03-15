// Файл с основным кодом программы

#![no_main] // отключение стандартной "точки старта" от компилятора
#![no_std] // отключение стандартной библиотеки

use crabik_board::{
    // модуль содержащий билеотеку hal для nrf52810
    hal::{
        prelude::*, // модуль с ранообразными обьявлениями для удобной работы
        Clocks,     // модуль для настройки тактирования микроконтроллера
    },
    pac::Peripherals,
    CrabikPins,
    Logger,
}; // импорт библеотеки для платы crabik
use rtt_target::*; // импорт библеотеки для работы с протоколом RTT


{% if rtic_enabled %}
#[rtic::app(device = crabik_board, peripherals = true,  monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    #[init]
    fn init(mut ctx: init::Context) {
        let clocks = Clocks::new(ctx.device.CLOCK); // передача переферии для настройки тактирования микроконтроллера
        let clocks = clocks.enable_ext_hfosc(); // использовать внешний источник (кварц) для тактирования микроконтроллера
        let clocks = clocks.set_lfclk_src_rc(); // использовать внутренний источник (кварц) для тактирования Rtc (часов)

        ctx.core.DCB.enable_trace();
        ctx.core.DWT.enable_cycle_counter();

        rtt_init_print!(NoBlockSkip, 16384); // инициализация канала RTT

        log::set_logger(&Logger).expect("Не удалось установить логгер"); // Установка логгера (обертки поверх RTT)
        log::set_max_level(log::LevelFilter::Info); // Установка фильтра логов по умолчанию на Info (Информационные сообщения)

        // Здесь пишем код настройки и обьявления
    }

    #[idle]
    fn main(_cx: main::Context) -> ! {
        // Здесь пишем код, он будет выполнятся когда другие задачи не работают

        loop {
            cortex_m::asm::wfi();
        }
    }

    extern "C" {
        fn SWI0_EGU0();
    }
};
{% else %}
#[cortex_m_rt::entry] // макрос предоставляющий "точку старта"
fn main() -> ! {
    rtt_init_print!(NoBlockSkip, 16384); // инициализация канала RTT

    let periph = Peripherals::take().expect("Не удалось получить доступ к периферии микроконтроллера"); // Получение всей периферии микроконтроллера
    let core_periph = cortex_m::Peripherals::take().expect("Не удалось получить доступ к периферии ядра"); // Получение всей периферии ядра Cortex-M

    log::set_logger(&Logger).expect("Не удалось установить логгер"); // Установка логгера (обертки поверх RTT)
    log::set_max_level(log::LevelFilter::Info); // Установка фильтра логов по умолчанию на Info (Информационные сообщения)

    let clocks = Clocks::new(periph.CLOCK); // передача переферии для настройки тактирования микроконтроллера
    let clocks = clocks.enable_ext_hfosc(); // использовать внешний источник (кварц) для тактирования микроконтроллера
    let clocks = clocks.set_lfclk_src_rc(); // использовать внутренний источник (кварц) для тактирования Rtc (часов)

    // Здесь пишем наш код

    crabik_board::exit(); // завершение программы (просто бесконечный цикл)
}
{% endif %}

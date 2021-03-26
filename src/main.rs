// Файл с основным кодом программы

// Отключение стандартной "точки старта" от компилятора
#![no_main]
// Отключение модуля std
#![no_std]

// Импорт модуля для платы crabik
use crabik_board::{
    // Модуль содержащий hal для nrf52810
    hal::{
        // Модуль с ранообразными обьявлениями для удобной работы
        prelude::*,
        // Модуль для настройки тактирования микроконтроллера
        Clocks,
    },
    pac::Peripherals,
    CrabikPins,
    Logger,
};

// Импорт модуля для работы с протоколом RTT
use rtt_target::*;

{% if rtic_enabled %}
// Настройка RTIC
#[rtic::app(device = crabik_board, peripherals = true,  monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {

    // Стартовая задача
    #[init]
    fn init(mut ctx: init::Context) {

        // передача переферии для настройки тактирования микроконтроллера
        let clocks = Clocks::new(ctx.device.CLOCK);
        
        // использовать внешний источник (кварц) для тактирования микроконтроллера
        let clocks = clocks.enable_ext_hfosc();
        
        // использовать внутренний источник (кварц) для тактирования Rtc (часов)
        let clocks = clocks.set_lfclk_src_rc();

        ctx.core.DCB.enable_trace();
        ctx.core.DWT.enable_cycle_counter();

        // Инициализация канала RTT
        rtt_init_print!(NoBlockSkip, 16384);

        // Установка логгера (обертки поверх RTT)
        log::set_logger(&Logger).expect("Не удалось установить логгер");

        // Установка фильтра логов по умолчанию на Info (Информационные сообщения)
        log::set_max_level(log::LevelFilter::Info);

        // Здесь пишем код настройки и обьявления
    }

    // Задача которая выполняется когда других задач для выполнения нет
    #[idle]
    fn main(_cx: main::Context) -> ! {
        // Здесь пишем код, он будет выполнятся когда другие задачи не работают

        loop {
            
            // Ожидание прерываний
            cortex_m::asm::wfi();
        }
    }

    extern "C" {
        fn SWI0_EGU0();
    }
};
{% else %}
// Макрос предоставляющий "точку старта"
#[cortex_m_rt::entry]
fn main() -> ! {

    // Инициализация канала RTT
    rtt_init_print!(NoBlockSkip, 16384);

    // Получение всей периферии микроконтроллера
    let periph = Peripherals::take().expect("Не удалось получить доступ к периферии микроконтроллера");

    // Получение всей периферии ядра Cortex-M
    let core_periph = cortex_m::Peripherals::take().expect("Не удалось получить доступ к периферии ядра");

    // Установка логгера (обертки поверх RTT)
    log::set_logger(&Logger).expect("Не удалось установить логгер");

    // Установка фильтра логов по умолчанию на Info (Информационные сообщения)
    log::set_max_level(log::LevelFilter::Info);

    // Передача переферии для настройки тактирования микроконтроллера
    let clocks = Clocks::new(periph.CLOCK);

    // Использовать внешний источник (кварц) для тактирования микроконтроллера
    let clocks = clocks.enable_ext_hfosc();

    // Использовать внутренний источник (кварц) для тактирования Rtc (часов)
    let clocks = clocks.set_lfclk_src_rc();

    // Здесь пишем наш код

    // Завершение программы (просто бесконечный цикл)
    crabik_board::exit();
}
{% endif %}

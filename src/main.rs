use crate::machine::Machine;
use batpu_assembler::assembler::Assembler;
use batpu_assembler::assembler_config::AssemblerConfig;
use iced::widget::image::FilterMethod;
use iced::widget::{column, row, slider, text, Column, Image, Scrollable};
use iced::Subscription;
use iced::{time, Length};
use std::time::{Duration, Instant};

mod machine;
mod stack;
mod components;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Message {
    Tick,
    ChangeIPS(u64)
}

struct App {
    pub instructions_per_second: u64,
    age: u64,

    previous_time: Instant,
    remaining_nanos: u128,

    machine: Machine
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                let current_time = Instant::now();
                let delta_time_nanos = current_time.duration_since(self.previous_time).as_nanos();
                self.previous_time = current_time;

                let tick_nanos = delta_time_nanos
                    .saturating_mul(self.instructions_per_second as u128)
                    .saturating_add(self.remaining_nanos);

                let ticks_to_run = (tick_nanos / 1_000_000_000) as usize;
                self.remaining_nanos = tick_nanos % 1_000_000_000;

                for _ in 0..ticks_to_run {
                    self.tick();
                }
            },
            Message::ChangeIPS(ips) => {
                self.instructions_per_second = ips;
            }
        }
    }

    fn tick(&mut self) {
        if !self.machine.running() {
            return;
        }

        self.machine.tick();
        self.age += 1;
    }

    fn view(&self) -> Column<Message> {
        let handle = self.machine.screen().handle();
        let image = Image::new(handle).width(Length::Fill).height(Length::Fill).filter_method(FilterMethod::Nearest);

        let mut registers_column: Column<Message> = Column::with_capacity(self.machine.registers().len());

        for (i, &register) in self.machine.registers().iter().enumerate() {
            registers_column = registers_column.push(text(format!("r{}: {}", i, register)));
        }

        let register_scrollable: Scrollable<Message> = Scrollable::new(registers_column)
            .width(Length::Fill)
            .height(Length::Fixed(210.0));

        let mut memory_column: Column<Message> = Column::with_capacity(self.machine.memory().len());

        for (i, &memory) in self.machine.memory().iter().enumerate() {
            memory_column = memory_column.push(text(format!("{}: {}", i, memory)));
        }

        let memory_scrollable: Scrollable<Message> = Scrollable::new(memory_column)
            .width(Length::Fill)
            .height(Length::Fixed(210.0));

        column![
            text(format!("Running: {}", self.machine.running())),
            text(format!("Age: {}", self.age)),
            text(format!("PC: {}", self.machine.program_counter())),
            text(format!("Zero: {}", self.machine.zero_flag())),
            text(format!("Carry: {}", self.machine.carry_flag())),
            text(format!("Characters: \"{}\"", self.machine.character_display().data())),
            text(format!("Number: {}", self.machine.number_display().value())),
            text(format!("Instructions per second: {}", self.instructions_per_second)),
            slider(0.0..=1_000.0, self.instructions_per_second as f32, |ips| Message::ChangeIPS(ips as u64)).width(Length::Fill),
            image,
            row![
                column![
                    text("Registers:"),
                    register_scrollable
                ],
                column![
                    text("Memory:"),
                    memory_scrollable
                ]
            ]
        ]
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(4)).map(|_| Message::Tick)
    }
}

impl Default for App {
    fn default() -> Self {
        let mut assembler = Assembler::new(AssemblerConfig::default());
        assembler.parse_file("program.asm").unwrap();

        let machine_code = assembler.assemble().unwrap();
        let instructions = batpu_assembly::binary_to_instructions(&machine_code).unwrap();

        let mut machine = Machine::new(instructions);
        machine.start();

        Self {
            instructions_per_second: 100,
            age: 0,

            previous_time: Instant::now(),
            remaining_nanos: 0,

            machine
        }
    }
}

fn main() -> iced::Result {
    iced::application("a meow", App::update, App::view)
        .subscription(App::subscription)
        .run()
}

use crate::machine::Machine;
use batpu_assembler::assembler::Assembler;
use batpu_assembler::assembler_config::AssemblerConfig;
use iced::widget::image::FilterMethod;
use iced::widget::pane_grid::Axis;
use iced::widget::{column, container, horizontal_space, pane_grid, row, scrollable, slider, text, text_input, Column, Image};
use iced::{time, Length};
use iced::{Element, Subscription};
use std::time::{Duration, Instant};

mod machine;
mod stack;
mod components;

#[derive(Debug, Copy, Clone)]
enum Message {
    None,
    Tick,
    SetIPS(u64),
    SetRegisterValue(usize, u8),
    SetMemoryValue(usize, u8),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pane {
    Display,
    Controller,
    Info,
    Assembly,
    Registers,
    Memory
}

struct App {
    pub instructions_per_second: u64,
    age: u64,

    previous_time: Instant,
    remaining_nanos: u128,

    machine: Machine,
    panes: pane_grid::State<Pane>
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::None => {},
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
            Message::SetIPS(ips) => {
                self.instructions_per_second = ips;
            },
            Message::SetRegisterValue(i, value) => {
                self.machine.registers_mut()[i] = value;
            },
            Message::SetMemoryValue(i, value) => {
                self.machine.memory_mut()[i] = value;
            },
            Message::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
            },
            Message::PaneDragged(_) => {},
            Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
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

    fn view(&self) -> Element<'_, Message> {
        pane_grid(&self.panes, |_, state, _| {
            pane_grid::Content::new(match state {
                Pane::Display => {
                    let screen_handle = self.machine.screen().handle();

                    // Border of image are cut off because of a cheap hack in iced/wgpu/src/image/mod.rs (Line 598)
                    let screen_image = Image::new(screen_handle)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .filter_method(FilterMethod::Nearest);

                    container(
                        column![
                            row![
                                text(format!("\"{}\"", self.machine.character_display().data())),
                                horizontal_space(),
                                text(self.machine.number_display().value())
                            ],
                            screen_image
                        ]
                    )
                },
                Pane::Controller => {
                    container(text("Controller"))
                },
                Pane::Info => {
                    container(
                        column![
                            row![
                                text("Instructions Per Second: "),
                                text_input("Input 0-10,000", self.instructions_per_second.to_string().as_str())
                                    .on_input(move |value| { // on_submit doesn't take functions
                                    let value = value.parse::<u64>();
                                    if let Ok(value) = value {
                                        if (0..=10_000).contains(&value) {
                                            return Message::SetIPS(value);
                                        }
                                    }

                                    Message::None
                                })
                                .width(Length::Fill)
                            ],
                            slider(0.0..=1_000.0, self.instructions_per_second as f32, |ips| Message::SetIPS(ips as u64)),
                            text(format!("Program Counter: {}", self.machine.program_counter())),
                            text(format!("Zero Flag: {}", self.machine.zero_flag())),
                            text(format!("Carry Flag: {}", self.machine.carry_flag()))
                        ]
                    )
                },
                Pane::Assembly => {
                    container(text("Assembly"))
                },
                Pane::Registers => {
                    let registers = self.machine.registers();
                    let mut registers_column: Column<Message> = Column::with_capacity(registers.len())
                        .width(Length::Fill);

                    for (i, &register) in registers.iter().enumerate() {
                        registers_column = registers_column.push(
                            row![
                                text(format!("r{}: ", i)),
                                text_input("Input 0-255", register.to_string().as_str())
                                    .on_input(move |value| { // on_submit doesn't take functions
                                    let value = value.parse::<u8>();
                                    if let Ok(value) = value {
                                        return Message::SetRegisterValue(i, value);
                                    }

                                    Message::None
                                })
                                .width(Length::Fill)
                            ]
                        );
                    }

                    container(
                        column![
                            text("Registers:"),
                            scrollable(registers_column)
                        ]
                    )
                },
                Pane::Memory => {
                    let memory = self.machine.memory();
                    let mut memory_column: Column<Message> = Column::with_capacity(memory.len())
                        .width(Length::Fill);

                    for (i, &value) in memory.iter().enumerate() {
                        memory_column = memory_column.push(
                            row![
                                text(format!("{}: ", i)),
                                text_input("Input 0-255", value.to_string().as_str())
                                    .on_input(move |value| { // on_submit doesn't take functions
                                    let value = value.parse::<u8>();
                                    if let Ok(value) = value {
                                        return Message::SetMemoryValue(i, value);
                                    }

                                    Message::None
                                })
                                .width(Length::Fill)
                            ]
                        );
                    }

                    container(
                        column![
                            text("Memory:"),
                            scrollable(memory_column)
                        ]
                    )
                }
            })
        })
            .on_drag(Message::PaneDragged)
            .on_resize(10, Message::PaneResized)
            .into()
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

        let (mut panes, screen_pane) = pane_grid::State::new(Pane::Display);
        let (info_pane, _) = panes.split(Axis::Vertical, screen_pane, Pane::Info).unwrap();
        let (registers_pane, _) = panes.split(Axis::Vertical, info_pane, Pane::Registers).unwrap();
        panes.split(Axis::Horizontal, screen_pane, Pane::Controller).unwrap();
        panes.split(Axis::Horizontal, info_pane, Pane::Assembly).unwrap();
        panes.split(Axis::Horizontal, registers_pane, Pane::Memory).unwrap();

        Self {
            instructions_per_second: 100,
            age: 0,

            previous_time: Instant::now(),
            remaining_nanos: 0,

            machine,

            panes
        }
    }
}

fn main() -> iced::Result {
    iced::application("BatPU Emulator v0.0.1", App::update, App::view)
        .subscription(App::subscription)
        .run()
}

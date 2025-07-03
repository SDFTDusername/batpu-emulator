use crate::machine::Machine;
use batpu_assembler::assembler::Assembler;
use batpu_assembler::assembler_config::AssemblerConfig;
use batpu_assembly::binary_to_instructions;
use eframe::Frame;
use egui::{menu, CentralPanel, Context, DragValue, Grid, Image, ScrollArea, SidePanel, Slider, TextureFilter, TextureHandle, TextureOptions, TextureWrapMode, TopBottomPanel, Ui, Widget};

mod machine;
mod stack;
mod components;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 420.0]),
        ..Default::default()
    };

    eframe::run_native(
        "BatPU Emulator",
        options,
        Box::new(|_| {
            Ok(Box::<View>::default())
        })
    )
}

struct View {
    instructions_per_second: u32,
    hi: u8,

    machine: Machine,
    screen_texture: Option<TextureHandle>
}

impl View {
    pub fn new() -> Self {
        let mut assembler = Assembler::new(AssemblerConfig::default());
        assembler.parse_file("program.asm").unwrap();

        let machine_code = assembler.assemble().unwrap();
        let instructions = binary_to_instructions(&machine_code).unwrap();

        let mut machine = Machine::new(instructions);
        machine.start();

        Self {
            instructions_per_second: 100,
            hi: 0,

            machine,
            screen_texture: None
        }
    }
    
    fn update_image(&mut self, ctx: &Context) {
        let screen = self.machine.screen_mut();
        if screen.image_updated() {
            let texture_options = TextureOptions {
                magnification: TextureFilter::Nearest,
                minification: TextureFilter::Nearest,
                wrap_mode: TextureWrapMode::ClampToEdge,
                mipmap_mode: None
            };

            let screen_texture = ctx.load_texture("screen", screen.image(), texture_options);
            self.screen_texture = Some(screen_texture);

            screen.disable_image_updated();
        }
    }
    
    fn menu(&mut self, ui: &mut Ui) {
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    
                }
            });
        });
    }
    
    fn screen(&mut self, ui: &mut Ui) {
        if let Some(screen_texture) = &self.screen_texture {
            ui.add(
                Image::new(screen_texture)
                    .fit_to_exact_size(ui.available_size())
            );
        }
    }
    
    fn controller(&mut self, ui: &mut Ui) {
        ui.heading("Controller");
    }
    
    fn info(&mut self, ui: &mut Ui) {
        ui.label("Instructions Per Second:");
        ui.add(Slider::new(&mut self.instructions_per_second, 0..=10_000));

        ui.separator();

        ui.label(format!("Program Counter: {}", self.machine.program_counter()));

        ui.separator();

        ui.label(format!("Zero Flag: {}", self.machine.zero_flag()));
        ui.label(format!("Carry Flag: {}", self.machine.carry_flag()));
    }
    
    fn assembly(&mut self, ui: &mut Ui) {
        ui.heading("Assembly");
    }
    
    fn register_view(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            Grid::new("register_view")
                .striped(true)
                .show(ui, |ui| {
                    for i in 0..8 {
                        for j in 0..2 {
                            ui.horizontal(|ui| {
                                ui.label(format!("r{}:", j + i * 2));
                                ui.add(DragValue::new(&mut self.hi));
                            });
                        }

                        ui.end_row();
                    }
                });
        });
    }
    
    fn memory_view(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            Grid::new("memory_view")
                .striped(true)
                .show(ui, |ui| {
                    for i in 0..128 {
                        for j in 0..2 {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}:", j + i * 2));
                                ui.add(DragValue::new(&mut self.hi));
                            });
                        }

                        ui.end_row();
                    }
                });
        });
    }
}

impl eframe::App for View {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.machine.tick();

        self.update_image(ctx);
        
        TopBottomPanel::top("menu_bar_panel").show(ctx, |ui| {
            self.menu(ui);
        });
        
        SidePanel::right("info_panel").show(ctx, |ui| {
            self.info(ui);
        });

        CentralPanel::default().show(ctx, |ui| {
            self.screen(ui);
        });

        ctx.request_repaint();
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}

use egui::{CentralPanel, Color32, ComboBox, SidePanel};
use egui_extras::{Size, StripBuilder};
use shine_core::atomic_refcell::AtomicRefCell;
use shine_frp_editor::node_graph::{ContextMenuItem, Graph, GraphEdit, Input, Node, Output, PortType};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq)]
enum SideTool {
    Memory,
    Settings,
}

struct MyApp {
    tool: SideTool,
    graph: Arc<AtomicRefCell<Graph>>,
    context_menu: Arc<Vec<ContextMenuItem>>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut graph = Graph::default();
        let type_u8 = graph.add_type(PortType::new("u8").with_color(Color32::KHAKI));
        let type_u16 = graph.add_type(PortType::new("u16"));
        let type_u32 = graph.add_type(PortType::new("u32"));

        let context_menu = vec![
            ContextMenuItem::sub_menu(
                "constants",
                vec![
                    ContextMenuItem::add_node("u8", move |node_id, pos| {
                        Node::primitive(node_id, "u8", pos, vec![], vec![Output::label("value", type_u8)])
                    }),
                    ContextMenuItem::add_node("u16", move |node_id, pos| {
                        Node::primitive(node_id, "u16", pos, vec![], vec![Output::label("value", type_u16)])
                    }),
                    ContextMenuItem::add_node("u32", move |node_id, pos| {
                        Node::primitive(node_id, "u32", pos, vec![], vec![Output::label("value", type_u32)])
                    }),
                ],
            ),
            ContextMenuItem::sub_menu(
                "logic",
                vec![ContextMenuItem::add_node("something", move |node_id, pos| {
                    Node::primitive(
                        node_id,
                        "zip",
                        pos,
                        vec![
                            Input::label("in1", type_u8),
                            Input::label("in2", type_u16),
                            Input::label("in3", type_u32),
                            Input::label("in4", type_u32),
                        ],
                        vec![Output::label("zipped", type_u8)],
                    )
                })],
            ),
        ];

        let graph = Arc::new(AtomicRefCell::new(graph));
        let context_menu = Arc::new(context_menu);

        Self {
            tool: SideTool::Memory,
            graph,
            context_menu,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::left("Settings").show(ctx, |ui| {
            ComboBox::new("Side panel", "")
                .selected_text(format!("{:?}", &mut self.tool))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.tool, SideTool::Memory, "Memory Tool");
                    ui.selectable_value(&mut self.tool, SideTool::Settings, "EGUI settings");
                });

            match self.tool {
                SideTool::Memory => ctx.memory_ui(ui),
                SideTool::Settings => ctx.settings_ui(ui),
            };
        });

        CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.5))
                .size(Size::exact(2.))
                .size(Size::relative(0.5))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ui.painter()
                            .rect_filled(ui.available_rect_before_wrap(), 0.0, Color32::DARK_BLUE);
                        GraphEdit::new("graph edit 1", self.graph.clone(), self.context_menu.clone()).show(ui);
                    });
                    strip.cell(|ui| {
                        ui.painter()
                            .rect_filled(ui.available_rect_before_wrap(), 0.0, Color32::WHITE);
                    });
                    strip.cell(|ui| {
                        ui.painter()
                            .rect_filled(ui.available_rect_before_wrap(), 0.0, Color32::DARK_RED);
                        GraphEdit::new("graph edit 2", self.graph.clone(), self.context_menu.clone()).show(ui);
                    });
                });
        });
    }
}

fn main() {
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 700.0)),
        ..Default::default()
    };
    eframe::run_native("graph editor", options, Box::new(|_cc| Box::new(MyApp::default())));
}

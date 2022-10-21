use egui::{CentralPanel, Color32, ComboBox, Id, Pos2, SidePanel};
use egui_extras::{Size, StripBuilder};
use shine_ui::node_graph::{
    arguments, Connection, ContextMenu, ContextMenuId, Graph, GraphEdit, GraphOperation, Input, InputId,
    Node, NodeId, Output, OutputId, PortType,
};
use slotmap::SecondaryMap;

#[derive(Clone, Debug, PartialEq, Eq)]
enum SideTool {
    Memory,
    Settings,
}

struct MyApp {
    tool: SideTool,
    graph: Graph,
    context_menu: ContextMenu,
    context_menu_action: SecondaryMap<ContextMenuId, Box<dyn Fn(NodeId, Pos2) -> Node>>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut graph = Graph::default();
        let type_u8 = graph.add_type(PortType::new("u8").with_color(Color32::KHAKI));
        let type_u16 = graph.add_type(PortType::new("u16"));
        let type_u32 = graph.add_type(PortType::new("u32"));

        let (context_menu, context_menu_action) = {
            let mut context_menu = ContextMenu::default();
            let mut context_menu_action = SecondaryMap::<_, Box<dyn Fn(NodeId, Pos2) -> Node>>::default();
            let mut builder = context_menu.builder();
            {
                let mut constants = builder.add_group("constants");
                constants
                    .add_item_with("u8", |menu_id| {
                        context_menu_action.insert(
                            menu_id,
                            Box::new(move |node_id, pos| {
                                Node::new(
                                    node_id,
                                    "u8",
                                    pos,
                                    vec![],
                                    vec![Output::new("value", type_u8)],
                                    vec![Box::new(arguments::HelloWorld)],
                                )
                            }),
                        );
                    })
                    .add_item_with("u8", |menu_id| {
                        context_menu_action.insert(
                            menu_id,
                            Box::new(move |node_id, pos| {
                                Node::new(
                                    node_id,
                                    "u16",
                                    pos,
                                    vec![],
                                    vec![Output::new("value", type_u16)],
                                    vec![],
                                )
                            }),
                        );
                    })
                    .add_item_with("u8", |menu_id| {
                        context_menu_action.insert(
                            menu_id,
                            Box::new(move |node_id, pos| {
                                Node::new(
                                    node_id,
                                    "u32",
                                    pos,
                                    vec![],
                                    vec![Output::new("value", type_u32)],
                                    vec![],
                                )
                            }),
                        );
                    });
            }

            {
                let mut logic = builder.add_group("logic");

                logic.add_item_with("zip", |menu_id| {
                    context_menu_action.insert(
                        menu_id,
                        Box::new(move |node_id, pos| {
                            Node::new(
                                node_id,
                                "zip",
                                pos,
                                vec![
                                    Input::new("in1", type_u8),
                                    Input::new("in2", type_u16),
                                    Input::new("in3", type_u32),
                                    Input::new("in4", type_u32),
                                ],
                                vec![Output::new("zipped", type_u8)],
                                vec![],
                            )
                        }),
                    );
                });
            }

            (context_menu, context_menu_action)
        };

        Self {
            tool: SideTool::Memory,
            graph,
            context_menu,
            context_menu_action,
        }
    }
}

impl MyApp {
    fn connection_validator(&self, input: InputId, output: OutputId) -> bool {
        input.type_id() == output.type_id()
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

        let mut operations = Vec::new();

        CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.5))
                .size(Size::exact(2.))
                .size(Size::relative(0.5))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ui.painter()
                            .rect_filled(ui.available_rect_before_wrap(), 0.0, Color32::DARK_BLUE);
                        operations.append(
                            &mut GraphEdit::new(
                                Id::new("graph edit 1"),
                                &self.graph,
                                &self.context_menu,
                                &|input, output| self.connection_validator(input, output),
                            )
                            .show(ui),
                        );
                    });
                    strip.cell(|ui| {
                        ui.painter()
                            .rect_filled(ui.available_rect_before_wrap(), 0.0, Color32::WHITE);
                    });
                    strip.cell(|ui| {
                        ui.painter()
                            .rect_filled(ui.available_rect_before_wrap(), 0.0, Color32::DARK_RED);
                        operations.append(
                            &mut GraphEdit::new(
                                Id::new("graph edit 2"),
                                &self.graph,
                                &self.context_menu,
                                &|input, output| self.connection_validator(input, output),
                            )
                            .show(ui),
                        );
                    });
                });
        });

        for operation in operations {
            match operation {
                GraphOperation::ContextMenu(pos, menu_id) => {
                    if let Some(builder) = self.context_menu_action.get(menu_id) {
                        let _ = self.graph.add_node(|node_id| (builder)(node_id, pos));
                    }
                }
                GraphOperation::Connect(input_id, output_id) => {
                    let _ = self
                        .graph
                        .add_connection(|connection_id| Connection::new(connection_id, input_id, output_id));
                }
                GraphOperation::SetNodeLocation(node_id, pos) => {
                    if let Some(node) = self.graph.nodes.get_mut(node_id) {
                        node.location = pos;
                    }
                }
            }
        }
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

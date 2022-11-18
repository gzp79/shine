use egui::{CentralPanel, Color32, ComboBox, Id, Pos2, SidePanel, Slider, Ui};
use egui_extras::{Size, StripBuilder};
use shine_ui::node_graph::{
    Connection, ConnectionData, ContextMenu, ContextMenuData, Graph, GraphEdit, Input, InputId, InputPortData, Node,
    NodeData, Output, OutputId, OutputPortData, PortStyle, PortStyles, Validator,
};
use std::any::TypeId;

#[derive(Clone, Debug, PartialEq, Eq)]
enum SideTool {
    Memory,
    Settings,
}

#[derive(Clone)]
enum MyContextMenuData {
    AddMinimalNode,
    AddU8Node,
    AddU16Node,
    AddU32Node,
    AddComplexNode,

    ClearGraph,
}

impl ContextMenuData for MyContextMenuData {
    fn on_select(&self, graph: &mut Graph, location: Pos2) {
        match self {
            MyContextMenuData::AddMinimalNode => {
                graph.add_node(Node::new("minimal", location, vec![], vec![]));
            }
            MyContextMenuData::AddU8Node => {
                graph.add_node(Node::new("u8", location, vec![], vec![Output::new::<u8>("value")]));
            }
            MyContextMenuData::AddU16Node => {
                graph.add_node(
                    Node::new("u16", location, vec![], vec![Output::new::<u16>("value")]).with_data(SampleNodeData {
                        value: "edit my node data".to_string(),
                    }),
                );
            }
            MyContextMenuData::AddU32Node => {
                graph.add_node(Node::new(
                    "u32",
                    location,
                    vec![],
                    vec![Output::new::<u32>("value").with(SampleOutput {
                        value: "update me".to_string(),
                    })],
                ));
            }
            MyContextMenuData::AddComplexNode => {
                graph.add_node(Node::new(
                    "complex",
                    location,
                    vec![
                        Input::new::<u8>("in1").with(SampleInput { value: 10. }),
                        Input::new::<u16>("in2"),
                        Input::new::<u32>("in3"),
                    ],
                    vec![Output::new::<u8>("calculated")],
                ));
            }
            MyContextMenuData::ClearGraph => {
                graph.clear();
            }
        }
    }
}

pub struct SampleInput {
    value: f32,
}

impl InputPortData for SampleInput {
    fn show(&mut self, ui: &mut Ui, _port_id: usize, _style: &PortStyle) {
        ui.add(Slider::new(&mut self.value, 0.0..=100.0).text("percent"));
    }
}

pub struct SampleOutput {
    value: String,
}

impl OutputPortData for SampleOutput {
    fn show(&mut self, ui: &mut Ui, _port_id: usize, _style: &PortStyle) {
        ui.text_edit_singleline(&mut self.value);
    }
}

pub struct SampleNodeData {
    value: String,
}

impl NodeData for SampleNodeData {
    fn show(&mut self, ui: &mut Ui, _inputs: &mut Vec<Input>, _outputs: &mut Vec<Output>) {
        ui.text_edit_singleline(&mut self.value);
    }
}

pub struct SampleConnectionData {
    value: String,
}

impl ConnectionData for SampleConnectionData {
    fn show(&mut self, ui: &mut Ui, _style: &PortStyle) {
        ui.label(&self.value);
    }
}

pub struct MyGraphValidator;

impl Validator for MyGraphValidator {
    fn try_create_connection(&self, _graph: &Graph, input_id: InputId, output_id: OutputId) -> Option<Connection> {
        if input_id.port_type_id() == output_id.port_type_id() {
            if input_id.port_type_id() == TypeId::of::<u8>() {
                Some(Connection::new(
                    input_id,
                    output_id,
                    SampleConnectionData {
                        value: "note".to_string(),
                    },
                ))
            } else {
                Some(Connection::new(input_id, output_id, ()))
            }
        } else {
            None
        }
    }
}

struct MyApp {
    tool: SideTool,
    graph: Graph,
    context_menu: ContextMenu,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut style = PortStyles::default();
        style.set::<u8>(PortStyle::new("u8").with_color(Color32::KHAKI));
        style.set::<u16>(PortStyle::new("u16"));
        style.set::<u32>(PortStyle::new("u32"));

        let mut graph = Graph::default();
        graph.set_validator(MyGraphValidator);
        graph.set_port_styles(style);

        let context_menu = {
            let mut context_menu = ContextMenu::default();
            let mut builder = context_menu.builder();

            builder
                .add_group("constants")
                .add_item("u8", MyContextMenuData::AddU8Node)
                .add_item("u16", MyContextMenuData::AddU16Node)
                .add_item("u32", MyContextMenuData::AddU32Node);
            builder
                .add_group("logic")
                .add_item("minimal", MyContextMenuData::AddMinimalNode)
                .add_item("complex", MyContextMenuData::AddComplexNode);
            builder.add_item("clear", MyContextMenuData::ClearGraph);

            context_menu
        };

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
                        GraphEdit::new(Id::new("graph edit 1"), &mut self.graph, &self.context_menu).show(ui);
                    });
                    strip.cell(|ui| {
                        ui.painter()
                            .rect_filled(ui.available_rect_before_wrap(), 0.0, Color32::WHITE);
                    });
                    strip.cell(|ui| {
                        ui.painter()
                            .rect_filled(ui.available_rect_before_wrap(), 0.0, Color32::DARK_RED);
                        GraphEdit::new(Id::new("graph edit 2"), &mut self.graph, &self.context_menu).show(ui);
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

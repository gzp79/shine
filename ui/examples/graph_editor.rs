use egui::{CentralPanel, Color32, ComboBox, Id, Pos2, SidePanel};
use egui_extras::{Size, StripBuilder};
use shine_ui::node_graph::{
    ConnectionData, ContextMenu, ContextMenuData, Graph, GraphData, GraphEdit, Input, InputId, Node, NodeData, Output,
    OutputId, PortType, PortTypeId,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum SideTool {
    Memory,
    Settings,
}
#[derive(Clone)]
struct MyNodeData;
impl NodeData for MyNodeData {}

#[derive(Clone)]
struct MyConnectionData;
impl ConnectionData for MyConnectionData {}

#[derive(Default, Clone)]
struct MyGraphData {
    type_u8: PortTypeId,
    type_u16: PortTypeId,
    type_u32: PortTypeId,
}

impl GraphData for MyGraphData {
    type NodeData = MyNodeData;
    type ConnectionData = MyConnectionData;

    fn create_connection_data(&mut self, input: InputId, output: OutputId) -> Option<Self::ConnectionData> {
        if input.type_id() == output.type_id() {
            Some(MyConnectionData)
        } else {
            None
        }
    }
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
    type GraphData = MyGraphData;

    fn on_select(&self, graph: &mut Graph<Self::GraphData>, location: Pos2) {
        match self {
            MyContextMenuData::AddMinimalNode => {
                graph.add_node(|node_id| {
                    Node::new(
                        node_id,
                        "minimal",
                        location,
                        MyNodeData,
                        vec![],
                        vec![],
                    )
                });
            }
            MyContextMenuData::AddU8Node => {
                let type_u8 = graph.data.type_u8;
                graph.add_node(|node_id| {
                    Node::new(
                        node_id,
                        "u8",
                        location,
                        MyNodeData,
                        vec![],
                        vec![Output::new("value", type_u8)],
                    )
                });
            }
            MyContextMenuData::AddU16Node => {
                let type_u16 = graph.data.type_u16;
                graph.add_node(|node_id| {
                    Node::new(
                        node_id,
                        "u16",
                        location,
                        MyNodeData,
                        vec![],
                        vec![Output::new("value", type_u16)],
                    )
                });
            }
            MyContextMenuData::AddU32Node => {
                let type_u32 = graph.data.type_u32;
                graph.add_node(|node_id| {
                    Node::new(
                        node_id,
                        "u32",
                        location,
                        MyNodeData,
                        vec![],
                        vec![Output::new("value", type_u32)],
                    )
                });
            }
            MyContextMenuData::AddComplexNode => {
                let type_u8 = graph.data.type_u8;
                let type_u16 = graph.data.type_u16;
                let type_u32 = graph.data.type_u32;
                graph.add_node(|node_id| {
                    Node::new(
                        node_id,
                        "complex",
                        location,
                        MyNodeData,
                        vec![
                            Input::new("in1", type_u8),
                            Input::new("in2", type_u16),
                            Input::new("in3", type_u32),
                            Input::new("in4", type_u32),
                        ],
                        vec![Output::new("zipped", type_u8)],
                    )
                });
            }
            MyContextMenuData::ClearGraph => {
                graph.nodes.clear();
                graph.connections.clear();
            }
        }
    }
}

struct MyApp {
    tool: SideTool,
    graph: Graph<MyGraphData>,
    context_menu: ContextMenu<MyContextMenuData>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut graph = Graph::<MyGraphData>::default();
        graph.data.type_u8 = graph.add_type(PortType::new("u8").with_color(Color32::KHAKI));
        graph.data.type_u16 = graph.add_type(PortType::new("u16"));
        graph.data.type_u32 = graph.add_type(PortType::new("u32"));

        let context_menu = {
            let mut context_menu = ContextMenu::default();
            let mut builder = context_menu.builder();

            builder
                .add_group("constants")
                .add_item("u8", MyContextMenuData::AddU8Node)
                .add_item("u16", MyContextMenuData::AddU16Node)
                .add_item("u32", MyContextMenuData::AddU32Node);
            builder.add_group("logic")
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

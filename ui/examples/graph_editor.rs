use egui::{CentralPanel, Color32, ComboBox, Id, Pos2, SidePanel, Slider, Ui};
use egui_extras::{Size, StripBuilder};
use shine_ui::node_graph::{
    ConnectionData, ContextMenu, ContextMenuData, Graph, GraphData, GraphEdit, Input, InputData, InputId, Node,
    NodeData, Output, OutputData, OutputId, PortStyle,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum SideTool {
    Memory,
    Settings,
}

#[derive(Clone)]
struct MyConnectionData;
impl ConnectionData for MyConnectionData {
    fn try_connect<G>(_graph: &mut Graph<G>, input_id: InputId, output_id: OutputId) -> Option<Self>
    where
        Self: Sized,
        G: GraphData<ConnectionData = Self>,
    {
        if input_id.port_type_id() == output_id.port_type_id() {
            Some(MyConnectionData)
        } else {
            None
        }
    }
}

#[derive(Default, Clone)]
struct MyGraphData;

impl GraphData for MyGraphData {
    type ConnectionData = MyConnectionData;

    fn clear(&mut self) {}
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
                graph.add_node(|node_id| Node::new(node_id, "minimal", location, vec![], vec![], ()));
            }
            MyContextMenuData::AddU8Node => {
                graph.add_node(|node_id| {
                    Node::new(
                        node_id,
                        "u8",
                        location,
                        vec![],
                        vec![Output::<u8, _>::new("value", ()).into()],
                        (),
                    )
                });
            }
            MyContextMenuData::AddU16Node => {
                graph.add_node(|node_id| {
                    Node::new(
                        node_id,
                        "u16",
                        location,
                        vec![],
                        vec![Output::<u16, _>::new("value", ()).into()],
                        SampleNodeData {
                            value: "edit my node data".to_string(),
                        },
                    )
                });
            }
            MyContextMenuData::AddU32Node => {
                graph.add_node(|node_id| {
                    Node::new(
                        node_id,
                        "u32",
                        location,
                        vec![],
                        vec![Output::<u32, _>::new(
                            "value",
                            SampleOutput {
                                value: "update me".to_string(),
                            },
                        )
                        .into()],
                        (),
                    )
                });
            }
            MyContextMenuData::AddComplexNode => {
                graph.add_node(|node_id| {
                    Node::new(
                        node_id,
                        "complex",
                        location,
                        vec![
                            Input::<u8, _>::new("in1", SampleInput { value: 10. }).into(),
                            Input::<u16, _>::new("in2", ()).into(),
                            Input::<u32, _>::new("in3", ()).into(),
                        ],
                        vec![Output::<u8, _>::new("calculated", ()).into()],
                        (),
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

pub struct SampleInput {
    value: f32,
}

impl InputData for SampleInput {
    fn show(&mut self, ui: &mut Ui, _style: &PortStyle) {
        ui.add(Slider::new(&mut self.value, 0.0..=100.0).text("percent"));
    }
}

pub struct SampleOutput {
    value: String,
}

impl OutputData for SampleOutput {
    fn show(&mut self, ui: &mut Ui, _style: &PortStyle) {
        ui.text_edit_singleline(&mut self.value);
    }
}

pub struct SampleNodeData {
    value: String,
}

impl NodeData for SampleNodeData {
    fn show(&mut self, ui: &mut Ui) {
        ui.text_edit_singleline(&mut self.value);
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
        graph.set_type_style::<u8>(PortStyle::new("u8").with_color(Color32::KHAKI));
        graph.set_type_style::<u16>(PortStyle::new("u16"));
        graph.set_type_style::<u32>(PortStyle::new("u32"));

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

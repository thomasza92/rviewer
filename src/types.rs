use egui::emath::TSTransform;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Copy, Debug)]
pub struct PanZoom {
    pub transform: TSTransform,
    pub drag_value: f32,
}

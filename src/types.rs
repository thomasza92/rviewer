use egui::emath::TSTransform;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Copy, Debug)]
pub struct PanZoom {
    pub(crate) transform: TSTransform,
    pub(crate) drag_value: f32,
}
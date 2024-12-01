use egui::emath::TSTransform;
use egui::{Pos2, TextureHandle, Vec2};
use rfd::FileDialog;
use rviewer::loader;
use rviewer::loader::{display_image, extract_exif_metadata};
use rviewer::types::PanZoom;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Rviewer {
    open_image: PathBuf,
    #[serde(skip)]
    img_texture: Option<TextureHandle>,
    save_image: PathBuf,
    show_menu: bool,
    fullscreen: bool,
    minimized: bool,
    pan_zoom: Option<PanZoom>,
    img_size: Vec2,
    img_pos: Pos2,
    rotation: f32,
    metadata: String,
}

impl Default for Rviewer {
    fn default() -> Self {
        Self {
            open_image: "".parse().unwrap(),
            img_texture: None,
            save_image: "".parse().unwrap(),
            show_menu: false,
            fullscreen: false,
            minimized: false,
            pan_zoom: None,
            img_size: Vec2::new(690.00, 920.00),
            img_pos: Pos2::new(0.00, 0.00),
            rotation: 0.0,
            metadata: String::new(),
        }
    }
}

impl Rviewer {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for Rviewer {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let nested_menus = |ui: &mut egui::Ui, rviewer: &mut Rviewer| {
            ui.set_max_width(80.0);

            if ui.button("Open…").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    rviewer.img_texture = None;
                    rviewer.open_image = path;

                    if let Some((texture, size)) =
                        loader::load_displayimage(ctx, rviewer.open_image.to_str().unwrap())
                    {
                        rviewer.img_texture = Some(texture);

                        // get aspect ratio of image
                        let aspect_ratio = size.x / size.y;

                        // get current screen or window size
                        let screen_rect = ctx.screen_rect();
                        let screen_size = screen_rect.size();

                        // determine the maximum size the image can have while fitting in the screen
                        if screen_size.x / screen_size.y > aspect_ratio {
                            rviewer.img_size =
                                Vec2::new(screen_size.y * aspect_ratio, screen_size.y);
                        } else {
                            // fit by width
                            rviewer.img_size =
                                Vec2::new(screen_size.x, screen_size.x / aspect_ratio);
                        }

                        // center image
                        rviewer.img_pos = Pos2::new(
                            (screen_size.x - rviewer.img_size.x) / 2.0,
                            (screen_size.y - rviewer.img_size.y) / 2.0,
                        );

                        log::info!(
                            "Image at: {} was opened with dimensions: {:.1}x{:.1}, centered at: {:.1}, {:.1}.",
                            rviewer.open_image.display(),
                            rviewer.img_size.x,
                            rviewer.img_size.y,
                            rviewer.img_pos.x,
                            rviewer.img_pos.y
                        );
                    }
                }
                ui.close_menu();
            }
            if ui.button("Save").clicked() {
                if let Some(path) = FileDialog::new().save_file() {
                    rviewer.save_image = path;
                    log::info!("Image at: {} was saved.", rviewer.save_image.display());
                }
                ui.close_menu();
            }
        };
        if self.pan_zoom.is_none() {
            self.pan_zoom = Some(PanZoom {
                transform: TSTransform::default(),
                drag_value: 0.0,
            });
        }
        let pan_zoom = self.pan_zoom.as_mut().unwrap();
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .show(ctx, |ui| {
                if let Some(metadata) = extract_exif_metadata(self.open_image.to_str().unwrap()) {
                    self.metadata = metadata;
                    log::info!("EXIF Metadata:\n{}", self.metadata);
                } else {
                    log::info!("No EXIF metadata found or failed to read the file.");
                }
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(&self.metadata);
                });
            });
        egui::CentralPanel::default().show(ctx, |_ui| {
            let (id, rect) = _ui.allocate_space(_ui.available_size());
            let response = _ui.interact(rect, id, egui::Sense::click_and_drag());
            if response.dragged() {
                pan_zoom.transform.translation += response.drag_delta();
            }
            if response.hovered() {
                let zoom_delta = _ui.ctx().input(|i| i.zoom_delta());
                if zoom_delta != 1.0 {
                    if let Some(pointer) = _ui.ctx().input(|i| i.pointer.hover_pos()) {
                        let pointer_in_layer = pan_zoom.transform.inverse() * pointer;
                        pan_zoom.transform = pan_zoom.transform
                            * TSTransform::from_translation(pointer_in_layer.to_vec2())
                            * TSTransform::from_scaling(zoom_delta)
                            * TSTransform::from_translation(-pointer_in_layer.to_vec2());
                    }
                }
            }
            if response.double_clicked() {
                pan_zoom.transform = TSTransform::default();
            }
            let transform = TSTransform::from_translation(_ui.min_rect().left_top().to_vec2())
                * pan_zoom.transform;

            if let Some(image_texture) = &self.img_texture {
                display_image(
                    _ui,
                    image_texture,
                    self.img_pos,
                    self.img_size,
                    self.rotation,
                    &transform,
                );
            }
        });
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("R").clicked() {
                        if self.img_texture.is_some() {
                            self.rotation += std::f32::consts::FRAC_PI_2; // Increment by 90 degrees
                            log::info!("Image rotated 90 degrees clockwise.");
                        } else {
                            log::info!("Rotation ignored: No image loaded.");
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.menu_button("[ rviewer ]", |ui| nested_menus(ui, self));
                });
                ui.separator();
                ui.add_space(8.0);
                egui::widgets::global_theme_preference_switch(ui);
            });
        });
    }
}

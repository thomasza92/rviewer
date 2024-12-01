use egui::emath::Rot2;
use egui::{Mesh, Pos2, TextureHandle, Vec2};
use exif::Reader;
use std::fs::File;
use std::io::BufReader;

pub fn extract_exif_metadata(file_path: &str) -> Option<String> {
    // Open the image file
    let file = File::open(file_path).ok()?;
    let mut bufreader = BufReader::new(file);

    // Create an EXIF reader and attempt to read EXIF data
    let exifreader = Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).ok()?; // Unwrap the Result safely

    // Collect relevant metadata into a formatted string
    let mut metadata = String::new();
    for field in exif.fields() {
        metadata.push_str(&format!(
            "{}: {}\n",
            field.tag,
            field.display_value().with_unit(&exif)
        ));
    }

    Some(metadata)
}

pub fn load_displayimage(ctx: &egui::Context, path: &str) -> Option<(TextureHandle, Vec2)> {
    let image = image::open(path).ok()?.into_rgba8();

    let size = Vec2::new(image.width() as f32, image.height() as f32);

    let pixels = image.clone().into_raw();

    let texture = ctx.load_texture(
        "image",
        egui::ColorImage::from_rgba_unmultiplied(
            [image.width() as _, image.height() as _],
            pixels.as_slice(),
        ),
        Default::default(),
    );

    Some((texture, size))
}

pub fn display_image(
    ui: &mut egui::Ui,
    texture: &TextureHandle,
    img_pos: Pos2,
    img_size: Vec2,
    rotation: f32,
    transform: &egui::emath::TSTransform,
) {
    let center = img_pos + img_size / 2.0;

    let rotation_matrix = Rot2::from_angle(rotation);

    // Define the corners of the image relative to its center
    let half_size = img_size / 2.0;
    let corners = [
        Pos2::new(-half_size.x, -half_size.y), // Top-left
        Pos2::new(half_size.x, -half_size.y),  // Top-right
        Pos2::new(half_size.x, half_size.y),   // Bottom-right
        Pos2::new(-half_size.x, half_size.y),  // Bottom-left
    ]
    .map(|corner| center + rotation_matrix * corner.to_vec2());

    // Apply the transform to the rotated corners
    let transformed_corners: Vec<Pos2> =
        corners.iter().map(|&corner| *transform * corner).collect();

    // Create UV coordinates for texture mapping (Pos2, as required by egui::epaint::Vertex)
    let uvs = [
        Pos2::new(0.0, 0.0), // Top-left
        Pos2::new(1.0, 0.0), // Top-right
        Pos2::new(1.0, 1.0), // Bottom-right
        Pos2::new(0.0, 1.0), // Bottom-left
    ];

    // Construct the mesh for the image
    let mut mesh = Mesh::with_texture(texture.id());
    for (corner, uv) in transformed_corners.iter().zip(uvs.iter()) {
        mesh.vertices.push(egui::epaint::Vertex {
            pos: *corner, // Position of the vertex
            uv: *uv,      // Texture coordinates (Pos2)
            color: egui::Color32::WHITE,
        });
    }

    // Define the indices for the two triangles that make up the quad
    mesh.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);

    // Paint the mesh onto the UI
    ui.painter().add(mesh);
}

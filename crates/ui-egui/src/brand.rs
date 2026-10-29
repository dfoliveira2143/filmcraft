//! Text-only identity for the independent Creative Workshop edition.

/// Draw the collection name, returning its rectangle for adjacent layout.
pub fn paint_wordmark(ui: &egui::Ui, left_center: egui::Pos2, height: f32, dark: bool) -> Option<egui::Rect> {
    let color = if dark { egui::Color32::WHITE } else { egui::Color32::from_gray(32) };
    Some(ui.painter().text(left_center, egui::Align2::LEFT_CENTER, "Creative Workshop", egui::FontId::proportional(height), color))
}

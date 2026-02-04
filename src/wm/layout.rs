use super::manager::WorkspaceManager;
use serde::{Deserialize, Serialize};
use x11rb::connection::Connection;
use crate::sender_layout_set;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Layout {
    // Corner layouts
    CornerNE = 0,
    CornerNW,
    CornerSE,
    CornerSW,
    // Tile variants
    TileRight,
    TileBottom,
    TileLeft, // alias explícito
    TileTop,
    // Grid / fair
    FairH,
    FairV,
    // Special
    Fullscreen,
    Dwindle,
    Magnifier,
    Max,
    Spiral,

}
impl Layout {
    pub fn id(self) -> u8 {
        self as u8
    }
}


impl WorkspaceManager {
    // ============================================================
    // 🔹 APLICAR LAYOUT DEL WORKSPACE ACTUAL
    // ============================================================
    pub fn apply_layout(&self) {
        let ws = self.current;
        let windows = &self.workspaces[ws];
        if windows.is_empty() {
            return;
        }
        
        
        // ✅ tamaño real de la pantalla
        let screen = &self.conn.setup().roots[0];

        let panel_zone = &self.panel.current_width(); // + self.vh(2,height_in_pixels);;
        let width = screen.width_in_pixels - panel_zone;
        let gap = self.vh(2, screen.height_in_pixels);
        let height = screen.height_in_pixels - gap;

        match self.layouts[ws] {
            Layout::TileLeft => self.tile_left(windows, width, height),

            Layout::TileRight => self.tile_right(windows, width, height),

            Layout::TileTop => self.tile_top(windows, width, height),

            Layout::TileBottom => self.tile_bottom(windows, width, height),

            Layout::CornerNW => self.corner_nw(windows, width, height),

            Layout::CornerNE => self.corner_ne(windows, width, height),

            Layout::CornerSW => self.corner_sw(windows, width, height),

            Layout::CornerSE => self.corner_se(windows, width, height),

            Layout::FairH => self.fair_h(windows, width, height),

            Layout::FairV => self.fair_v(windows, width, height),

            Layout::Max => self.max(windows, width, height),

            Layout::Fullscreen => self.fullscreen(windows, width, height),

            Layout::Magnifier => self.magnifier(windows, width, height),

            Layout::Spiral => self.spiral(windows, width, height),

            Layout::Dwindle => self.dwindle(windows, width, height),
        }
    }

   
    pub fn toggle_layout_persist(&mut self) {
        let ws = self.current;

        self.layouts[ws] = match self.layouts[ws] {
            Layout::CornerNE => Layout::CornerNW,
            Layout::CornerNW => Layout::CornerSE,
            Layout::CornerSE => Layout::CornerSW,
            Layout::CornerSW => Layout::TileRight,
            Layout::TileRight => Layout::TileTop,
            Layout::TileTop => Layout::TileLeft,
            Layout::TileLeft => Layout::TileBottom,
            Layout::TileBottom => Layout::FairH,
            Layout::FairH => Layout::FairV,
            Layout::FairV => Layout::Fullscreen,
            Layout::Fullscreen => Layout::Dwindle,
            Layout::Dwindle => Layout::Magnifier,
            Layout::Magnifier => Layout::Max,
            Layout::Max => Layout::Spiral,
            Layout::Spiral => Layout::CornerNE,
        };
        self.apply_layout();
        let _ = self.save_layouts();
        sender_layout_set(self);
    }

    #[inline]
    pub fn vh(&self,count : u16, screen_height: u16) -> u16 {
        count *screen_height / 100
    }
}

use crate::workspace::Workspace;
use smithay::utils::IsAlive;
use crate::shell::ssd::HEADER_BAR_HEIGHT;
use crate::state::SpaceExt;
use crate::dock::PunkDock;
impl Workspace{

    pub fn arrange_tile_left(&mut self, dock: &mut PunkDock) {
        if self.windows.len() <= 1 {return self.arrange_max(dock);}
        let margin = self.space.vh();
        let output = self.space.outputs().next().cloned();
        let Some(output_geo) =output.and_then(|o| self.space.output_geometry(&o))else {return;};
        self.arrange_dock(dock);

        // -------------------------
        // Dock lateral izquierdo
        // -------------------------
        let dock_zone = dock.size() + margin;

        // -------------------------
        // Área útil
        // -------------------------
        let total_w = output_geo.size.w - dock_zone - (2 * margin);
        let total_h = output_geo.size.h - (2 * margin);

        // -------------------------
        // Columnas
        // -------------------------
        let left_w = (total_w / 2) - (margin / 2);
        let right_w = total_w - left_w - margin;

        // -------------------------
        // Master window
        // -------------------------
        let master_x = output_geo.loc.x + dock_zone + margin;
        let master_y = output_geo.loc.y + margin;
        let windows: Vec<_> = self.windows.iter().filter(|w| w.alive()).cloned().collect();
        let master = windows[0].clone();
        let master_bar = if master.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
        let master_h = total_h - master_bar;
        self.map_window(master.clone(), master_x, master_y, left_w, master_h);
        
        // -------------------------
        // Stack derecho
        // -------------------------
        let stack_x = master_x + left_w + margin;
        let stack_count = (self.windows.len() - 1) as i32;
        if stack_count <= 0 { return;}

        // gaps internos verticales
        let available_h = total_h - (margin * (stack_count - 1));
        let stack_h = available_h / stack_count;
        let mut y = output_geo.loc.y + margin;

        for window in windows.iter().skip(1)  {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = stack_h - bar;
            self.map_window(window.clone(), stack_x, y, right_w, final_h);
            y += stack_h + margin;
        }
    }

    pub fn arrange_tile_right(&mut self, dock: &mut PunkDock) {
        if self.windows.len() <= 1 {return self.arrange_max(dock);}
        let margin = self.space.vh();
        let output = self.space.outputs().next().cloned();
        let Some(output_geo) =output.and_then(|o| self.space.output_geometry(&o))else {return;};
        self.arrange_dock(dock);

        // -------------------------
        // Dock lateral izquierdo
        // -------------------------
        let dock_zone = dock.size() + margin;

        // -------------------------
        // Área útil
        // -------------------------
        let total_w = output_geo.size.w - dock_zone - (2 * margin);
        let total_h = output_geo.size.h - (2 * margin);

        // -------------------------
        // Columnas
        // -------------------------
        let left_w = (total_w / 2) - (margin / 2);
        let right_w = total_w - left_w - margin;

        // -------------------------
        // Master window
        // -------------------------
        let stack_x = output_geo.loc.x + dock_zone + margin;
        let master_x = stack_x + left_w + margin;
        let master_y = output_geo.loc.y + margin;
        let windows: Vec<_> = self.windows.iter().filter(|w| w.alive()).cloned().collect();
        let master = windows[0].clone();
        let master_bar = if master.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
        let master_h = total_h - master_bar;
        self.map_window(master.clone(), master_x, master_y, left_w, master_h);
        
        // -------------------------
        // Stack derecho
        // -------------------------
        let stack_count = (self.windows.len() - 1) as i32;
        if stack_count <= 0 { return;}
        // gaps internos verticales
        let available_h = total_h - (margin * (stack_count - 1));
        let stack_h = available_h / stack_count;
        let mut y = output_geo.loc.y + margin;
        for window in windows.iter().skip(1) {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = stack_h - bar;
            self.map_window(window.clone(), stack_x, y, right_w, final_h);
            y += stack_h + margin;
        }
    }

    pub fn arrange_tile_top(&mut self, dock: &mut PunkDock) {
        if self.windows.len() <= 1 {return self.arrange_max(dock);}
        let margin = self.space.vh();
        let output = self.space.outputs().next().cloned();
        let Some(output_geo) = output.and_then(|o| self.space.output_geometry(&o))else {return;};
        self.arrange_dock(dock);

        // ---------------------------------
        // Dock lateral izquierdo
        // ---------------------------------
        let dock_zone = dock.size() + margin;

        // ---------------------------------
        // Área útil
        // ---------------------------------
        let total_w = output_geo.size.w - dock_zone - (2 * margin);
        let total_h = output_geo.size.h - (2 * margin);

        // ---------------------------------
        // División vertical
        // ---------------------------------
        let top_h = (total_h / 2) - (margin / 2);
        let bottom_h = total_h - top_h - margin;

        // ---------------------------------
        // Ventana principal
        // ---------------------------------
        let master_x = output_geo.loc.x + dock_zone + margin;
        let master_y = output_geo.loc.y + margin;
        let windows: Vec<_> = self.windows.iter().filter(|w| w.alive()).cloned().collect();
        let master = windows[0].clone();
        let master_bar = if master.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
        let master_final_h = top_h - master_bar;
        self.map_window(master.clone(), master_x, master_y, total_w, master_final_h);
        
        // ---------------------------------
        // Stack inferior horizontal
        // ---------------------------------
        let stack_count = (self.windows.len() - 1) as i32;
        if stack_count <= 0 {return;}
        // gaps horizontales internos
        let available_w = total_w - (margin * (stack_count - 1));
        let stack_w = available_w / stack_count;
        let stack_y = master_y + top_h + margin;
        let mut x = master_x;
        for window in windows.iter().skip(1) {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = bottom_h - bar;
            self.map_window(window.clone(), x, stack_y, stack_w, final_h);
            x += stack_w + margin;
        }
    }

    pub fn arrange_tile_bottom(&mut self, dock: &mut PunkDock) {
        if self.windows.len() <= 1 {return self.arrange_max(dock);}
        let margin = self.space.vh();
        let output = self.space.outputs().next().cloned();
        let Some(output_geo) = output.and_then(|o| self.space.output_geometry(&o))else {return;};
        self.arrange_dock(dock);

        // ---------------------------------
        // Dock lateral izquierdo
        // ---------------------------------
        let dock_zone = dock.size() + margin;

        // ---------------------------------
        // Área útil
        // ---------------------------------
        let total_w = output_geo.size.w - dock_zone - (2 * margin);
        let total_h = output_geo.size.h - (2 * margin);

        // ---------------------------------
        // División vertical
        // ---------------------------------
        let top_h = (total_h / 2) - (margin / 2);
        let bottom_h = total_h - top_h - margin;
        let base_x = output_geo.loc.x + dock_zone + margin;
        let base_y = output_geo.loc.y + margin;
        let windows: Vec<_> = self.windows.iter().filter(|w| w.alive()).cloned().collect();
        let master = windows[0].clone();

        // ---------------------------------
        // Stack superior horizontal
        // ---------------------------------
        let stack_count = (self.windows.len() - 1) as i32;
        let available_w = total_w - (margin * (stack_count - 1));
        let stack_w = available_w / stack_count;
        let stack_y = base_y;
        let mut x = base_x;
        for window in windows.iter().skip(1) {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = top_h - bar;
            self.map_window(window.clone(), x, stack_y, stack_w, final_h);
            x += stack_w + margin;
        }

        // ---------------------------------
        // Master abajo
        // ---------------------------------
        let master_bar = if master.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
        let master_y = base_y + top_h + margin;
        let master_h = bottom_h - master_bar;
        self.map_window(master.clone(), base_x, master_y, total_w, master_h);
    }
}
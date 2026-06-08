use crate::workspace::Workspace;
use smithay::utils::IsAlive;
use crate::shell::ssd::HEADER_BAR_HEIGHT;
use crate::state::SpaceExt;
use crate::dock::PunkDock;
impl Workspace{

    pub fn arrange_fair_h(&mut self, dock: &mut PunkDock) {
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
        
        let windows: Vec<_> = self.windows.iter().filter(|w| w.alive()).cloned().collect();
        let stack_x = output_geo.loc.x + dock_zone + margin;

        
        // -------------------------
        // Stack derecho
        // -------------------------
        let stack_count = (self.windows.len()) as i32;
        if stack_count <= 0 { return;}
        // gaps internos verticales
        let available_h = total_h - (margin * (stack_count - 1));
        let stack_h = available_h / stack_count;
        let mut y = output_geo.loc.y + margin;
        for window in windows.iter() {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = stack_h - bar;
            self.map_window(window.clone(), stack_x, y, total_w, final_h);
            y += stack_h + margin;
        }
    }

     pub fn arrange_fair_v(&mut self, dock: &mut PunkDock) {
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
        
        let base_x = output_geo.loc.x + dock_zone + margin;
        let base_y = output_geo.loc.y + margin;
        let windows: Vec<_> = self.windows.iter().filter(|w| w.alive()).cloned().collect();
        // let master = windows[0].clone();

        // ---------------------------------
        // Stack horizontal
        // ---------------------------------
        let stack_count = (self.windows.len()) as i32;
        let available_w = total_w - (margin * (stack_count - 1));
        let stack_w = available_w / stack_count;
        let stack_y = base_y;
        let mut x = base_x;
        for window in windows.iter() {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = total_h - bar;
            self.map_window(window.clone(), x, stack_y, stack_w, final_h);
            x += stack_w + margin;
        }
    }


}
use crate::workspace::Workspace;
use smithay::utils::IsAlive;
use crate::shell::ssd::HEADER_BAR_HEIGHT;
use crate::state::SpaceExt;
use crate::dock::PunkDock;
impl Workspace{

    pub fn arrange_corner_nw(&mut self, dock: &mut PunkDock) {
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
        let left_w = (total_w * 7 / 10) - (margin / 2);
        let right_w = total_w - left_w - margin;
        let top_h = (total_h * 7 / 10) - (margin / 2);
        let bottom_h = total_h - top_h - margin;
        // -------------------------
        // Master window
        // -------------------------
        let master_x = output_geo.loc.x + dock_zone + margin;
        let master_y = output_geo.loc.y + margin;
        let windows: Vec<_> = self.windows.iter().filter(|w| w.alive()).cloned().collect();
        let master = windows[0].clone();
        let master_bar = if master.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
        // let master_h = total_h - master_bar;
        let master_h = if self.windows.len() == 2 {
            total_h - master_bar
        }else{
            top_h - master_bar

        };
        self.map_window(master.clone(), master_x, master_y, left_w, master_h);
        
        // -------------------------
        // Stack Principal
        // -------------------------
        let stack_x_p = master_x + left_w + margin;
        let stack_count_p = self.windows.iter().skip(1).step_by(2).count() as i32;
        if stack_count_p <= 0 { return;}

        // gaps internos verticales
        let available_h_p = total_h - (margin * (stack_count_p - 1));
        let stack_h_p = available_h_p / stack_count_p;
        let mut y_p = output_geo.loc.y + margin;

        for window in windows.iter().skip(1).step_by(2)  {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = stack_h_p - bar;
            self.map_window(window.clone(), stack_x_p, y_p, right_w, final_h);
            y_p += stack_h_p + margin;
        }


        // ---------------------------------
        // Stack inferior horizontal
        // ---------------------------------
        let stack_count_s = self.windows.iter().skip(2).step_by(2).count() as i32;
        if stack_count_s <= 0 {return;}
        // gaps horizontales internos
        let available_w_s = left_w - (margin * (stack_count_s - 1));
        let stack_w_s = available_w_s / stack_count_s;
        let stack_y_s = master_y + top_h + margin;
        let mut x_s = master_x;
        for window in windows.iter().skip(2).step_by(2) {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = bottom_h - bar;
            self.map_window(window.clone(), x_s, stack_y_s, stack_w_s, final_h);
            x_s += stack_w_s + margin;
        }

    }

    pub fn arrange_corner_ne(&mut self, dock: &mut PunkDock) {
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
        let left_w = (total_w * 3 / 10) - (margin / 2);
        let right_w = total_w - left_w - margin;
        let top_h = (total_h * 7 / 10) - (margin / 2);
        let bottom_h = total_h - top_h - margin;
        // -------------------------
        // Master window
        // -------------------------
        let stack_x_p = output_geo.loc.x + dock_zone + margin;
        let master_x = left_w + stack_x_p + margin;
        let master_y = output_geo.loc.y + margin;
        let windows: Vec<_> = self.windows.iter().filter(|w| w.alive()).cloned().collect();
        let master = windows[0].clone();
        let master_bar = if master.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
        // let master_h = total_h - master_bar;
        let master_h = if self.windows.len() == 2 {
            total_h - master_bar
        }else{
            top_h - master_bar

        };
        self.map_window(master.clone(), master_x, master_y, right_w, master_h);
        
        // -------------------------
        // Stack Principal
        // -------------------------
        

        let stack_count_p = self.windows.iter().skip(1).step_by(2).count() as i32;
        if stack_count_p <= 0 { return;}

        // gaps internos verticales
        let available_h_p = total_h - (margin * (stack_count_p - 1));
        let stack_h_p = available_h_p / stack_count_p;
        let mut y_p = output_geo.loc.y + margin;

        for window in windows.iter().skip(1).step_by(2)  {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = stack_h_p - bar;
            self.map_window(window.clone(), stack_x_p, y_p, left_w, final_h);
            y_p += stack_h_p + margin;
        }


        // ---------------------------------
        // Stack inferior horizontal
        // ---------------------------------
        let stack_count_s = self.windows.iter().skip(2).step_by(2).count() as i32;
        if stack_count_s <= 0 {return;}
        // gaps horizontales internos
        let available_w_s = right_w - (margin * (stack_count_s - 1));
        let stack_w_s = available_w_s / stack_count_s;
        let stack_y_s = master_y + top_h + margin;
        let mut x_s = master_x;
        for window in windows.iter().skip(2).step_by(2) {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = bottom_h - bar;
            self.map_window(window.clone(), x_s, stack_y_s, stack_w_s, final_h);
            x_s += stack_w_s + margin;
        }

    }

    pub fn arrange_corner_sw(&mut self, dock: &mut PunkDock) {
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
        let left_w = (total_w * 7 / 10) - (margin / 2);
        let right_w = total_w - left_w - margin;
        let top_h = (total_h * 3 / 10) - (margin / 2);
        let bottom_h = total_h - top_h - margin;
        // -------------------------
        // Master window
        // -------------------------

        let stack_y_s = output_geo.loc.y + margin;
        let master_x = output_geo.loc.x + dock_zone + margin;
        let master_y = if self.windows.len() == 2 {
            output_geo.loc.y + margin
        }else{
            stack_y_s + top_h + margin
        };
        let windows: Vec<_> = self.windows.iter().filter(|w| w.alive()).cloned().collect();
        let master = windows[0].clone();
        let master_bar = if master.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
        // let master_h = total_h - master_bar;
        let master_h = if self.windows.len() == 2 {
            total_h - master_bar
        }else{
            bottom_h - master_bar

        };
        self.map_window(master.clone(), master_x, master_y, left_w, master_h);
        
        // -------------------------
        // Stack Principal
        // -------------------------
        let stack_x_p = master_x + left_w + margin;
        let stack_count_p = self.windows.iter().skip(1).step_by(2).count() as i32;
        if stack_count_p <= 0 { return;}

        // gaps internos verticales
        let available_h_p = total_h - (margin * (stack_count_p - 1));
        let stack_h_p = available_h_p / stack_count_p;
        let mut y_p = output_geo.loc.y + margin;

        for window in windows.iter().skip(1).step_by(2)  {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = stack_h_p - bar;
            self.map_window(window.clone(), stack_x_p, y_p, right_w, final_h);
            y_p += stack_h_p + margin;
        }


        // ---------------------------------
        // Stack inferior horizontal
        // ---------------------------------
        let stack_count_s = self.windows.iter().skip(2).step_by(2).count() as i32;
        if stack_count_s <= 0 {return;}
        // gaps horizontales internos
        let available_w_s = left_w - (margin * (stack_count_s - 1));
        let stack_w_s = available_w_s / stack_count_s;
        
        let mut x_s = master_x;
        for window in windows.iter().skip(2).step_by(2) {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = top_h - bar;
            self.map_window(window.clone(), x_s, stack_y_s, stack_w_s, final_h);
            x_s += stack_w_s + margin;
        }

    }

    pub fn arrange_corner_se(&mut self, dock: &mut PunkDock) {
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
        let left_w = (total_w * 3 / 10) - (margin / 2);
        let right_w = total_w - left_w - margin;
        let top_h = (total_h * 3 / 10) - (margin / 2);
        let bottom_h = total_h - top_h - margin;
        // -------------------------
        // Master window
        // -------------------------
        let stack_x_p = output_geo.loc.x + dock_zone + margin;
        let stack_y_s = output_geo.loc.y + margin;
        let master_x = left_w + stack_x_p + margin;
        let master_y = if self.windows.len() == 2 {
             output_geo.loc.y + margin
            }else{
                stack_y_s + top_h + margin
            };
        let windows: Vec<_> = self.windows.iter().filter(|w| w.alive()).cloned().collect();
        let master = windows[0].clone();
        let master_bar = if master.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
        // let master_h = total_h - master_bar;
        let master_h = if self.windows.len() == 2 {
            total_h - master_bar
        }else{
            bottom_h - master_bar

        };
        self.map_window(master.clone(), master_x, master_y, right_w, master_h);
        
        // -------------------------
        // Stack Principal
        // -------------------------
        

        let stack_count_p = self.windows.iter().skip(1).step_by(2).count() as i32;
        if stack_count_p <= 0 { return;}

        // gaps internos verticales
        let available_h_p = total_h - (margin * (stack_count_p - 1));
        let stack_h_p = available_h_p / stack_count_p;
        let mut y_p = output_geo.loc.y + margin;

        for window in windows.iter().skip(1).step_by(2)  {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = stack_h_p - bar;
            self.map_window(window.clone(), stack_x_p, y_p, left_w, final_h);
            y_p += stack_h_p + margin;
        }


        // ---------------------------------
        // Stack inferior horizontal
        // ---------------------------------
        let stack_count_s = self.windows.iter().skip(2).step_by(2).count() as i32;
        if stack_count_s <= 0 {return;}
        // gaps horizontales internos
        let available_w_s = right_w - (margin * (stack_count_s - 1));
        let stack_w_s = available_w_s / stack_count_s;
        
        let mut x_s = master_x;
        for window in windows.iter().skip(2).step_by(2) {
            let bar = if window.decoration_state().is_ssd {HEADER_BAR_HEIGHT} else {0};
            let final_h = top_h - bar;
            self.map_window(window.clone(), x_s, stack_y_s, stack_w_s, final_h);
            x_s += stack_w_s + margin;
        }

    }
}
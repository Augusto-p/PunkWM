use crate::workspace::Workspace;
use smithay::utils::IsAlive;
use crate::shell::ssd::HEADER_BAR_HEIGHT;
use crate::state::SpaceExt;

use crate::dock::PunkDock;
impl Workspace{
     pub fn arrange_max(&mut self, dock: &mut PunkDock) {
        let margin = self.space.vh();
        let output = self.space.outputs().next().cloned();
        let Some(output_geo) = output.and_then(|o| self.space.output_geometry(&o)) else {return;};
        self.arrange_dock(dock);
        let windows = self.windows.iter().filter(|w| w.alive());
        let Some(window) = windows.last().cloned() else {return;};
        let dock_zone = dock.size() + margin;
        let new_x = output_geo.loc.x + margin + dock_zone;
        let new_y = output_geo.loc.y + margin;
        let bar = if window.decoration_state().is_ssd { HEADER_BAR_HEIGHT } else { 0 };
        let new_w = output_geo.size.w - (2 * margin) - dock_zone;
        let new_h = output_geo.size.h - (2 * margin) - bar;
        self.map_window(window.clone(), new_x, new_y, new_w, new_h);       
    }
}
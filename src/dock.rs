use crate::config::config::GLOBAL_CFG;
use crate::utils::random::random_string;
use crate::shell::WindowElement;
use std::env;
use std::process::Command;
#[derive(Debug)]
pub struct PunkDock{
    pub window: Option<WindowElement>,
    pub lockwindow: Option<WindowElement>,
    pub name: String,
    pub pannel: bool,
}

impl PunkDock{
   pub fn new()->Self{
      Self {
            window: None,
            lockwindow: None,
            name: format!("PunkDock_{}", random_string(32)),
            pannel: false
        }
     }

   pub fn make(&self){
      let current_dir = env::current_dir().unwrap();
      Command::new("./com_punkwm_dock_")
         .current_dir(current_dir)
         .env("WEBKIT_INSPECTOR_SERVER", "127.0.0.1:9222")
         .env("WEBKIT_DISABLE_DMABUF_RENDERER", "1")
         .args(&[format!("--title={}", &self.name),])
         .spawn()
         .expect("failed to start dock");
   }

   pub fn open(&mut self){self.pannel = true;}
   pub fn closed(&mut self){self.pannel = false;}
   pub fn is_open(&self)->bool{self.pannel == true}
   pub fn is_lock(&self)->bool{self.lockwindow != None}
   pub fn size(&self)->i32{
      if self.window == None{ return 0}
      let cfg = GLOBAL_CFG.read().expect("No se pudo bloquear para lectura");
      if self.is_open() {
         (cfg.styles().get_dock_width()+ cfg.styles().get_pannel_width()) as i32
      } else {
         cfg.styles().get_dock_width() as i32
      }

   }
}
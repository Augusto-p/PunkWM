use std::{convert::TryInto, process::Command, sync::atomic::Ordering};
use crate::{AnvilState, focus::PointerFocusTarget, shell::FullscreenSurface};
use crate::config::config::GLOBAL_CFG;
use std::collections::HashMap;
use crate::utils::system_usage::SystemUsage;
use crate::utils::volume::Volume;
use crate::utils::tools::spawn;
use crate::utils::brightness::Brightness;
use crate::udev::UdevData;
use smithay::backend::renderer::DebugFlags;
use serde_json::json;
use crate::ipc::message::{IpcMessage,IpcMode};
use crate::focus::KeyboardFocusTarget;
use smithay::{
    backend::input::{
        self, Axis, AxisSource, Event, InputBackend, InputEvent, KeyState, KeyboardKeyEvent,
        PointerAxisEvent, PointerButtonEvent,
    },
    desktop::{WindowSurfaceType, layer_map_for_output},
    input::{
        keyboard::{FilterResult, keysyms as xkb},
        pointer::{AxisFrame, ButtonEvent, MotionEvent},
    },
    output::Scale,
    reexports::{
        wayland_protocols::xdg::decoration::zv1::server::zxdg_toplevel_decoration_v1,
        wayland_server::protocol::wl_pointer,
    },
    utils::{Logical, Point, SERIAL_COUNTER as SCOUNTER, Serial, Transform},
    wayland::{
        input_method::InputMethodSeat,
        keyboard_shortcuts_inhibit::KeyboardShortcutsInhibitorSeat,
        shell::wlr_layer::{KeyboardInteractivity, Layer as WlrLayer},
    },
};
use xkbcommon::xkb as other_xkb;
use smithay::backend::input::AbsolutePositionEvent;
use tracing::{debug, error, info};
use crate::state::Backend;
use smithay::{
    backend::{
        input::{
            Device, DeviceCapability, GestureBeginEvent, GestureEndEvent, GesturePinchUpdateEvent as _,
            GestureSwipeUpdateEvent as _, PointerMotionEvent, ProximityState, TabletToolButtonEvent,
            TabletToolEvent, TabletToolProximityEvent, TabletToolTipEvent, TabletToolTipState, TouchEvent,
        },
        session::Session,
    },
    input::{
        pointer::{
            GestureHoldBeginEvent, GestureHoldEndEvent, GesturePinchBeginEvent, GesturePinchEndEvent,
            GesturePinchUpdateEvent, GestureSwipeBeginEvent, GestureSwipeEndEvent, GestureSwipeUpdateEvent,
            RelativeMotionEvent,
        },
        touch::{DownEvent, UpEvent},
    },
    reexports::wayland_server::DisplayHandle,
    wayland::{
        pointer_constraints::{PointerConstraint, with_pointer_constraint},
        seat::WaylandFocus,
        tablet_manager::{TabletDescriptor, TabletSeatTrait},
    },
};

impl<BackendData: Backend> AnvilState<BackendData> {
    // Allow in this method because of existing usage
    #[allow(clippy::uninlined_format_args)]
    fn process_common_key_action(&mut self, action: KeyAction) {
        match action {
            KeyAction::None => (),

            KeyAction::Quit => {
                info!("Quitting.");
                self.running.store(false, Ordering::SeqCst);
            }

            KeyAction::Run(cmd) => {
                info!(cmd, "Starting program");

                if let Err(e) = Command::new(&cmd)
                    .envs(
                        self.socket_name
                            .clone()
                            .map(|v| ("WAYLAND_DISPLAY", v))
                            .into_iter()
                            .chain(
                                #[cfg(feature = "xwayland")]
                                self.xdisplay.map(|v| ("DISPLAY", format!(":{v}"))),
                                #[cfg(not(feature = "xwayland"))]
                                None,
                            ),
                    )
                    .spawn()
                {
                    error!(cmd, err = %e, "Failed to start program");
                }
            }

            KeyAction::Test=>{
                
                println!("TEST {:?}", self.current_workspace().windows)
                
            }

            KeyAction::SwitchWorkspaceTo1 =>{self.switch_workspace(1);}
            KeyAction::SwitchWorkspaceTo2 =>{self.switch_workspace(2);}
            KeyAction::SwitchWorkspaceTo3 =>{self.switch_workspace(3);}
            KeyAction::SwitchWorkspaceTo4 =>{self.switch_workspace(4);}
            KeyAction::SwitchWorkspaceTo5 =>{self.switch_workspace(5);}
            KeyAction::SwitchWorkspaceTo6 =>{self.switch_workspace(6);}
            KeyAction::SwitchWorkspaceTo7 =>{self.switch_workspace(7);}
            KeyAction::SwitchWorkspaceTo8 =>{self.switch_workspace(8);}
            KeyAction::SwitchWorkspaceTo9 =>{self.switch_workspace(9);}
            KeyAction::MoveToWorksace1 => {self.move_to_workspace(1);}
            KeyAction::MoveToWorksace2 => {self.move_to_workspace(2);}
            KeyAction::MoveToWorksace3 => {self.move_to_workspace(3);}
            KeyAction::MoveToWorksace4 => {self.move_to_workspace(4);}
            KeyAction::MoveToWorksace5 => {self.move_to_workspace(5);}
            KeyAction::MoveToWorksace6 => {self.move_to_workspace(6);}
            KeyAction::MoveToWorksace7 => {self.move_to_workspace(7);}
            KeyAction::MoveToWorksace8 => {self.move_to_workspace(8);}
            KeyAction::MoveToWorksace9 => {self.move_to_workspace(9);}
            KeyAction::MoveAndSwitchWorkspaceTo1 =>{self.switch_and_move_to_workspace(1);}
            KeyAction::MoveAndSwitchWorkspaceTo2 =>{self.switch_and_move_to_workspace(2);}
            KeyAction::MoveAndSwitchWorkspaceTo3 =>{self.switch_and_move_to_workspace(3);}
            KeyAction::MoveAndSwitchWorkspaceTo4 =>{self.switch_and_move_to_workspace(4);}
            KeyAction::MoveAndSwitchWorkspaceTo5 =>{self.switch_and_move_to_workspace(5);}
            KeyAction::MoveAndSwitchWorkspaceTo6 =>{self.switch_and_move_to_workspace(6);}
            KeyAction::MoveAndSwitchWorkspaceTo7 =>{self.switch_and_move_to_workspace(7);}
            KeyAction::MoveAndSwitchWorkspaceTo8 =>{self.switch_and_move_to_workspace(8);}
            KeyAction::MoveAndSwitchWorkspaceTo9 =>{self.switch_and_move_to_workspace(9);}
            KeyAction::WindowClose =>{self.window_closed();}
            KeyAction::FocusNext => {self.focus_next();}
            KeyAction::FocusPrevious =>{self.focus_prev();}
            KeyAction::ToggleLayout => {self.toggle_layout();}
            KeyAction::WindowPrevious =>{self.window_prev()}
            KeyAction::WindowNext =>{self.window_next()}


            KeyAction::TogglePreview => {self.show_window_preview = !self.show_window_preview;}

            KeyAction::ToggleDecorations => {
                for element in self.current_workspace().space.elements() {
                    #[allow(irrefutable_let_patterns)]
                    if let Some(toplevel) = element.window.toplevel() {
                        let mode_changed = toplevel.with_pending_state(|state| {
                            if let Some(current_mode) = state.decoration_mode {
                                let new_mode =
                                    if current_mode == zxdg_toplevel_decoration_v1::Mode::ClientSide {
                                        zxdg_toplevel_decoration_v1::Mode::ServerSide
                                    } else {
                                        zxdg_toplevel_decoration_v1::Mode::ClientSide
                                    };
                                state.decoration_mode = Some(new_mode);
                                true
                            } else {
                                false
                            }
                        });

                        if mode_changed && toplevel.is_initial_configure_sent() {
                            toplevel.send_pending_configure();
                        }
                    }
                }
            }

            _ => unreachable!(
                "Common key action handler encountered backend specific action {:?}",
                action
            ),
        }
    }

    fn keyboard_key_to_action<B: InputBackend>(&mut self, evt: B::KeyboardKeyEvent) -> KeyAction {
        let keycode = evt.key_code();
        let state = evt.state();
        debug!(?keycode, ?state, "key");
        let serial = SCOUNTER.next_serial();
        let time = Event::time_msec(&evt);
        let mut suppressed_keys = self.suppressed_keys.clone();
        let keyboard = self.seat.get_keyboard().unwrap();

        for layer in self.layer_shell_state.layer_surfaces().rev() {
            let exclusive = layer.with_cached_state(|data| {
                data.keyboard_interactivity == KeyboardInteractivity::Exclusive
                    && (data.layer == WlrLayer::Top || data.layer == WlrLayer::Overlay)
            });
            if exclusive {
                let surface = self.current_workspace().space.outputs().find_map(|o| {
                    let map = layer_map_for_output(o);
                    map.layers().find(|l| l.layer_surface() == &layer).cloned()
                });
                if let Some(surface) = surface {
                    keyboard.set_focus(self, Some(surface.into()), serial);
                    keyboard.input::<(), _>(self, keycode, state, serial, time, |_, _, _| {
                        FilterResult::Forward
                    });
                    return KeyAction::None;
                };
            }
        }

        let inhibited = self
            .current_workspace().space
            .element_under(self.pointer.current_location())
            .and_then(|(window, _)| {
                let surface = window.wl_surface()?;
                self.seat.keyboard_shortcuts_inhibitor_for_surface(&surface)
            })
            .map(|inhibitor| inhibitor.is_active())
            .unwrap_or(false);


        let keybindings = self.keybindings.clone();

        let action = keyboard.input(self, keycode, state, serial, time, |_, modifiers, handle| {
            let keysym = handle.modified_sym();
            let keysymraw = handle.raw_syms()[0];
                
            let normalized = if let Some(c) = char::from_u32(keysymraw.into()) {c.to_ascii_uppercase() as u32} else {keysymraw.raw()};
            debug!(?state,mods = ?modifiers,keysym = ::xkbcommon::xkb::keysym_get_name(keysym),"keysym");
            if let KeyState::Pressed = state {
                if !inhibited {
                    let atajo = format!("{}{}{}{}{}",modifiers.ctrl as u32,modifiers.logo as u32,modifiers.alt as u32,modifiers.shift as u32,normalized,);
                    let action = keybindings.get(atajo);
                    println!("{:?}",action);
                    if action == Some(KeyAction::None) {
                        println!("hry");
                        return FilterResult::Forward;
                    }
                    
                    if action.is_some() {
                        suppressed_keys.push(keysym);
                    }
                    action.map(FilterResult::Intercept).unwrap_or(FilterResult::Forward)

                } else {FilterResult::Forward}
            } else {
                let suppressed = suppressed_keys.contains(&keysym);
                if suppressed {
                    suppressed_keys.retain(|k| *k != keysym);
                    FilterResult::Intercept(KeyAction::None)
                } else {FilterResult::Forward}
            }
        }).unwrap_or(KeyAction::None);
        
        self.suppressed_keys = suppressed_keys;
                
        action
    }

    fn on_pointer_button<B: InputBackend>(&mut self, evt: B::PointerButtonEvent) {
        let serial = SCOUNTER.next_serial();
        let button = evt.button_code();

        let state = wl_pointer::ButtonState::from(evt.state());

        if wl_pointer::ButtonState::Pressed == state {
            self.update_keyboard_focus(self.pointer.current_location(), serial);
        };
        let pointer = self.pointer.clone();
        pointer.button(
            self,
            &ButtonEvent {
                button,
                state: state.try_into().unwrap(),
                serial,
                time: evt.time_msec(),
            },
        );
        pointer.frame(self);
    }

    fn update_keyboard_focus(&mut self, location: Point<f64, Logical>, serial: Serial) {
        let keyboard = self.seat.get_keyboard().unwrap();
        let touch = self.seat.get_touch();
        let input_method = self.seat.input_method();
        // change the keyboard focus unless the pointer or keyboard is grabbed
        // We test for any matching surface type here but always use the root
        // (in case of a window the toplevel) surface for the focus.
        // So for example if a user clicks on a subsurface or popup the toplevel
        // will receive the keyboard focus. Directly assigning the focus to the
        // matching surface leads to issues with clients dismissing popups and
        // subsurface menus (for example firefox-wayland).
        // see here for a discussion about that issue:
        // https://gitlab.freedesktop.org/wayland/wayland/-/issues/294
        if !self.pointer.is_grabbed()
            && (!keyboard.is_grabbed() || input_method.keyboard_grabbed())
            && !touch.map(|touch| touch.is_grabbed()).unwrap_or(false)
        {
            let output = self.current_workspace_mut().space.output_under(location).next().cloned();
            if let Some(output) = output.as_ref() {
                let output_geo = self.current_workspace_mut().space.output_geometry(output).unwrap();
                if let Some(window) = output.user_data().get::<FullscreenSurface>().and_then(|f| f.get()){
                    if let Some((_, _)) = window.surface_under(location - output_geo.loc.to_f64(), WindowSurfaceType::ALL){
                        #[cfg(feature = "xwayland")]
                        if let Some(surface) = window.window.x11_surface() {
                            self.xwm.as_mut().unwrap().raise_window(surface).unwrap();
                        }
                        keyboard.set_focus(self, Some(window.into()), serial);

                        return;
                    }
                }

                let layers = layer_map_for_output(output);
                if let Some(layer) = layers
                    .layer_under(WlrLayer::Overlay, location - output_geo.loc.to_f64())
                    .or_else(|| layers.layer_under(WlrLayer::Top, location - output_geo.loc.to_f64()))
                {
                    if layer.can_receive_keyboard_focus() {
                        if let Some((_, _)) = layer.surface_under(
                            location
                                - output_geo.loc.to_f64()
                                - layers.layer_geometry(layer).unwrap().loc.to_f64(),
                            WindowSurfaceType::ALL,
                        ) {
                            keyboard.set_focus(self, Some(layer.clone().into()), serial);
                            return;
                        }
                    }
                }
            }

            if let Some((window, _)) = self.current_workspace_mut().space.element_under(location).map(|(w, p)| (w.clone(), p)) {
                self.current_workspace_mut().space.raise_element(&window, true);
                #[cfg(feature = "xwayland")]
                if let Some(surface) = window.window.x11_surface() {
                    self.xwm.as_mut().unwrap().raise_window(surface).unwrap();
                }
                keyboard.set_focus(self, Some(window.into()), serial);
                return;
            }

            if let Some(output) = output.as_ref() {
                let output_geo = self.current_workspace_mut().space.output_geometry(output).unwrap();
                let layers = layer_map_for_output(output);
                if let Some(layer) = layers
                    .layer_under(WlrLayer::Bottom, location - output_geo.loc.to_f64())
                    .or_else(|| layers.layer_under(WlrLayer::Background, location - output_geo.loc.to_f64()))
                {
                    if layer.can_receive_keyboard_focus() {
                        if let Some((_, _)) = layer.surface_under(
                            location
                                - output_geo.loc.to_f64()
                                - layers.layer_geometry(layer).unwrap().loc.to_f64(),
                            WindowSurfaceType::ALL,
                        ) {
                            keyboard.set_focus(self, Some(layer.clone().into()), serial);
                        }
                    }
                }
            };
        }
    }

    pub fn surface_under(&self,pos: Point<f64, Logical>,) -> Option<(PointerFocusTarget, Point<f64, Logical>)> {
        let output = self.current_workspace().space.outputs().find(|o| {
            let geometry = self.current_workspace().space.output_geometry(o).unwrap();
            geometry.contains(pos.to_i32_round())
        })?;
        let output_geo = self.current_workspace().space.output_geometry(output).unwrap();
        let layers = layer_map_for_output(output);

        let mut under = None;
        if let Some((surface, loc)) = output
            .user_data()
            .get::<FullscreenSurface>()
            .and_then(|f| f.get())
            .and_then(|w| w.surface_under(pos - output_geo.loc.to_f64(), WindowSurfaceType::ALL))
        {
            under = Some((surface, loc + output_geo.loc));
        } else if let Some(focus) = layers
            .layer_under(WlrLayer::Overlay, pos - output_geo.loc.to_f64())
            .or_else(|| layers.layer_under(WlrLayer::Top, pos - output_geo.loc.to_f64()))
            .and_then(|layer| {
                let layer_loc = layers.layer_geometry(layer).unwrap().loc;
                layer
                    .surface_under(
                        pos - output_geo.loc.to_f64() - layer_loc.to_f64(),
                        WindowSurfaceType::ALL,
                    )
                    .map(|(surface, loc)| {
                        (
                            PointerFocusTarget::from(surface),
                            loc + layer_loc + output_geo.loc,
                        )
                    })
            })
        {
            under = Some(focus)
        } else if let Some(focus) = self.current_workspace().space.element_under(pos).and_then(|(window, loc)| {
            window
                .surface_under(pos - loc.to_f64(), WindowSurfaceType::ALL)
                .map(|(surface, surf_loc)| (surface, surf_loc + loc))
        }) {
            under = Some(focus);
        } else if let Some(focus) = layers
            .layer_under(WlrLayer::Bottom, pos - output_geo.loc.to_f64())
            .or_else(|| layers.layer_under(WlrLayer::Background, pos - output_geo.loc.to_f64()))
            .and_then(|layer| {
                let layer_loc = layers.layer_geometry(layer).unwrap().loc;
                layer
                    .surface_under(
                        pos - output_geo.loc.to_f64() - layer_loc.to_f64(),
                        WindowSurfaceType::ALL,
                    )
                    .map(|(surface, loc)| {
                        (
                            PointerFocusTarget::from(surface),
                            loc + layer_loc + output_geo.loc,
                        )
                    })
            })
        {
            under = Some(focus)
        };
        under.map(|(s, l)| (s, l.to_f64()))
    }

    fn on_pointer_axis<B: InputBackend>(&mut self, evt: B::PointerAxisEvent) {
        let horizontal_amount = evt
            .amount(input::Axis::Horizontal)
            .unwrap_or_else(|| evt.amount_v120(input::Axis::Horizontal).unwrap_or(0.0) * 15.0 / 120.);
        let vertical_amount = evt
            .amount(input::Axis::Vertical)
            .unwrap_or_else(|| evt.amount_v120(input::Axis::Vertical).unwrap_or(0.0) * 15.0 / 120.);
        let horizontal_amount_discrete = evt.amount_v120(input::Axis::Horizontal);
        let vertical_amount_discrete = evt.amount_v120(input::Axis::Vertical);

        {
            let mut frame = AxisFrame::new(evt.time_msec()).source(evt.source());
            if horizontal_amount != 0.0 {
                frame = frame.relative_direction(Axis::Horizontal, evt.relative_direction(Axis::Horizontal));
                frame = frame.value(Axis::Horizontal, horizontal_amount);
                if let Some(discrete) = horizontal_amount_discrete {
                    frame = frame.v120(Axis::Horizontal, discrete as i32);
                }
            }
            if vertical_amount != 0.0 {
                frame = frame.relative_direction(Axis::Vertical, evt.relative_direction(Axis::Vertical));
                frame = frame.value(Axis::Vertical, vertical_amount);
                if let Some(discrete) = vertical_amount_discrete {
                    frame = frame.v120(Axis::Vertical, discrete as i32);
                }
            }
            if evt.source() == AxisSource::Finger {
                if evt.amount(Axis::Horizontal) == Some(0.0) {
                    frame = frame.stop(Axis::Horizontal);
                }
                if evt.amount(Axis::Vertical) == Some(0.0) {
                    frame = frame.stop(Axis::Vertical);
                }
            }
            let pointer = self.pointer.clone();
            pointer.axis(self, frame);
            pointer.frame(self);
        }
    }
}


impl AnvilState<UdevData> {
    pub fn process_input_event<B: InputBackend>(&mut self, dh: &DisplayHandle, event: InputEvent<B>) {
        match event {
            InputEvent::Keyboard { event, .. } => match self.keyboard_key_to_action::<B>(event) {
            
                KeyAction::VtSwitch(vt) => {
                    info!(to = vt, "Trying to switch vt");
                    if let Err(err) = self.backend_data.session.change_vt(vt) {
                        error!(vt, "Error switching vt: {}", err);
                    }
                }
                KeyAction::Screen(num) => {
                    let geometry = self
                        .current_workspace().space
                        .outputs()
                        .nth(num)
                        .map(|o| self.current_workspace().space.output_geometry(o).unwrap());

                    if let Some(geometry) = geometry {
                        let x = geometry.loc.x as f64 + geometry.size.w as f64 / 2.0;
                        let y = geometry.size.h as f64 / 2.0;
                        let location = (x, y).into();
                        let pointer = self.pointer.clone();
                        let under = self.surface_under(location);
                        pointer.motion(
                            self,
                            under,
                            &MotionEvent {
                                location,
                                serial: SCOUNTER.next_serial(),
                                time: self.clock.now().as_millis(),
                            },
                        );
                        pointer.frame(self);
                    }
                }
                KeyAction::ScaleUp => {
                    let pos = self.pointer.current_location().to_i32_round();
                    let output = self
                        .current_workspace().space
                        .outputs()
                        .find(|o| self.current_workspace().space.output_geometry(o).unwrap().contains(pos))
                        .cloned();

                    if let Some(output) = output {
                        let (output_location, scale) = (
                            self.current_workspace().space.output_geometry(&output).unwrap().loc,
                            output.current_scale().fractional_scale(),
                        );
                        let new_scale = scale + 0.25;
                        output.change_current_state(None, None, Some(Scale::Fractional(new_scale)), None);

                        let rescale = scale / new_scale;
                        let output_location = output_location.to_f64();
                        let mut pointer_output_location = self.pointer.current_location() - output_location;
                        pointer_output_location.x *= rescale;
                        pointer_output_location.y *= rescale;
                        let pointer_location = output_location + pointer_output_location;
                        self.current_workspace_arrange();
                        // crate::shell::fixup_positions(&mut self.current_workspace().space, pointer_location);
                        let pointer = self.pointer.clone();
                        let under = self.surface_under(pointer_location);
                        pointer.motion(
                            self,
                            under,
                            &MotionEvent {
                                location: pointer_location,
                                serial: SCOUNTER.next_serial(),
                                time: self.clock.now().as_millis(),
                            },
                        );
                        pointer.frame(self);
                        self.backend_data.reset_buffers(&output);
                    }
                }
                KeyAction::ScaleDown => {
                    let pos = self.pointer.current_location().to_i32_round();
                    let output = self
                        .current_workspace().space
                        .outputs()
                        .find(|o| self.current_workspace().space.output_geometry(o).unwrap().contains(pos))
                        .cloned();

                    if let Some(output) = output {
                        let (output_location, scale) = (
                            self.current_workspace().space.output_geometry(&output).unwrap().loc,
                            output.current_scale().fractional_scale(),
                        );
                        let new_scale = f64::max(1.0, scale - 0.25);
                        output.change_current_state(None, None, Some(Scale::Fractional(new_scale)), None);

                        let rescale = scale / new_scale;
                        let output_location = output_location.to_f64();
                        let mut pointer_output_location = self.pointer.current_location() - output_location;
                        pointer_output_location.x *= rescale;
                        pointer_output_location.y *= rescale;
                        let pointer_location = output_location + pointer_output_location;
                        self.current_workspace_arrange();
                        // crate::shell::fixup_positions(&mut self.current_workspace().space, pointer_location);
                        let pointer = self.pointer.clone();
                        let under = self.surface_under(pointer_location);
                        pointer.motion(
                            self,
                            under,
                            &MotionEvent {
                                location: pointer_location,
                                serial: SCOUNTER.next_serial(),
                                time: self.clock.now().as_millis(),
                            },
                        );
                        pointer.frame(self);
                        self.backend_data.reset_buffers(&output);
                    }
                }
                KeyAction::RotateOutput => {
                    let pos = self.pointer.current_location().to_i32_round();
                    let output = self
                        .current_workspace().space
                        .outputs()
                        .find(|o| self.current_workspace().space.output_geometry(o).unwrap().contains(pos))
                        .cloned();

                    if let Some(output) = output {
                        let current_transform = output.current_transform();
                        let new_transform = match current_transform {
                            Transform::Normal => Transform::_90,
                            Transform::_90 => Transform::_180,
                            Transform::_180 => Transform::_270,
                            Transform::_270 => Transform::Flipped,
                            Transform::Flipped => Transform::Flipped90,
                            Transform::Flipped90 => Transform::Flipped180,
                            Transform::Flipped180 => Transform::Flipped270,
                            Transform::Flipped270 => Transform::Normal,
                        };
                        output.change_current_state(None, Some(new_transform), None, None);
                        self.current_workspace_arrange();
                        // crate::shell::fixup_positions(&mut self.current_workspace().space, self.pointer.current_location());
                        self.backend_data.reset_buffers(&output);
                    }
                }
                KeyAction::ToggleTint => {
                    let mut debug_flags = self.backend_data.debug_flags();
                    debug_flags.toggle(DebugFlags::TINT);
                    self.backend_data.set_debug_flags(debug_flags);
                }

                action => match action {
                    KeyAction::None
                    | KeyAction::Quit
                    | KeyAction::Run(_)
                    | KeyAction::Test
                    | KeyAction::TogglePreview
                    | KeyAction::SwitchWorkspaceTo1
                    | KeyAction::SwitchWorkspaceTo2
                    | KeyAction::SwitchWorkspaceTo3
                    | KeyAction::SwitchWorkspaceTo4
                    | KeyAction::SwitchWorkspaceTo5
                    | KeyAction::SwitchWorkspaceTo6
                    | KeyAction::SwitchWorkspaceTo7
                    | KeyAction::SwitchWorkspaceTo8
                    | KeyAction::SwitchWorkspaceTo9
                    | KeyAction::MoveToWorksace1
                    | KeyAction::MoveToWorksace2
                    | KeyAction::MoveToWorksace3
                    | KeyAction::MoveToWorksace4
                    | KeyAction::MoveToWorksace5
                    | KeyAction::MoveToWorksace6
                    | KeyAction::MoveToWorksace7
                    | KeyAction::MoveToWorksace8
                    | KeyAction::MoveToWorksace9
                    | KeyAction::MoveAndSwitchWorkspaceTo1
                    | KeyAction::MoveAndSwitchWorkspaceTo2
                    | KeyAction::MoveAndSwitchWorkspaceTo3
                    | KeyAction::MoveAndSwitchWorkspaceTo4
                    | KeyAction::MoveAndSwitchWorkspaceTo5
                    | KeyAction::MoveAndSwitchWorkspaceTo6
                    | KeyAction::MoveAndSwitchWorkspaceTo7
                    | KeyAction::MoveAndSwitchWorkspaceTo8
                    | KeyAction::MoveAndSwitchWorkspaceTo9
                    | KeyAction::FocusNext
                    | KeyAction::FocusPrevious
                    | KeyAction::WindowClose
                    | KeyAction::WindowPrevious
                    | KeyAction::WindowNext
                    | KeyAction::ToggleDecorations
                    | KeyAction::ToggleLayout => self.process_common_key_action(action),

                    _ => unreachable!(),
                },
            },
            InputEvent::PointerMotion { event, .. } => self.on_pointer_move::<B>(dh, event),
            InputEvent::PointerMotionAbsolute { event, .. } => self.on_pointer_move_absolute::<B>(dh, event),
            InputEvent::PointerButton { event, .. } => self.on_pointer_button::<B>(event),
            InputEvent::PointerAxis { event, .. } => self.on_pointer_axis::<B>(event),
            InputEvent::TabletToolAxis { event, .. } => self.on_tablet_tool_axis::<B>(event),
            InputEvent::TabletToolProximity { event, .. } => self.on_tablet_tool_proximity::<B>(dh, event),
            InputEvent::TabletToolTip { event, .. } => self.on_tablet_tool_tip::<B>(event),
            InputEvent::TabletToolButton { event, .. } => self.on_tablet_button::<B>(event),
            InputEvent::GestureSwipeBegin { event, .. } => self.on_gesture_swipe_begin::<B>(event),
            InputEvent::GestureSwipeUpdate { event, .. } => self.on_gesture_swipe_update::<B>(event),
            InputEvent::GestureSwipeEnd { event, .. } => self.on_gesture_swipe_end::<B>(event),
            InputEvent::GesturePinchBegin { event, .. } => self.on_gesture_pinch_begin::<B>(event),
            InputEvent::GesturePinchUpdate { event, .. } => self.on_gesture_pinch_update::<B>(event),
            InputEvent::GesturePinchEnd { event, .. } => self.on_gesture_pinch_end::<B>(event),
            InputEvent::GestureHoldBegin { event, .. } => self.on_gesture_hold_begin::<B>(event),
            InputEvent::GestureHoldEnd { event, .. } => self.on_gesture_hold_end::<B>(event),

            InputEvent::TouchDown { event } => self.on_touch_down::<B>(event),
            InputEvent::TouchUp { event } => self.on_touch_up::<B>(event),
            InputEvent::TouchMotion { event } => self.on_touch_motion::<B>(event),
            InputEvent::TouchFrame { event } => self.on_touch_frame::<B>(event),
            InputEvent::TouchCancel { event } => self.on_touch_cancel::<B>(event),

            InputEvent::DeviceAdded { device } => {
                if device.has_capability(DeviceCapability::TabletTool) {
                    self.seat
                        .tablet_seat()
                        .add_tablet::<Self>(dh, &TabletDescriptor::from(&device));
                }
                if device.has_capability(DeviceCapability::Touch) && self.seat.get_touch().is_none() {
                    self.seat.add_touch();
                }
            }
            InputEvent::DeviceRemoved { device } => {
                if device.has_capability(DeviceCapability::TabletTool) {
                    let tablet_seat = self.seat.tablet_seat();

                    tablet_seat.remove_tablet(&TabletDescriptor::from(&device));

                    // If there are no tablets in seat we can remove all tools
                    if tablet_seat.count_tablets() == 0 {
                        tablet_seat.clear_tools();
                    }
                }
            }
            _ => {
                // other events are not handled in anvil (yet)
            }
        }
    }

    fn on_pointer_move<B: InputBackend>(&mut self, _dh: &DisplayHandle, evt: B::PointerMotionEvent) {
        println!("Rezize");
        let mut pointer_location = self.pointer.current_location();
        let serial = SCOUNTER.next_serial();

        let pointer = self.pointer.clone();
        let under = self.surface_under(pointer_location);

        let mut pointer_locked = false;
        let mut pointer_confined = false;
        let mut confine_region = None;
        if let Some((surface, surface_loc)) = under
            .as_ref()
            .and_then(|(target, l)| Some((target.wl_surface()?, l)))
        {
            with_pointer_constraint(&surface, &pointer, |constraint| match constraint {
                Some(constraint) if constraint.is_active() => {
                    // Constraint does not apply if not within region
                    if !constraint
                        .region()
                        .is_none_or(|x| x.contains((pointer_location - *surface_loc).to_i32_round()))
                    {
                        return;
                    }
                    match &*constraint {
                        PointerConstraint::Locked(_locked) => {
                            pointer_locked = true;
                        }
                        PointerConstraint::Confined(confine) => {
                            pointer_confined = true;
                            confine_region = confine.region().cloned();
                        }
                    }
                }
                _ => {}
            });
        }

        pointer.relative_motion(
            self,
            under.clone(),
            &RelativeMotionEvent {
                delta: evt.delta(),
                delta_unaccel: evt.delta_unaccel(),
                utime: evt.time(),
            },
        );

        // If pointer is locked, only emit relative motion
        if pointer_locked {
            pointer.frame(self);
            return;
        }

        pointer_location += evt.delta();

        // clamp to screen limits
        // this event is never generated by winit
        pointer_location = self.clamp_coords(pointer_location);

        let new_under = self.surface_under(pointer_location);

        // If confined, don't move pointer if it would go outside surface or region
        if pointer_confined {
            if let Some((surface, surface_loc)) = &under {
                if new_under.as_ref().and_then(|(under, _)| under.wl_surface()) != surface.wl_surface() {
                    pointer.frame(self);
                    return;
                }
                if let Some(region) = confine_region {
                    if !region.contains((pointer_location - *surface_loc).to_i32_round()) {
                        pointer.frame(self);
                        return;
                    }
                }
            }
        }

        pointer.motion(
            self,
            under,
            &MotionEvent {
                location: pointer_location,
                serial,
                time: evt.time_msec(),
            },
        );
        pointer.frame(self);

        // If pointer is now in a constraint region, activate it
        // TODO Anywhere else pointer is moved needs to do this
        if let Some((under, surface_location)) =
            new_under.and_then(|(target, loc)| Some((target.wl_surface()?.into_owned(), loc)))
        {
            with_pointer_constraint(&under, &pointer, |constraint| match constraint {
                Some(constraint) if !constraint.is_active() => {
                    let point = (pointer_location - surface_location).to_i32_round();
                    if constraint.region().is_none_or(|region| region.contains(point)) {
                        constraint.activate();
                    }
                }
                _ => {}
            });
        }
    }

    fn on_pointer_move_absolute<B: InputBackend>(&mut self,_dh: &DisplayHandle,evt: B::PointerMotionAbsoluteEvent,) {
        let serial = SCOUNTER.next_serial();

        let max_x = self
            .current_workspace().space
            .outputs()
            .fold(0, |acc, o| acc + self.current_workspace().space.output_geometry(o).unwrap().size.w);

        let max_h_output = self
            .current_workspace().space
            .outputs()
            .max_by_key(|o| self.current_workspace().space.output_geometry(o).unwrap().size.h)
            .unwrap();

        let max_y = self.current_workspace().space.output_geometry(max_h_output).unwrap().size.h;

        let mut pointer_location = (evt.x_transformed(max_x), evt.y_transformed(max_y)).into();

        // clamp to screen limits
        pointer_location = self.clamp_coords(pointer_location);

        let pointer = self.pointer.clone();
        let under = self.surface_under(pointer_location);

        pointer.motion(
            self,
            under,
            &MotionEvent {
                location: pointer_location,
                serial,
                time: evt.time_msec(),
            },
        );
        pointer.frame(self);
    }

    fn on_tablet_tool_axis<B: InputBackend>(&mut self, evt: B::TabletToolAxisEvent) {
        let tablet_seat = self.seat.tablet_seat();

        if let Some(pointer_location) = self.touch_location_transformed(&evt) {
            let pointer = self.pointer.clone();
            let under = self.surface_under(pointer_location);
            let tablet = tablet_seat.get_tablet(&TabletDescriptor::from(&evt.device()));
            let tool = tablet_seat.get_tool(&evt.tool());

            pointer.motion(
                self,
                under.clone(),
                &MotionEvent {
                    location: pointer_location,
                    serial: SCOUNTER.next_serial(),
                    time: self.clock.now().as_millis(),
                },
            );

            if let (Some(tablet), Some(tool)) = (tablet, tool) {
                if evt.pressure_has_changed() {
                    tool.pressure(evt.pressure());
                }
                if evt.distance_has_changed() {
                    tool.distance(evt.distance());
                }
                if evt.tilt_has_changed() {
                    tool.tilt(evt.tilt());
                }
                if evt.slider_has_changed() {
                    tool.slider_position(evt.slider_position());
                }
                if evt.rotation_has_changed() {
                    tool.rotation(evt.rotation());
                }
                if evt.wheel_has_changed() {
                    tool.wheel(evt.wheel_delta(), evt.wheel_delta_discrete());
                }

                tool.motion(
                    pointer_location,
                    under.and_then(|(f, loc)| f.wl_surface().map(|s| (s.into_owned(), loc))),
                    &tablet,
                    SCOUNTER.next_serial(),
                    evt.time_msec(),
                );
            }

            pointer.frame(self);
        }
    }

    fn on_tablet_tool_proximity<B: InputBackend>(&mut self,dh: &DisplayHandle,evt: B::TabletToolProximityEvent,) {
        let tablet_seat = self.seat.tablet_seat();

        if let Some(pointer_location) = self.touch_location_transformed(&evt) {
            let tool = evt.tool();
            tablet_seat.add_tool::<Self>(self, dh, &tool);

            let pointer = self.pointer.clone();
            let under = self.surface_under(pointer_location);
            let tablet = tablet_seat.get_tablet(&TabletDescriptor::from(&evt.device()));
            let tool = tablet_seat.get_tool(&tool);

            pointer.motion(
                self,
                under.clone(),
                &MotionEvent {
                    location: pointer_location,
                    serial: SCOUNTER.next_serial(),
                    time: evt.time_msec(),
                },
            );
            pointer.frame(self);

            if let (Some(under), Some(tablet), Some(tool)) = (
                under.and_then(|(f, loc)| f.wl_surface().map(|s| (s.into_owned(), loc))),
                tablet,
                tool,
            ) {
                match evt.state() {
                    ProximityState::In => tool.proximity_in(
                        pointer_location,
                        under,
                        &tablet,
                        SCOUNTER.next_serial(),
                        evt.time_msec(),
                    ),
                    ProximityState::Out => tool.proximity_out(evt.time_msec()),
                }
            }
        }
    }

    fn on_tablet_tool_tip<B: InputBackend>(&mut self, evt: B::TabletToolTipEvent) {
        let tool = self.seat.tablet_seat().get_tool(&evt.tool());

        if let Some(tool) = tool {
            match evt.tip_state() {
                TabletToolTipState::Down => {
                    let serial = SCOUNTER.next_serial();
                    tool.tip_down(serial, evt.time_msec());

                    // change the keyboard focus
                    self.update_keyboard_focus(self.pointer.current_location(), serial);
                }
                TabletToolTipState::Up => {
                    tool.tip_up(evt.time_msec());
                }
            }
        }
    }

    fn on_tablet_button<B: InputBackend>(&mut self, evt: B::TabletToolButtonEvent) {
        let tool = self.seat.tablet_seat().get_tool(&evt.tool());

        if let Some(tool) = tool {
            tool.button(
                evt.button(),
                evt.button_state(),
                SCOUNTER.next_serial(),
                evt.time_msec(),
            );
        }
    }

    fn on_gesture_swipe_begin<B: InputBackend>(&mut self, evt: B::GestureSwipeBeginEvent) {
        let serial = SCOUNTER.next_serial();
        let pointer = self.pointer.clone();
        pointer.gesture_swipe_begin(
            self,
            &GestureSwipeBeginEvent {
                serial,
                time: evt.time_msec(),
                fingers: evt.fingers(),
            },
        );
    }

    fn on_gesture_swipe_update<B: InputBackend>(&mut self, evt: B::GestureSwipeUpdateEvent) {
        let pointer = self.pointer.clone();
        pointer.gesture_swipe_update(
            self,
            &GestureSwipeUpdateEvent {
                time: evt.time_msec(),
                delta: evt.delta(),
            },
        );
    }

    fn on_gesture_swipe_end<B: InputBackend>(&mut self, evt: B::GestureSwipeEndEvent) {
        let serial = SCOUNTER.next_serial();
        let pointer = self.pointer.clone();
        pointer.gesture_swipe_end(
            self,
            &GestureSwipeEndEvent {
                serial,
                time: evt.time_msec(),
                cancelled: evt.cancelled(),
            },
        );
    }

    fn on_gesture_pinch_begin<B: InputBackend>(&mut self, evt: B::GesturePinchBeginEvent) {
        let serial = SCOUNTER.next_serial();
        let pointer = self.pointer.clone();
        pointer.gesture_pinch_begin(
            self,
            &GesturePinchBeginEvent {
                serial,
                time: evt.time_msec(),
                fingers: evt.fingers(),
            },
        );
    }

    fn on_gesture_pinch_update<B: InputBackend>(&mut self, evt: B::GesturePinchUpdateEvent) {
        let pointer = self.pointer.clone();
        pointer.gesture_pinch_update(
            self,
            &GesturePinchUpdateEvent {
                time: evt.time_msec(),
                delta: evt.delta(),
                scale: evt.scale(),
                rotation: evt.rotation(),
            },
        );
    }

    fn on_gesture_pinch_end<B: InputBackend>(&mut self, evt: B::GesturePinchEndEvent) {
        let serial = SCOUNTER.next_serial();
        let pointer = self.pointer.clone();
        pointer.gesture_pinch_end(
            self,
            &GesturePinchEndEvent {
                serial,
                time: evt.time_msec(),
                cancelled: evt.cancelled(),
            },
        );
    }

    fn on_gesture_hold_begin<B: InputBackend>(&mut self, evt: B::GestureHoldBeginEvent) {
        let serial = SCOUNTER.next_serial();
        let pointer = self.pointer.clone();
        pointer.gesture_hold_begin(
            self,
            &GestureHoldBeginEvent {
                serial,
                time: evt.time_msec(),
                fingers: evt.fingers(),
            },
        );
    }

    fn on_gesture_hold_end<B: InputBackend>(&mut self, evt: B::GestureHoldEndEvent) {
        let serial = SCOUNTER.next_serial();
        let pointer = self.pointer.clone();
        pointer.gesture_hold_end(
            self,
            &GestureHoldEndEvent {
                serial,
                time: evt.time_msec(),
                cancelled: evt.cancelled(),
            },
        );
    }

    fn touch_location_transformed<B: InputBackend, E: AbsolutePositionEvent<B>>(&self,evt: &E,) -> Option<Point<f64, Logical>> {
        let output = self
            .current_workspace().space
            .outputs()
            .find(|output| output.name().starts_with("eDP"))
            .or_else(|| self.current_workspace().space.outputs().next());

        let output = output?;
        let output_geometry = self.current_workspace().space.output_geometry(output)?;

        let transform = output.current_transform();
        let size = transform.invert().transform_size(output_geometry.size);
        Some(
            transform.transform_point_in(evt.position_transformed(size), &size.to_f64())
                + output_geometry.loc.to_f64(),
        )
    }

    fn on_touch_down<B: InputBackend>(&mut self, evt: B::TouchDownEvent) {
        let Some(handle) = self.seat.get_touch() else {
            return;
        };

        let Some(touch_location) = self.touch_location_transformed(&evt) else {
            return;
        };

        let serial = SCOUNTER.next_serial();
        self.update_keyboard_focus(touch_location, serial);

        let under = self.surface_under(touch_location);
        handle.down(
            self,
            under,
            &DownEvent {
                slot: evt.slot(),
                location: touch_location,
                serial,
                time: evt.time_msec(),
            },
        );
    }
    fn on_touch_up<B: InputBackend>(&mut self, evt: B::TouchUpEvent) {
        let Some(handle) = self.seat.get_touch() else {
            return;
        };
        let serial = SCOUNTER.next_serial();
        handle.up(
            self,
            &UpEvent {
                slot: evt.slot(),
                serial,
                time: evt.time_msec(),
            },
        )
    }
    fn on_touch_motion<B: InputBackend>(&mut self, evt: B::TouchMotionEvent) {
        let Some(handle) = self.seat.get_touch() else {
            return;
        };
        let Some(touch_location) = self.touch_location_transformed(&evt) else {
            return;
        };

        let under = self.surface_under(touch_location);
        handle.motion(
            self,
            under,
            &smithay::input::touch::MotionEvent {
                slot: evt.slot(),
                location: touch_location,
                time: evt.time_msec(),
            },
        );
    }
    fn on_touch_frame<B: InputBackend>(&mut self, _evt: B::TouchFrameEvent) {
        println!("HOLKJHGFGHJKLÑ");
        let Some(handle) = self.seat.get_touch() else {
            return;
        };
        handle.frame(self);
    }
    fn on_touch_cancel<B: InputBackend>(&mut self, _evt: B::TouchCancelEvent) {
        let Some(handle) = self.seat.get_touch() else {
            return;
        };
        handle.cancel(self);
    }

    fn clamp_coords(&self, pos: Point<f64, Logical>) -> Point<f64, Logical> {
        if self.current_workspace().space.outputs().next().is_none() {
            return pos;
        }

        let (pos_x, pos_y) = pos.into();
        let max_x = self
            .current_workspace().space
            .outputs()
            .fold(0, |acc, o| acc + self.current_workspace().space.output_geometry(o).unwrap().size.w);
        let clamped_x = pos_x.clamp(0.0, max_x as f64);
        let max_y = self
            .current_workspace().space
            .outputs()
            .find(|o| {
                let geo = self.current_workspace().space.output_geometry(o).unwrap();
                geo.contains((clamped_x as i32, 0))
            })
            .map(|o| self.current_workspace().space.output_geometry(o).unwrap().size.h);

        if let Some(max_y) = max_y {
            let clamped_y = pos_y.clamp(0.0, max_y as f64);
            (clamped_x, clamped_y).into()
        } else {
            (clamped_x, pos_y).into()
        }
    }
}


impl AnvilState<UdevData> {
    pub fn process_dock_handler(&mut self, msg: IpcMessage){
        println!("WM    {:?}", msg);
        match msg.category().as_str() {
            "Workspace" =>self.process_dock_handler_workspace(msg),
            "System" =>self.process_dock_handler_system(msg),
            "Layout" =>self.process_dock_handler_layout(msg),
            "Panel:Home" =>self.process_dock_handler_pannel_home(msg),
            "Panel:Apps" =>self.process_dock_handler_pannel_apps(msg),
            "Panel:Network" =>self.process_dock_handler_pannel_network(msg),
            "Panel:Music" =>self.process_dock_handler_pannel_music(msg),

            _ => {
                let _ = println!("Categoria desconocido: [{}]",msg.category());
            }
        }
    }

    pub fn process_dock_handler_system(&mut self, msg: IpcMessage){
        match msg.name().as_str() {
            "Start Dock"=>{
                self.battery_update_sender();
                self.layout_set_sender();
                self.workspace_update_sender();
                self.pannel_load_sender();
            }
            "Poweroff" => spawn("poweroff"),
            "Reboot"=>spawn("reboot"),
            "Open Panel"=>{
                if !self.dock.is_open(){
                    self.dock.open();
                    self.current_workspace_arrange();
                }
            }
            "Close Panel"=>{
                 if self.dock.is_open(){
                    self.dock.closed();
                    self.current_workspace_arrange();
                }
            }
            "Set Volume"=>{
                if let Some(volume_value) = msg.data().get("Volume") {
                    if let Some(volume) = volume_value.as_u64() {
                        Volume::set(volume as u8);
                    }
                }
            },
            "Set Glow"=>{
                if let Some(glow_value) = msg.data().get("Glow") {
                    if let Some(glow) = glow_value.as_u64() {
                        Brightness::set(glow as u8);
                    }
                }
            },
            "Log Out"=>{
                println!("WM Socket: Log Out");
            },
            

            "Auth"=>{
                println!("Auth");
            //     if let Some(password) = msg.get_data().get("password") {
            //         notifier.send(CustomEvent::LockAuth(password.to_string()));
            //     }
            }
            "Start Lock"=>{
                

                println!("Start Lock");
                // notifier.send(CustomEvent::DockLock());
            }
            _ => {
                
                let _ = println!("Nombre desconocido: [{}:{}]",
                    msg.category(),
                    msg.name());
            }
        }
    }   // 3

    pub fn process_dock_handler_workspace(&mut self, msg: IpcMessage){
        match msg.name().as_str() {
            "Set" => {
                if let Some(space_value) = msg.data().get("space") {
                    if let Some(space) = space_value.as_i64() {
                        if let Ok(space_u8) = u8::try_from(space +1) {
                            self.switch_workspace(space_u8.into());
                        }
                    }
                }
            }
            _ => info!("Nombre desconocido: [{}:{}]",msg.category(),msg.name()),
        }
    }   // 0
    
    pub fn process_dock_handler_layout(&mut self, msg: IpcMessage){
         match msg.name().as_str() {
            "Toggle"=>self.toggle_layout(),
            _ => info!("Nombre desconocido: [{}:{}]",msg.category(),msg.name()),
        }
    }   // 0

    pub fn process_dock_handler_pannel_home(&mut self, msg: IpcMessage){
         match msg.name().as_str() {
            "Open" => {
                let system_usage = SystemUsage::get();
                self.stats_home_pannel_sender(system_usage);
                self.volume_system_sender(Volume::get());
                self.glow_system_sender(Brightness::get());
                self.google_daily_load();
                //                 notifier_clone.send(CustomEvent::HomePanelLoadWeather());

            }
            "Google:Diary:Refresh" => self.google_daily_load(),

            "Google:Oauth:Code" => {
                let code = msg.data()["code"].as_str().unwrap().to_string();
                let punk_ipc = self.punk_ipc.clone();
                if let Some(google) = self.google.clone() {
                    tokio::spawn(async move {
                        let mut google = google.lock().await;
                        if !google.is_authenticated(){ 
                            let _ = google.exchange_code_for_token(&code).await;
                            let daily = google.get_daily().await;
                            let msg = IpcMessage::new(Some(IpcMode::Bridge),"Panel:Home","Google:Daily", json!({"events": daily}));
                            punk_ipc.send(msg);    
                        }
                        
                    });
                }
            }

            _ => println!("Nombre desconocido: [{}:{}]",msg.category(),msg.name()),
            
        }
    }   //  1

    pub fn process_dock_handler_pannel_apps(&mut self, msg: IpcMessage){
        match msg.name().as_str() {
            "Open" =>self.apps_load_sender(),
            "Search" => {
                let q = msg.data()["q"].as_str().unwrap().to_string();
                self.apps_search_sender(&q);
            }
            "Load Apps" => self.apps_load_sender(),
            "Open App" => {
                let package = msg.data()["package"].as_str().unwrap().to_string();
                let _ = self.apps_manager.launch(&package);
            }
            _ => println!("Nombre desconocido: [{}:{}]",msg.category(),msg.name()),
        }
    }

    pub fn process_dock_handler_pannel_network(&mut self, msg: IpcMessage){
         match msg.name().as_str() {
            // "Open" => {
            //     notifier.send(CustomEvent::OpenNetworkPanel());
            // }
            // "Refresh" => {
            //     notifier.send(CustomEvent::NetworkPanelLoadWiFi());
            // }
            // "Connect Public WiFi" => {
            //     let ssid = msg.get_data()["SSID"].as_str().unwrap().to_string();
            //     notifier.send(CustomEvent::NetworkPanelConnectWiFiPublic(ssid));
            // }
            // "Disconnect WiFi" => {
            //     notifier.send(CustomEvent::NetworkPanelDisconnectWiFi());
            // }
            // "Connect WiFi" => {
            //     let ssid = msg.get_data()["SSID"].as_str().unwrap().to_string();
            //     let password = msg.get_data()["Password"].as_str().unwrap().to_string();
            //     notifier.send(CustomEvent::NetworkPanelConnectWiFi(ssid, password));
            // }
            // "Share WiFi" => {
            //     notifier.send(CustomEvent::NetworkPanelShareWiFi());
            // }
            _ => {
            
                println!("Nombre desconocido: [{}:{}]",
                    msg.category(),
                    msg.name());
            }
     }
    }

    pub fn process_dock_handler_pannel_music(&mut self, msg: IpcMessage){
        match msg.name().as_str() {
            // "Local:Load:Song" => {
            //     notifier.send(CustomEvent::SongsLocalLoad());
            // }
            // "Local:Start:Song" => {
            //     let path = msg.get_data()["path"].as_str().unwrap().to_string();
            //     notifier.send(LocalAudioCommand::Load(path));
            // }
            // "Local:Play:Song" => {
            //     notifier.send(LocalAudioCommand::Play());
            // }
            // "Local:Pause:Song" => {
            //     notifier.send(LocalAudioCommand::Pause());
            // }
            // "Local:Reset:Song" => {
            //     notifier.send(LocalAudioCommand::Reset());
            // }
            // "Local:Stop:Song" => {
            //     notifier.send(LocalAudioCommand::Stop());
            // }
            // "Local:Search:Song" => {
            //     let q = msg.get_data()["q"].as_str().unwrap().to_string();
            //     notifier.send(CustomEvent::SongsLocalSearch(q));
            // }
        
            _ => {
                println!("Nombre desconocido: [{}:{}]",
                    msg.category(),
                    msg.name());
            }
        }
    }

    fn google_daily_load(&mut self) {
        let punk_ipc = self.punk_ipc.clone();
        if let Some(google) = self.google.clone() {
            tokio::spawn(async move {
                let mut google = google.lock().await;
                if !google.is_authenticated() {
                    let msg = IpcMessage::new(Some(IpcMode::Bridge),"Panel:Home","Google:Oauth:url", json!({"Url": google.auth_url()}));
                    punk_ipc.send(msg);
                    return;
                }

                let daily = google.get_daily().await;
                let msg = IpcMessage::new(Some(IpcMode::Bridge),"Panel:Home","Google:Daily", json!({"events": daily}));
                punk_ipc.send(msg);    
            });
        }
    }

}




/// Possible results of a keyboard action
#[allow(dead_code)] // some of these are only read if udev is enabled
#[derive(Debug, Clone, PartialEq)]
enum KeyAction {
    /// Quit the compositor
    Quit,
    /// Trigger a vt-switch
    VtSwitch(i32),
    /// run a command
    Run(String),
    /// Switch the current screen
    Screen(usize),
    ScaleUp,
    ScaleDown,
    TogglePreview,
    RotateOutput,
    ToggleTint,
    ToggleDecorations,
    // ToggleWorkspace(usize),
    MoveAndSwitchWorkspaceTo1,
    MoveAndSwitchWorkspaceTo2,
    MoveAndSwitchWorkspaceTo3,
    MoveAndSwitchWorkspaceTo4,
    MoveAndSwitchWorkspaceTo5,
    MoveAndSwitchWorkspaceTo6,
    MoveAndSwitchWorkspaceTo7,
    MoveAndSwitchWorkspaceTo8,
    MoveAndSwitchWorkspaceTo9,
    MoveToWorksace1,
    MoveToWorksace2,
    MoveToWorksace3,
    MoveToWorksace4,
    MoveToWorksace5,
    MoveToWorksace6,
    MoveToWorksace7,
    MoveToWorksace8,
    MoveToWorksace9,
    SwitchWorkspaceTo1,
    SwitchWorkspaceTo2,
    SwitchWorkspaceTo3,
    SwitchWorkspaceTo4,
    SwitchWorkspaceTo5,
    SwitchWorkspaceTo6,
    SwitchWorkspaceTo7,
    SwitchWorkspaceTo8,
    SwitchWorkspaceTo9,

    FocusNext,
    FocusPrevious,
    ToggleLayout,
    WindowClose,
    WindowPrevious,
    WindowNext,
    OpenApp(String),
    Test,
    /// Do nothing more
    None,
}






impl KeyAction {
    fn from_str(s: &str) -> Self{
        match s {
            "SwitchWorkspaceTo1" => KeyAction::SwitchWorkspaceTo1,
            "SwitchWorkspaceTo2" => KeyAction::SwitchWorkspaceTo2,
            "SwitchWorkspaceTo3" => KeyAction::SwitchWorkspaceTo3,
            "SwitchWorkspaceTo4" => KeyAction::SwitchWorkspaceTo4,
            "SwitchWorkspaceTo5" => KeyAction::SwitchWorkspaceTo5,
            "SwitchWorkspaceTo6" => KeyAction::SwitchWorkspaceTo6,
            "SwitchWorkspaceTo7" => KeyAction::SwitchWorkspaceTo7,
            "SwitchWorkspaceTo8" => KeyAction::SwitchWorkspaceTo8,
            "SwitchWorkspaceTo9" => KeyAction::SwitchWorkspaceTo9,
            "MoveToWorksace1" => KeyAction::MoveToWorksace1,
            "MoveToWorksace2" => KeyAction::MoveToWorksace2,
            "MoveToWorksace3" => KeyAction::MoveToWorksace3,
            "MoveToWorksace4" => KeyAction::MoveToWorksace4,
            "MoveToWorksace5" => KeyAction::MoveToWorksace5,
            "MoveToWorksace6" => KeyAction::MoveToWorksace6,
            "MoveToWorksace7" => KeyAction::MoveToWorksace7,
            "MoveToWorksace8" => KeyAction::MoveToWorksace8,
            "MoveToWorksace9" => KeyAction::MoveToWorksace9,
            "MoveAndSwitchWorkspaceTo1" => KeyAction::MoveAndSwitchWorkspaceTo1,
            "MoveAndSwitchWorkspaceTo2" => KeyAction::MoveAndSwitchWorkspaceTo2,
            "MoveAndSwitchWorkspaceTo3" => KeyAction::MoveAndSwitchWorkspaceTo3,
            "MoveAndSwitchWorkspaceTo4" => KeyAction::MoveAndSwitchWorkspaceTo4,
            "MoveAndSwitchWorkspaceTo5" => KeyAction::MoveAndSwitchWorkspaceTo5,
            "MoveAndSwitchWorkspaceTo6" => KeyAction::MoveAndSwitchWorkspaceTo6,
            "MoveAndSwitchWorkspaceTo7" => KeyAction::MoveAndSwitchWorkspaceTo7,
            "MoveAndSwitchWorkspaceTo8" => KeyAction::MoveAndSwitchWorkspaceTo8,
            "MoveAndSwitchWorkspaceTo9" => KeyAction::MoveAndSwitchWorkspaceTo9,
            "ToggleLayout" => KeyAction::ToggleLayout,
            "WindowClose" => KeyAction::WindowClose,
            "WindowPrevious" => KeyAction::WindowPrevious,
            "WindowNext" => KeyAction::WindowNext,
            "FocusPrevious" => KeyAction::FocusPrevious,
            "FocusNext" => KeyAction::FocusNext,
            "Quit" => KeyAction::Quit,
            "Test" => KeyAction::Test,

            _ if s.starts_with("Open:") => {
                KeyAction::OpenApp(
                    s["Open:".len()..].to_string()
                )
            },

            _ => KeyAction::None,
        }
    }
}





#[derive(Debug,Clone)]
pub struct ShortcutManager{
    shortcuts: HashMap<String, KeyAction>
}





impl ShortcutManager{
    pub fn new()->Self{
        let cfg = GLOBAL_CFG.read().expect("No se pudo bloquear para lectura");
        let mut shortcuts: HashMap<String, KeyAction> = HashMap::new();
        let keybindings = cfg.keybindings().get_keybindings();
         for (key_action, shortcut) in keybindings.iter() {
            let upper = shortcut.to_uppercase();
            let mut shortcuts_list: Vec<_> = upper.split('+').collect();
            let key = match shortcuts_list.pop().unwrap() {
                "RETURN" => xkb::KEY_Return,
                "BACKSPACE" => xkb::KEY_BackSpace,
                "SPACE"     => xkb::KEY_space,
                "TAB"       => xkb::KEY_Tab,
                "ESCAPE"    => xkb::KEY_Escape,
                "DELETE"    => xkb::KEY_Delete,
                "LEFT"      => xkb::KEY_Left,
                "RIGHT"     => xkb::KEY_Right,
                "UP"        => xkb::KEY_Up,
                "DOWN"      => xkb::KEY_Down,
                other => other_xkb::keysym_from_name(other, other_xkb::KEYSYM_NO_FLAGS).into(),
            };
            let atajo = format!(
                "{}{}{}{}{}",
                shortcuts_list.contains(&"CTRL") as u32,
                shortcuts_list.contains(&"SUPER") as u32,
                shortcuts_list.contains(&"ALT") as u32,
                shortcuts_list.contains(&"SHIFT") as u32,
                key,
            );
            shortcuts.insert(atajo ,KeyAction::from_str(&key_action));
         }
        Self{shortcuts}
    }

    fn get(&self, shortcut: String) -> Option<KeyAction> {
        Some(self.shortcuts
            .get(&shortcut)
            .cloned()
            .unwrap_or(KeyAction::None))
    }


}

#![allow(dead_code)]
#![cfg_attr(debug_assertions, deny(missing_debug_implementations))]

use rust_gui_core::data::{Color, Position};
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow},
    window::{WindowAttributes, WindowId}
};
use rust_gui_core::{
    ApiVersion, ApplicationInfo,
    render::{RenderContext, Size},
    data::shape::Rectangle
};
use std::{
    collections::HashMap,
    ffi::CStr,
    rc::Rc,
};


pub mod event;
use event::*;
pub mod node;
mod window;
use window::RenderWindow;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Application {
    info:ApplicationInfo<'static>,
    ctx: Option<Rc<RenderContext>>,
    list: HashMap<WindowId, RenderWindow>,
    listeners: event::EventTargetCore
}

impl Application {
    pub fn new() -> Self {
        Self {
            info: ApplicationInfo::default(),
            ctx: None,
            list: HashMap::new(),
            listeners: event::EventTargetInner::new()
        }
    }

    pub fn set_name(&mut self, app_name:&'static CStr) {
        self.info.name = app_name;
    }

    pub fn get_name(&self)-> &CStr {
        self.info.name
    }

    pub fn set_version<Api:ApiVersion>(&mut self, app_version:Api) {
        self.info.api_version = app_version.as_api();
    }

    pub fn get_version(&self) -> u32 {
        self.info.api_version
    }

    pub fn run(&mut self) -> Result<(), EventLoopError>{
        let event_loop = EventLoop::new()?;

        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run_app(self)
    }

    pub fn window(&self, id:&WindowId) -> Option<&RenderWindow> {
        self.list.get(id)
    }

    pub fn window_mut(&mut self, id:&WindowId) -> Option<&mut RenderWindow> {
        self.list.get_mut(id)
    }

    pub fn open_windows(&self) -> usize {
        self.list.len()
    }

    pub(crate) fn handle_winit_event(&mut self, event:impl ExternalEvent, window_id:WindowId) {
        let mut event = if let Some(window) = self.window_mut(&window_id) {
            match window.handle_winit_event(event) {
                Ok(close) => {
                    if close && let Some(w) = self.list.remove(&window_id) {
                        // Destroy && Drop window
                        w.destory();
                    }

                    return;
                },
                Err(e) => e
            }
        } else {
            event.into_event(&mut EventHistory::new())
        };

        if let Err(msg) = self.dispatch_event(&mut event) {
            panic!("{}", msg)
        }

        if let Some(app_event) = event.get_actionable::<ApplicationEvent>().ok().flatten() {
            /*match app_event {
                
            }*/

            println!("Application Handler: {:?}", app_event);
        }
    }
}

impl event::EventTarget for Application {
    fn dispatch_event(&self, event:&mut Event) -> Result<(), impl std::fmt::Display> {
        self.listeners.dispatch_event(event)
    }

    fn add_event_listener(&mut self, type_name:&str, listener:EventListener) -> usize {
        self.listeners.add_event_listener(type_name, listener)
    }

    fn add_event_listener_once(&mut self, type_name:&str, listener:EventListener) -> usize {
        self.listeners.add_event_listener_once(type_name, listener)
    }

    fn remove_event_listener(&mut self, id:usize) -> Option<EventListener> {
        self.listeners.remove_event_listener(id)
    }
}

impl event::ParentEventTarget for Application {
    fn inner_ref(&self) -> EventTargetCore {
        self.listeners.inner_ref()
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(_ctx) = &mut self.ctx {
            println!("TODO: Create new window in engine!")
        } else {
            let window = event_loop.create_window(WindowAttributes::default())
                .unwrap();

            let ctx = RenderContext::new(&self.info, &window).unwrap();
            let id = window.id();
            let mut target = RenderWindow::from(window, &ctx, self).unwrap();

            self.ctx = Some(ctx);

            target.draw(Rectangle{
                color: Color::RED,
                pos: Position::new_coordinate(10, 10),
                size: Size{
                    width: 50.0,
                    height: 50.0
                }
            });
            
            target.request_redraw();
            self.list.insert(id, target);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id:WindowId, event:WinitEvent) {
        self.handle_winit_event(event, window_id);
        if self.open_windows() == 0 {
            event_loop.exit();
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, device_id: DeviceId, event: winit::event::DeviceEvent){
        println!("Device Event: {:?}\n{:?}", device_id, event)
    }

    fn exiting(&mut self, _:&ActiveEventLoop) {
        let mut drain = self.list.drain();
        while let Some((_, window)) = drain.next() {
            window.destory();
        }
    }
}
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{WindowAttributes, WindowId}
};
use rust_gui_core::{
    ApiVersion, ApplicationInfo,
    render::RenderContext
};
use std::{
    collections::LinkedList,
    ffi::CStr,
    rc::Rc,
};


mod event;
pub use event::*;
mod window;
use window::RenderWindow;

const DEFAULT_APP_NAME:&'static CStr = c"";


pub struct Application {
    info:ApplicationInfo<'static>,
    ctx: Option<Rc<RenderContext>>,
    list: LinkedList<RenderWindow>,
    listeners: GlobalEventTarget
}

impl Application {
    pub fn new() -> Self {
        Self {
            info: ApplicationInfo::default(),
            ctx: None,
            list: LinkedList::new(),
            listeners: GlobalEventTarget::new()
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
        event_loop.run_app(self)
    }

    pub fn get_by_id(&self, id:WindowId) -> Option<&RenderWindow> {
        for target in &self.list {
            if target.id() == id {
                return Some(target)
            }
        }

        None
    }

    pub fn get_by_id_mut(&mut self, id:WindowId) -> Option<&mut RenderWindow> {
        for target in &mut self.list {
            if target.id() == id {
                return Some(target)
            }
        }

        None
    }

    pub fn primary_window(&self) -> Option<&RenderWindow> {
        self.list.iter().next()
    }

    pub fn primary_window_mut(&mut self) -> Option<&mut RenderWindow> {
        self.list.iter_mut().next()
    }

    pub fn append_event_listener(&self, event_listener:GlobalEventListener) {
        self.listeners.append_event_listener(event_listener);
    }

    pub fn dispatch_event(&self, event:GlobalEvent) {
        self.listeners.dispatch_event(event);
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
            self.ctx = Some(ctx);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id:WindowId, event:WinitEvent) {
        if let Some(target) = self.get_by_id_mut(window_id) {
            target.dispatch_event(event, Some(event_loop));
        }
        
        if event == WinitEvent::CloseRequested {
            event_loop.exit();
        }
    }

    fn exiting(&mut self, _:&ActiveEventLoop) {
        if let Some(_ctx) = self.ctx.take() {
            //ctx.destory();
        }
    }
}
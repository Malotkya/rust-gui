use rust_gui_core::{
    render::{
        RenderTarget,
        RenderContext,
        VertexShape,
        RenderError
    },
    data::Color
};
use winit::{
    event_loop::ActiveEventLoop, window::{Window, WindowAttributes, WindowId}
};
use super::event::*;
use std::{
    sync::Arc,
    rc::Rc,
    any::Any,
    collections::LinkedList
};

pub struct RenderWindow {
    window: Arc<Window>,
    target: RenderTarget,
    listeners: LinkedList<WindowEventListener>,
    global: GlobalEventTarget,
    shapes: Vec<VertexShape>,
    clear:Option<Color>
}

impl RenderWindow {
    pub(crate) fn from(window:Window, ctx:&Rc<RenderContext>, global:&GlobalEventTarget) -> Result<Self, RenderError> {
        let window = Arc::new(window);
        Ok(Self {
            target: ctx.create_target(&window)?,
            window,
            listeners: LinkedList::new(),
            global: global.clone(),
            shapes: Vec::new(),
            clear: None
        })
    }

    pub(crate) fn new(event_loop:&ActiveEventLoop, ctx:&Rc<RenderContext>, global:&GlobalEventTarget) -> Result<Self, RenderError> {
        let window = Arc::new(
            event_loop.create_window(WindowAttributes::default())?
        );
        let target = ctx.create_target(&window)?;

        Ok(Self {
            window, target,
            listeners: LinkedList::new(),
            global: global.clone(),
            shapes: Vec::new(),
            clear: None
        })
    }

    pub fn id(&self) -> WindowId {
        self.window.id()
    }

    pub fn add_event_listner<T: Into<WindowEventListener>>(&mut self, listener:T) {
        self.listeners.push_back(listener.into());
    }

    pub fn draw<T:Into<VertexShape>>(&mut self, shape:T) {
        self.shapes.push(shape.into())
    }

    pub fn clear<C:Into<Color>>(&mut self, color:C) {
        self.clear = Some(color.into());
    }

    pub fn render(&mut self) -> Result<(), RenderError>{
        let clear_color = self.clear.take()
            .unwrap_or(Color::BLACK);
        self.target.draw(clear_color.float32(), &self.shapes)?;

        self.shapes.clear();

        Ok(())
    }

    fn dispatch_window_event(&mut self, event:&WinitEvent, event_loop:&ActiveEventLoop) -> Result<u32, RenderError> {
        let mut mouse_pos = MouseHistory::None;
        let mut touch_pos = MouseHistory::None;

        if *event == WinitEvent::RedrawRequested {
            self.render()?;
        } else {

            let mut count = 0;
            for listener in &self.listeners {
                if listener.match_call(event, event_loop, &mut mouse_pos, &mut touch_pos) {
                    count += 1
                }
            }

            return Ok(count);
        }
        
        return Ok(0);
    }

    pub fn dispatch_event<T: Into<WindowEvent>>(&self, event:T, event_loop:Option<&ActiveEventLoop>) -> u32 {
        let event = event.into();

        let mut count = 0;
        for listener in &self.listeners {
            if listener.event_call(&event, event_loop) {
                count += 1
            }
        }

        return count;
    }

    pub fn dispatch_custom_event<S:ToString>(&self, name:S, listener:EventListener<Rc<dyn Any>>, event_loop:Option<&ActiveEventLoop>) -> u32 {
        let event = WindowEvent::custom_event(name, listener);

        let mut count = 0;
        for listener in &self.listeners {
            if listener.event_call(&event, event_loop) {
                count += 1
            }
        }

        return count;
    }
}
use rust_gui_core::{
    render::{
        RenderTarget,
        RenderContext,
        VertexData,
        VertexShape,
        RenderError
    },
    data::Color
};
use winit::{
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes}
};
use super::event::*;
use std::{
    sync::Arc,
    rc::Rc,
    ops::Deref
};

pub struct RenderWindow {
    inner: Arc<Window>,
    target: RenderTarget,
    listeners: WindowEventTarget,
    shapes: Vec<VertexData>,
    clear:Option<Color>,
    mouse_history: EventHistory,
    touch_history: EventHistory
}

impl RenderWindow {
    pub(crate) fn from(window:Window, ctx:&Rc<RenderContext>, global_target:&impl ParentEventTarget) -> Result<Self, RenderError> {
        let inner = Arc::new(window);
        Ok(Self {
            target: ctx.create_target(&inner)?,
            inner,
            listeners: WindowEventTarget::new_parrent(global_target),
            shapes: Vec::new(),
            clear: None,
            mouse_history: EventHistory::new(),
            touch_history: EventHistory::new()
        })
    }

    pub(crate) fn new(event_loop:&ActiveEventLoop, ctx:&Rc<RenderContext>, global_target:&impl ParentEventTarget) -> Result<Self, RenderError> {
        let inner = Arc::new(
            event_loop.create_window(WindowAttributes::default())?
        );
        let target = ctx.create_target(&inner)?;

        Ok(Self {
            inner, target,
            listeners: WindowEventTarget::new_parrent(global_target),
            shapes: Vec::new(),
            clear: None,
            mouse_history: EventHistory::new(),
            touch_history: EventHistory::new()
        })
    }

    pub fn add_event_listner(&mut self, type_name:&str, listener:EventListener) -> usize{
        self.listeners.add_event_listener(type_name, listener)
    }

    pub fn draw<T:VertexShape>(&mut self, shape:T) {
        let data = VertexData{
            color: shape.color(),
            topology: shape.topology(),
            positions: shape.positions(&self.target.size())
        };
        self.shapes.push(data)
    }

    pub fn clear<C:Into<Color>>(&mut self, color:Option<C>) {
        self.clear = color.map(|c|c.into());
        self.shapes.clear();
    }

    pub fn render(&mut self) -> Result<(), RenderError>{
        let clear_color = self.clear.clone()
            .unwrap_or(Color::BLACK);
        
        self.target.draw(clear_color.float32(), &self.shapes)?;

        Ok(())
    }

    /// Handle WinitEvent
    /// 
    /// Err(event) ErrorEvent should be handled by application layer
    /// Ok(true) close window
    /// Ok(false) keep window open
    pub(crate) fn handle_winit_event(&mut self, event:impl ExternalEvent) -> Result<bool, Event> {
        let resp = match self.listeners.handle_external_event(event) {
            EventResponse::ElementEvent(e)
                => self.handle_element_event(e),
            EventResponse::WindowEvent(mut e)
                => if let Err(msg) = self.listeners.dispatch_event(&mut e) {
                    Err(Event::new("error", format!("{}", msg)))
                } else {
                    e.get_actionable()
                },
            EventResponse::ApplicationEvent(e) | EventResponse::Error(e)
                => Err(e)
        };

        if let Some(event) = resp? {
            match event {
                WindowEvent::RedrawRequested => if let Err(e) = self.render() {
                       Err(Event::new("error", format!("{}", e))) 
                } else {
                    Ok(false)
                },
                WindowEvent::SurfaceResized(new_size) => {
                    todo!("Update swapchain {:?}", new_size)
                },
                WindowEvent::MouseWheel(data) => {
                    println!("Scrolling!\n{:?}", data);
                    Ok(false)
                },
                WindowEvent::CloseRequested => Ok(true),
                _ => Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    fn handle_element_event(&self, event:Event) -> Result<Option<WindowEvent>, Event> {
        println!("Event: {}", event);
        todo!("Handle bubbling/Finding Internal Events")
    }

    pub fn destory(mut self) {
        //SAFETY: This is the last time the window is used.
        unsafe { self.target.destory() };
    }
}

impl EventTarget for RenderWindow {
    fn add_event_listener(&mut self, type_name:&str, listener:EventListener) -> usize {
        self.listeners.add_event_listener(type_name, listener)
    }

    fn add_event_listener_once(&mut self, type_name:&str, listener:EventListener) -> usize {
        self.listeners.add_event_listener_once(type_name, listener)
    }

    fn remove_event_listener(&mut self, id:usize) -> Option<EventListener> {
        self.listeners.remove_event_listener(id)
    }

    fn dispatch_event(&self, event:&mut Event) -> Result<(), impl std::fmt::Display> {
        self.listeners.dispatch_event(event)
    }
}

impl ParentEventTarget for RenderWindow {
    fn inner_ref(&self) -> EventTargetCore {
        self.listeners.inner_ref()
    }
}

impl Deref for RenderWindow {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
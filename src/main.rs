use rust_gui_application::Application;


fn main() {
    let mut app = Application::new();
    app.set_name(c"Hello World!");
    

    app.run().unwrap();
}

use druid::{
    AppLauncher,
    WidgetExt,
    Widget,
    Env,
    UpdateCtx,
    WindowDesc,
    widget::TextBox,
    widget::Controller
};

struct UpdateCallback();

impl Controller<String, TextBox<String>> for UpdateCallback {
    fn update(&mut self, 
        child: &mut TextBox<String>, 
        ctx: &mut UpdateCtx<'_, '_>, 
        old_data: &String, 
        data: &String, 
        env: &Env
    ) {
        if old_data != data {
            // the data has changed, you can call your function here
            println!("{}", data);
        }
        // also inform the child that the data has changed
        child.update(ctx, old_data, data, env)
    }
}

fn build_root_widget() -> impl Widget<String> {
    TextBox::new().controller(UpdateCallback())
}

fn main() {
    AppLauncher::with_window(WindowDesc::new(build_root_widget())).launch("Test".to_string()).unwrap();
}

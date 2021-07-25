use fltk::{prelude::*, *};
use fltk_theme::{ColorTheme, color_themes};
use fltk_theme::{widget_themes, WidgetTheme, ThemeType};
mod ui;

fn main() {
    let app = app::App::default();
    //let app = app::App::default().with_scheme(app::Scheme::Base );

    //let theme = ColorTheme::from_colormap(color_themes::DARK_THEME);
    //theme.apply();
    
    let widget_theme = WidgetTheme::new(ThemeType::Aero);
    widget_theme.apply();
    
    let mut ui = ui::UserInterface::make_window();
   
    ui.button_input_file.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    ui.button_output.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    ui.button_run.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);

    ui.button_input_file.set_callback(move |_| {
        println!("Works!");
    });
    
    app.run().unwrap();
}

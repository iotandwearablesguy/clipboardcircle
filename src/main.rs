use gdk::Display;
use glib::clone;
#[allow(unused_imports)]
use gtk::{
 CssProvider, StyleContext, gio, glib, Application, ApplicationWindow, Button, Label, prelude::*, gdk,
};
fn main() {
    
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.clipboard"),
        Default::default(),
    );
       
    application.connect_startup(|_| load_css());
    application.connect_activate(build_ui);
    application.run();
}
/// This fn *is* [`load_css`]!
fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Calculations of a Circle")
        .default_width(400)
        .default_height(300)
        .build();

    let display = gdk::Display::default().unwrap();
    let clipboard = display.clipboard();

    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(24)
        .build();

    // The text copy/paste part
    //let outputcir =format!("Distance between {} and {} on the surface of Earth is  {:.1} miles.", city1, city2, miles_distance );
    let title = gtk::Label::builder()
        //.label("Radius                                                          Circumference")
        .label("Circumference of a Circle")
        .halign(gtk::Align::Start)
        .build();
    title.add_css_class("title-2");
    container.append(&title);

    let text_container = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .orientation(gtk::Orientation::Horizontal)
        .spacing(24)
        .build();

    let from_entry = gtk::Entry::builder()
        .placeholder_text("Radius")
        .build();
    text_container.append(&from_entry);

    let copy_btn = gtk::Button::with_label("Enter");
    copy_btn.connect_clicked(clone!(@weak clipboard, @weak from_entry => move |_btn| {
        let text = from_entry.text();
        let radius: f32 = text.trim().parse::<f32>().unwrap();
        let circum_circle: f32 = radius * 2. * 3.1415;
        let circumanswer = circum_circle.to_string();
        clipboard.set_text(&circumanswer);
    }));
      copy_btn.set_widget_name("button_enter");
    text_container.append(&copy_btn);

    let into_entry = gtk::Entry::new();
    text_container.append(&into_entry);

    let paste_btn = gtk::Button::with_label("Result");
    paste_btn.connect_clicked(clone!(@weak clipboard, @weak into_entry => move |_btn| {
        clipboard.read_text_async(gio::Cancellable::NONE, clone!(@weak into_entry => move|res| {
            if let Ok(Some(circumanswer)) = res {
                into_entry.set_text(&circumanswer);
            }
        }));
    }));
      paste_btn.set_widget_name("button_results");
    text_container.append(&paste_btn);
    container.append(&text_container);
//
//
let title3 = gtk::Label::builder()
     //   .label("Radius                                                           Area")
    .label("Area of a Circle")
        .halign(gtk::Align::Start)
        .build();
    title3.add_css_class("title-2");
    container.append(&title3);

    let text_container3 = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .orientation(gtk::Orientation::Horizontal)
        .spacing(24)
        .build();

    let from_entry3 = gtk::Entry::builder()
        .placeholder_text("Enter the radius of Circle")
        .build();
    text_container3.append(&from_entry3);

    let copy_btn3 = gtk::Button::with_label("Enter");
    copy_btn3.connect_clicked(clone!(@weak clipboard, @weak from_entry => move |_btn| {
        let text3 = from_entry3.text();
        let radius: f32 = text3.trim().parse::<f32>().unwrap();
        let areacircle: f32 = radius * radius * 3.1415;
        let area_answer = areacircle.to_string();
        clipboard.set_text(&area_answer);
    }));
      copy_btn3.set_widget_name("button_enter");
    text_container3.append(&copy_btn3);

    let into_entry = gtk::Entry::new();
    text_container3.append(&into_entry);

    let paste_btn3 = gtk::Button::with_label("Result");
    paste_btn3.connect_clicked(clone!(@weak clipboard, @weak into_entry => move |_btn| {
        clipboard.read_text_async(gio::Cancellable::NONE, clone!(@weak into_entry => move|res| {
            if let Ok(Some(area_answer)) = res {
                into_entry.set_text(&area_answer);
            }
        }));
    }));
      paste_btn3.set_widget_name("button_results");
    text_container3.append(&paste_btn3);
    container.append(&text_container3);
//
//
    let title2 = gtk::Label::builder()
       // .label("Text Entry                                                      Added")
        .label("Text Adder Example").halign(gtk::Align::Start)
        .build();
    title2.add_css_class("title-2");
    container.append(&title2);

    let text_container2 = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .orientation(gtk::Orientation::Horizontal)
        .spacing(24)
        .build();

    let from_entry2 = gtk::Entry::builder()
        .placeholder_text("Text Adder")
        .build();
    text_container2.append(&from_entry2);

    let copy_btn2 = gtk::Button::with_label("Enter");
    copy_btn2.connect_clicked(clone!(@weak clipboard, @weak from_entry => move |_btn| {
        let text2 = from_entry2.text();
        let terry_text = format!{"Terry "} + &text2;
        clipboard.set_text(&terry_text);
    }));
      copy_btn2.set_widget_name("button_enter");
    text_container2.append(&copy_btn2);

    let into_entry2 = gtk::Entry::new();
    text_container2.append(&into_entry2);

    let paste_btn2 = gtk::Button::with_label("Result");
    paste_btn2.connect_clicked(clone!(@weak clipboard, @weak into_entry => move |_btn| {
        clipboard.read_text_async(gio::Cancellable::NONE, clone!(@weak into_entry2 => move|res| {
            if let Ok(Some(terry_text)) = res {
                into_entry2.set_text(&terry_text);
            }
        }));
    }));
        paste_btn2.set_widget_name("button_results");
    text_container2.append(&paste_btn2);
    container.append(&text_container2);
    //
    //
        let titleq = gtk::Label::builder()
            .label("Quit")
        .halign(gtk::Align::Center)
        .build();
    titleq.add_css_class("title-2");
    container.append(&titleq);

    let text_containerq = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .orientation(gtk::Orientation::Horizontal)
        .spacing(24)
        .build();

    let button_4 = gtk::Button::with_label("Quit");
    button_4.connect_clicked(clone!(@weak window => move |_| window.destroy()));
    button_4.add_css_class("destructive-action");

    text_containerq.append(&button_4);
     container.append(&text_containerq);
    window.set_child(Some(&container));
    window.show();
}

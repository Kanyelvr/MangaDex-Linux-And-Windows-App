#[cfg(target_os = "linux")]
use {
    gtk4::prelude::*,
    gtk4::{Application, ApplicationWindow, Box, Orientation, EventControllerKey, EventControllerMotion, Button, Settings, Revealer, ToggleButton},
    webkit6::prelude::*,
    webkit6::WebView,
    arboard::Clipboard,
};

#[cfg(target_os = "windows")]
use {
    tao::event_loop::{ControlFlow, EventLoop},
    tao::window::WindowBuilder,
    wry::WebViewBuilder,
    arboard::Clipboard,
};

#[cfg(target_os = "linux")]
fn main() -> gtk4::glib::ExitCode {
    let app = Application::builder()
        .application_id("org.mangadex.desktop")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

#[cfg(target_os = "linux")]
fn build_ui(app: &Application) {
    let settings = Settings::default().unwrap();
    settings.set_property("gtk-application-prefer-dark-theme", true);
    
    let main_layout = Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // 1. REVEALER - Kontener, który płynnie wysuwa pasek
    let revealer = Revealer::builder()
        .transition_type(gtk4::RevealerTransitionType::SlideDown)
        .transition_duration(250)
        .build();

    let custom_bar = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(6)
        .margin_end(6)
        .build();
    
    let btn_back = Button::from_icon_name("go-previous-symbolic");
    let btn_forward = Button::from_icon_name("go-next-symbolic");
    let btn_reload = Button::from_icon_name("view-refresh-symbolic");
    
    // 2. PINEZKA - ToggleButton
    let btn_pin = ToggleButton::builder()
        .icon_name("pin-symbolic")
        .tooltip_text("Przypnij pasek")
        .build();

    custom_bar.append(&btn_back);
    custom_bar.append(&btn_forward);
    custom_bar.append(&btn_reload);

    let btn_url = Button::builder()
        .label("https://mangadex.org")
        .css_classes(["flat"])
        .hexpand(true)
        .build();
    custom_bar.append(&btn_url);
    custom_bar.append(&btn_pin);

    revealer.set_child(Some(&custom_bar));

    // 3. WEBVIEW
    let webview = WebView::new();
    webview.load_uri("https://mangadex.org");
    
    let webview_clone = webview.clone();
    btn_back.connect_clicked(move |_| { webview_clone.go_back(); });
    let webview_clone = webview.clone();
    btn_forward.connect_clicked(move |_| { webview_clone.go_forward(); });
    let webview_clone = webview.clone();
    btn_reload.connect_clicked(move |_| { webview_clone.reload(); });

    let webview_clone = webview.clone();
    let btn_url_clone = btn_url.clone();
    webview.connect_load_changed(move |_, event| {
        if event == webkit6::LoadEvent::Finished {
            if let Some(uri) = webview_clone.uri() {
                btn_url_clone.set_label(uri.as_str());
            }
        }
    });

    btn_url.connect_clicked(move |btn| {
        let text = btn.label().unwrap_or_default();
        if let Ok(mut clipboard) = Clipboard::new() {
            let _ = clipboard.set_text(text.as_str());
        }
    });

    main_layout.append(&revealer);
    webview.set_vexpand(true);
    webview.set_hexpand(true);
    main_layout.append(&webview);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("MangaDex")
        .default_width(1200)
        .default_height(800)
        .decorated(true)
        .child(&main_layout)
        .build();

    // 4. LOGIKA AUTO-HIDE (Wykrywanie myszy)
    let motion_controller = EventControllerMotion::new();
    let rev_clone = revealer.clone();
    let pin_clone = btn_pin.clone();
    
    motion_controller.connect_motion(move |_, _, y| {
        // Jeśli mysz jest w górnej części okna (np. 50px) lub pinezka jest wciśnięta
        if y < 50.0 || pin_clone.is_active() {
            rev_clone.set_reveal_child(true);
        } else {
            rev_clone.set_reveal_child(false);
        }
    });

    // Kiedy zjeżdżamy myszą całkiem z okna, chowamy (chyba że pinezka)
    let motion_controller_leave = EventControllerMotion::new();
    let rev_leave = revealer.clone();
    let pin_leave = btn_pin.clone();
    motion_controller_leave.connect_leave(move |_| {
        if !pin_leave.is_active() {
            rev_leave.set_reveal_child(false);
        }
    });

    window.add_controller(motion_controller);
    window.add_controller(motion_controller_leave);

    let key_controller = EventControllerKey::new();
    let window_clone = window.clone();
    key_controller.connect_key_pressed(move |_, key, _, _| {
        if key == gtk4::gdk::Key::F11 {
            if window_clone.is_fullscreen() {
                window_clone.unfullscreen();
            } else {
                window_clone.fullscreen();
            }
        }
        gtk4::glib::Propagation::Proceed
    });
    
    window.add_controller(key_controller);
    window.present();
}

#[cfg(target_os = "windows")]
fn main() {
    // Sekcja Windows (prosta wersja bez dynamicznego paska)
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("MangaDex")
        .with_inner_size(tao::dpi::LogicalSize::new(1200, 800))
        .with_decorations(true)
        .build(&event_loop)
        .unwrap();

    let _webview = WebViewBuilder::new(&window)
        .with_url("https://mangadex.org")
        .build()
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            tao::event::Event::WindowEvent {
                event: tao::event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
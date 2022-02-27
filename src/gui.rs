#![windows_subsystem = "windows"]
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
pub mod main;
pub use g_u_i_ui::GUIUi;
use main::SideBar;
use nwd::NwgUi;
use nwg::stretch::{geometry::Size, style::Dimension as D};
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct GUI {
    #[nwg_control(size: (1000, 500), position: (400, 200), title: "Lisa - CyphersSupporter")]
    #[nwg_events( OnWindowClose: [GUI::exit], OnInit: [GUI::setup] )]
    window: nwg::Window,

    #[nwg_layout(parent:window)]
    layout: nwg::FlexboxLayout,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, size:Size{width:D::Percent(0.2), height:D::Auto})]
    sidebar_frame: nwg::Frame,

    #[nwg_partial(parent:sidebar_frame)]
    side_bar: SideBar,

    #[nwg_control(text: "Btn 1")]
    #[nwg_layout_item(layout: layout, size:Size{width:D::Percent(0.8), height:D::Auto})]
    contents: nwg::Button,
}
impl GUI {
    fn setup(&self) {
        print!("setup");
    }
    fn exit(&self) {
        nwg::simple_message("Goodbye", "bye");
        nwg::stop_thread_dispatch();
    }
    pub fn load_ui(&self) {
        todo!();
    }
}

pub fn init() -> GUIUi {
    nwg::init().expect("Filed to start gui");
    // nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let mut font = nwg::Font::default();
    nwg::Font::builder()
        .size(16)
        .family("Segoe UI")
        .build(&mut font)
        .expect("failed set font");
    nwg::Font::set_global_default(Some(font));
    let app = GUI::build_ui(Default::default()).expect("failed to build ui");
    nwg::dispatch_thread_events();
    app
}

pub enum UI {
    Settings,
    Main,
    Records,
    Matches,
}

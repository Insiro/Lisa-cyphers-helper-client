use native_windows_derive as nwd;
use native_windows_gui as nwg;
use nwd::{NwgPartial, NwgUi};
use nwg::stretch::{
    geometry::{Rect, Size},
    style::{Dimension as D, FlexDirection},
};
const FIFTY_PC: D = D::Percent(100.0);
#[derive(Default, NwgPartial)]
pub struct SideBar {
    #[nwg_layout]
    layout: nwg::FlexboxLayout,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, size:Size{width:D::Percent(1.0), height:D::Percent(0.2)})]
    search_bar_frame: nwg::Frame,

    #[nwg_partial(parent:search_bar_frame)]
    search_bar: SearchBar,
}
// nwg::subclass_control!(SideBar, FlexboxLayout, layout);

#[derive(Default, NwgPartial)]
pub struct SearchBar {
    #[nwg_layout]
    layout: nwg::FlexboxLayout,
    #[nwg_control(text:"검색")]
    #[nwg_layout_item(layout: layout, size: Size { width: D::Percent(0.8), height: D::Auto })]
    search_bar: nwg::TextInput,
    #[nwg_control(text:"검색")]
    #[nwg_layout_item(layout: layout, size: Size { width: D::Percent(0.2), height: D::Auto })]
    search_btn: nwg::Button,
}

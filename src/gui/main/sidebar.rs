use native_windows_derive as nwd;
use native_windows_gui as nwg;
use nwd::NwgPartial;
use nwg::stretch::{
    geometry::{Rect, Size},
    style::{Dimension as D, FlexDirection},
};

#[derive(Default, NwgPartial)]
pub struct SideBar {
    #[nwg_layout(flex_direction:FlexDirection::Column)]
    layout: nwg::FlexboxLayout,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, size:Size{width:D::Auto, height:D::Points(40.0)})]
    search_bar_frame: nwg::Frame,

    #[nwg_control(text:"새로 고침")]
    #[nwg_layout_item(layout: layout, size:Size{width:D::Auto, height:D::Points(20.0)})]
    #[nwg_events(OnButtonClick: [SideBar::update_accessings(SELF)])]
    search_btn: nwg::Button,

    #[nwg_partial(parent:search_bar_frame)]
    search_bar: SearchBar,

    #[nwg_control(collection: vec![])]
    #[nwg_layout_item(layout:layout,flex_grow:1.0, size:Size{width:D::Auto, height:D::Auto})]
    result: nwg::ListBox<String>,
}
// nwg::subclass_control!(SideBar, FlexboxLayout, layout);

#[derive(Default, NwgPartial)]
pub struct SearchBar {
    #[nwg_layout(flex_direction:FlexDirection::Row,padding:Rect{bottom:D::Points(3.0), start:D::Points(1.0), end:D::Points(1.0),top:D::Points(3.0) },        )]
    layout: nwg::FlexboxLayout,
    #[nwg_control(text:"닉네임")]
    #[nwg_layout_item(layout: layout, flex_grow:1.0, size:Size{width:D::Auto, height:D::Auto})]
    search_bar: nwg::TextInput,
    #[nwg_control(text:"검색")]
    #[nwg_layout_item(layout: layout, size:Size{width:D::Percent(0.2), height:D::Points(30.0)})]
    search_btn: nwg::Button,
}

impl SideBar {
    pub fn update_accessings(&self) {}
}

use native_windows_gui as nwg;
use nwg::ControlHandle;
use std::rc::Rc;

fn build_side_bar(parent: &ControlHandle) {
    let layout = nwg::FlexboxLayout::default();
    nwg::FlexboxLayout::builder().parent(&parent).child()

}

use native_windows_derive as nwd;
use native_windows_gui as nwg;
use nwd::NwgPartial;

#[derive(Default, NwgPartial)]
pub struct Content {
    #[nwg_control(text:"content area")]
    result: nwg::Button,
}

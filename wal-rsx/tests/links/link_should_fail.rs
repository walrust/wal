use wal_rsx::rsx;

include!("../utils/non_display_struct.rs");

fn main() {
    rsx! { <Link /> };
    rsx! { <Link></Link> };
    rsx! { <Link to /> };
    rsx! { <Link to= /> };
    rsx! { <Link unsupported_attr="value" /> };
    rsx! { <Link to="value" unsupported_attr="value" /> };
    rsx! { <Link key="value" /> };
    rsx! { </Link> };
    rsx! { <Link> };
    rsx! { <Link to="value"> };
    rsx! { <Link to={NonDisplayStruct}></Link> };
    rsx! { <Link key={NonDisplayStruct} to="value"></Link> };
}

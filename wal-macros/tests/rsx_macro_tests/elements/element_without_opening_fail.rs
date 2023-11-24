use wal_macros::rsx;

fn main() {
    rsx! { </div> };
    rsx! { </span> };
    rsx! { </> };
    rsx! { <div></span> };
    rsx! { <div><span></div></span></div> };
}

use wal_macros::rsx;

fn main() {
    rsx! { if let };
    rsx! { if let {} };
    rsx! { if let true {}};
    rsx! { if let {} {} };
}

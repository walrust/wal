use wal_rsx::rsx;

fn main() {
    rsx! { if let };
    rsx! { if let {} };
    rsx! { if let true {}};
    rsx! { if let {} {} };
}

use wal_macros::html;

fn main() {
    html! { <Link /> };
    html! { <Link></Link> };
    html! { <Link to /> };
    html! { <Link to= /> };
    html! { <Link unsupported_attr="value" /> };
    html! { <Link to="value" unsupported_attr="value" /> };
    html! { <Link key="value" /> };
    html! { </Link> };
    html! { <Link> };
    html! { <Link to="value"> };
}

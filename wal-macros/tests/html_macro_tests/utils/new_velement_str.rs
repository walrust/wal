pub fn new_velement_str(
    tag_name: &str,
    attr: HashMap<&str, &str>,
    children: Vec<VNode>,
) -> VElement {
    VElement::new(
        tag_name.to_string(),
        attr.into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        children,
    )
}

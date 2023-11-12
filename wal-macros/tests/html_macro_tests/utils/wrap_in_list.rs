fn wrap_in_list(html: VNode) -> VNode {
    VNode::List(VList::new(vec![html], None))
}

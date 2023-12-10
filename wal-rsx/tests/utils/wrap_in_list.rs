fn wrap_in_list(vnode: VNode) -> VNode {
    VNode::List(VList::new(vec![vnode], None))
}

use crate::{EdgeT, NodeT};

pub(crate) trait TripleToItem<Item> {
    fn triple_to_item(triple: (EdgeT, NodeT, NodeT)) -> Item;
}

impl<I> TripleToItem<(NodeT, NodeT)> for I {
    #[inline(always)]
    fn triple_to_item(triple: (EdgeT, NodeT, NodeT)) -> (NodeT, NodeT) {
        (triple.1, triple.2)
    }
}

impl<I> TripleToItem<(EdgeT, NodeT, NodeT)> for I {
    #[inline(always)]
    fn triple_to_item(triple: (EdgeT, NodeT, NodeT)) -> (EdgeT, NodeT, NodeT) {
        triple
    }
}

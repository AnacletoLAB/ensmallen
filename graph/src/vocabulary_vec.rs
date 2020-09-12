use super::*;

pub(crate) struct VocabolaryVec<IndexT: ToFromUsize> {
    pub(crate) ids: Vec<IndexT>,
    pub(crate) vocabolary: Vocabolary<IndexT>
}

impl<IndexT: ToFromUsize> VocabolaryVec<IndexT> {
    pub fn new() -> VocabolaryVec<IndexT> {
        VocabolaryVec {
            ids: Vec::new(),
            vocabolary: Vocabolary::new(),
        }
    }

    pub fn add(&mut self, value: String) -> IndexT {
        self.vocabolary.add(value);
        let id = *self.get(&value).unwrap();
        self.ids.push(id);
        id
    }

    pub fn translate(&self, id: IndexT) -> String {
        self.vocabolary.translate(id)
    }

    pub fn get(&self, value: &str) -> Option<&IndexT> {
        self.vocabolary.get(value)
    }
}

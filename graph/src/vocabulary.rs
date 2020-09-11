



pub(crate) struct Vocabolary<KeyT, IndexT>  {
    pub(crate) ids: Vec<IndexT>,
    pub(crate) map: HashMap<KeyT, IndexT>,
    pub(crate) reverse_map: Vec<KeyT>
}

impl Vocabolary<KeyT, IndexT> {

    pub fn new() -> Vocabolary {
        Vocabolary{
            ids: Vec::new(),
            map: HashMap::new(),
            reverse_map: Vec::new(),
        }
    }

    pub fn add(&mut self, value: KeyT) -> IndexT {
        if !self.map.contains(value) {
            self.map.insert(value, map.len());
            reverse_map.push(value);
        }
        let id = self.get_id(value);
        ids.push(id);
        id
    }

    pub fn translate(&self, id: IndexT) -> KeyT {
        self.reverse_map[id]
    }

    pub fn get_id(&self, value: KeyT) -> Option<IndexT> {
        self.map.get(value)
    }
}
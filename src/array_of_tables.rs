use table::{Item, Table};

/// Type representing a TOML array of tables
#[derive(Clone, Debug, Default)]
pub struct ArrayOfTables {
    // always Vec<Item::Table>
    pub(crate) values: Vec<Item>,
}

type ArrayOfTablesIter<'a> = Box<Iterator<Item = &'a Table> + 'a>;

impl ArrayOfTables {
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns an iterator over tables
    pub fn iter(&self) -> ArrayOfTablesIter {
        Box::new(self.values.iter().filter_map(Item::as_table))
    }

    /// Returns an optional reference to the table
    pub fn get(&self, index: usize) -> Option<&Table> {
        self.values.get(index).and_then(Item::as_table)
    }

    /// Returns an optional mutable reference to the table
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Table> {
        self.values.get_mut(index).and_then(Item::as_table_mut)
    }

    pub fn append(&mut self, table: Table) -> &mut Table {
        self.values.push(Item::Table(table));
        let i = self.len() - 1;
        self.get_mut(i).unwrap()
    }

    pub fn remove(&mut self, index: usize) {
        self.values.remove(index);
    }

    pub fn clear(&mut self) {
        self.values.clear()
    }

    pub fn len(&self) -> usize {
        self.values.iter().filter(|i| i.is_table()).count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

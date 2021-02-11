use comfy_table::{Row, Table};
use frunk_core::hlist::{HCons, HNil};

/// A `HList` of `Cell`s
pub trait RowList {
    fn to_table(self) -> Table;
    fn add_rows(self, row: &mut Table);
}

impl<Tail> RowList for HCons<Row, Tail>
where
    Tail: RowList,
{
    fn to_table(self) -> Table {
        let mut table = Table::new();
        self.add_rows(&mut table);
        table
    }

    fn add_rows(self, table: &mut Table) {
        table.add_row(self.head);
        self.tail.add_rows(table);
    }
}

impl RowList for HNil {
    fn to_table(self) -> Table {
        panic!()
    }

    fn add_rows(self, _: &mut Table) {}
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use comfy_table::Row;
    use frunk_core::hlist;

    #[test]
    fn row_list() {
        let list = hlist![
            Row::from(vec![1, 2, 3]),
            Row::from(vec![4, 5, 6]),
            Row::from(vec![7, 8, 9]),
        ];
        let table = list.to_table();
        println!("\nTable:\n{}", table);
    }
}

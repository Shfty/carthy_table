use comfy_table::{Cell, Row};
use frunk_core::hlist::{HCons, HNil};

/// A `HList` of `Cell`s
pub trait CellList {
    fn to_row(self) -> Row;
    fn add_cells(self, row: &mut Row);
}

impl<Tail> CellList for HCons<Cell, Tail>
where
    Tail: CellList,
{
    fn to_row(self) -> Row {
        let mut row = Row::new();
        self.add_cells(&mut row);
        row
    }

    fn add_cells(self, row: &mut Row) {
        row.add_cell(self.head);
        self.tail.add_cells(row);
    }
}

impl CellList for HNil {
    fn to_row(self) -> Row {
        panic!()
    }

    fn add_cells(self, _: &mut Row) {}
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::tables::to_cell_list::ToCellList;

    use frunk_core::hlist;

    #[test]
    fn cell_list() {
        let list = hlist![1, 2, 3];
        let cells = list.list_to_cells();
        let row = cells.to_row();
        println!("\nRow:\n{:#?}", row);
    }
}

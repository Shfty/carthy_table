use comfy_table::{Row, Table};
use frunk_core::hlist::{HCons, HNil};

use super::{cell_list::CellList, row_list::RowList, to_cell_list::ToCellList};

/// `HList` of `ToCellList` types
pub trait ToCellTable {
    type Output;

    fn table_to_cells(self) -> Self::Output;
}

impl<Head, Tail> ToCellTable for HCons<Head, Tail>
where
    Head: ToCellList,
    Tail: ToCellTable,
{
    type Output = HCons<Head::Output, Tail::Output>;

    fn table_to_cells(self) -> Self::Output {
        HCons {
            head: self.head.list_to_cells(),
            tail: self.tail.table_to_cells(),
        }
    }
}

impl ToCellTable for HNil {
    type Output = HNil;

    fn table_to_cells(self) -> Self::Output {
        HNil
    }
}

/// `HList` of `RowList` types
pub trait CellListTable {
    type Output;

    fn table_to_rows(self) -> Self::Output;
}

impl<Head, Tail> CellListTable for HCons<Head, Tail>
where
    Head: CellList,
    Tail: CellListTable,
{
    type Output = HCons<Row, Tail::Output>;

    fn table_to_rows(self) -> Self::Output {
        HCons {
            head: self.head.to_row(),
            tail: self.tail.table_to_rows(),
        }
    }
}

impl CellListTable for HNil {
    type Output = HNil;

    fn table_to_rows(self) -> Self::Output {
        HNil
    }
}

/// `ToCellTable` type that can be converted into a `Table`
pub trait ToTable: ToCellTable {
    fn to_table(self) -> Table;
}

impl<T> ToTable for T
where
    T: ToCellTable,
    T::Output: CellListTable,
    <T::Output as CellListTable>::Output: RowList,
{
    fn to_table(self) -> Table {
        self.table_to_cells().table_to_rows().to_table()
    }
}

/// `Iterator` type that can be converted into a `Table`
pub trait IteratorToTable<T> {
    fn to_table(self) -> Table;
}

impl<T, I> IteratorToTable<T> for I
where
    I: Iterator<Item = T>,
    T: ToCellList,
{
    fn to_table(self) -> Table {
        let mut table = Table::new();
        for row in self {
            table.add_row(row.list_to_cells().to_row());
        }
        table
    }
}

#[cfg(test)]
mod tests {
    use carth::{hlist::homogenous_list::HomogenousListIntoIter, htable::Transpose};

    use super::*;
    use frunk_core::hlist;

    #[test]
    fn test_to_cell_table() {
        let table = hlist![hlist![1, 2, 3], hlist![4, 5, 6]];
        println!("\nTable:\n{}", table.transpose().to_table());
    }

    #[test]
    fn test_iterator_to_table() {
        let table = hlist![hlist![1, 2, 3], hlist![4, 5, 6]];
        let iter = table.into_iter();
        let _proof: &dyn IteratorToTable<_> = &iter;
        let table = iter.to_table();
        println!("\nTable:\n{}", table);
    }
}

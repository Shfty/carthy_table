use carth::{
    hlist::homogenous_list::HomogenousList,
    htable::{PopColumn, Transpose},
};
use comfy_table::Table;
use frunk_core::hlist::{HCons, HNil};

use carth::hlist::FieldList;

use super::{cell_list::CellList, table::ToTable, to_cell_list::ToCellList};

/// `HList` of `FieldList` types
pub trait FieldTable<'a> {
    type NameTypes;
    type ValueTypes;

    type NamesOutput;
    type ValuesOutput;

    fn names(&self) -> Self::NamesOutput;
    fn values(&'a self) -> Self::ValuesOutput;
}

impl<'a, Head, Tail> FieldTable<'a> for HCons<Head, Tail>
where
    Head: FieldList<'a>,
    Tail: FieldTable<'a>,
{
    type NameTypes = HCons<<Head as FieldList<'a>>::NameTypes, Tail::NameTypes>;
    type ValueTypes = HCons<<Head as FieldList<'a>>::ValueTypes, Tail::ValueTypes>;

    type NamesOutput = HCons<<Head as FieldList<'a>>::NamesOutput, Tail::NamesOutput>;
    type ValuesOutput = HCons<<Head as FieldList<'a>>::ValuesOutput, Tail::ValuesOutput>;

    fn names(&self) -> Self::NamesOutput {
        HCons {
            head: self.head.names(),
            tail: self.tail.names(),
        }
    }

    fn values(&'a self) -> Self::ValuesOutput {
        HCons {
            head: self.head.values(),
            tail: self.tail.values(),
        }
    }
}

impl<'a> FieldTable<'a> for HNil {
    type NameTypes = HNil;
    type ValueTypes = HNil;

    type NamesOutput = HNil;
    type ValuesOutput = HNil;

    fn names(&self) -> Self::NamesOutput {
        HNil
    }

    fn values(&'a self) -> Self::ValuesOutput {
        HNil
    }
}

/// `HList` of `HomogenousList` types
trait HomogenousRowTable<T> {}

impl<NameHead, NameTail, Head, Tail> HomogenousRowTable<HCons<NameHead, NameTail>>
    for HCons<Head, Tail>
where
    Head: HomogenousList<NameHead>,
    Tail: HomogenousRowTable<NameTail>,
{
}

impl HomogenousRowTable<HNil> for HNil {}

/// `HList` of `HList`s of `Field` types whose name types are homogenous
trait InnerNamedColumnTable<'a, N, Width> {
    fn to_table(&'a self) -> Table;
}

impl<'a, T, N, Width> InnerNamedColumnTable<'a, N, Width> for T
where
    T: FieldTable<'a>,
    T::ValuesOutput: Transpose<Width>,
    <T::ValuesOutput as Transpose<Width>>::Output: ToTable,
    T::NamesOutput: PopColumn,
    <T::NamesOutput as PopColumn>::Column: ToCellList,
    T::NameTypes: HomogenousRowTable<N>,
{
    fn to_table(&'a self) -> Table {
        let mut table = self.values().transpose().to_table();
        let header = self.names().pop_column().0.list_to_cells().to_row();
        table.set_header(header);
        table
    }
}

/// `HList` of `HList`s of `Field` types whose name types are homogenous
trait InnerNamedRowTable<'a, N, Width> {
    fn to_table(&'a self) -> Table;
}

impl<'a, T, N, Width, NamesInnerTail, NamesTail> InnerNamedRowTable<'a, N, Width> for T
where
    T: FieldTable<'a, NamesOutput = HCons<HCons<&'static str, NamesInnerTail>, NamesTail>>,
    T::ValuesOutput: ToTable,
    T::NameTypes: Transpose<Width>,
    <T::NameTypes as Transpose<Width>>::Output: HomogenousRowTable<N>,
    HCons<&'static str, NamesInnerTail>: ToCellList,
{
    fn to_table(&'a self) -> Table {
        let mut table = self.values().to_table();
        let header = self.names().pop().0.list_to_cells().to_row();
        table.set_header(header);
        table
    }
}

/// `Iterator` over inner-named rows that can be converted into a `Table`
pub trait InnerNamedRowIteratorToTable<'a, T> {
    fn to_table(self) -> Table;
}

impl<'a, T, I> InnerNamedRowIteratorToTable<'a, T> for I
where
    I: Iterator<Item = T>,
    T: FieldList<'a>,
    T::NamesOutput: ToCellList,
    T::ValueTypes: ToCellList,
{
    fn to_table(mut self) -> Table {
        let mut table = Table::new();
        
        if let Some(head) = self.next() {
            table.set_header(head.names().list_to_cells().to_row());
            table.add_row(head.into_values().list_to_cells().to_row());
        }

        for item in self {
            table.add_row(item.into_values().list_to_cells().to_row());
        }

        table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use carth::hlist::homogenous_list::HomogenousListIntoIter;
    use frunk_core::{field, hlist};

    #[test]
    fn test_inner_named_table() {
        let inner_named_columns_table = hlist![
            hlist![field!(i32, 1), field!(i32, 2), field!(i32, 3)],
            hlist![field!(f32, 4.1), field!(f32, 4.2), field!(f32, 4.3)],
            hlist![field!(char, '7'), field!(char, '8'), field!(char, '9')],
        ];

        let names = inner_named_columns_table.names();
        let values = inner_named_columns_table.values();
        println!("\nNames:\n{:#?}", names);
        println!("\nValues:\n{:#?}", values);
    }

    #[test]
    fn test_inner_named_columns_table() {
        let inner_named_columns_table = hlist![
            hlist![field!(i32, 1), field!(i32, 2), field!(i32, 3)],
            hlist![field!(f32, 4.1), field!(f32, 4.2), field!(f32, 4.3)],
            hlist![field!(char, '7'), field!(char, '8'), field!(char, '9')],
        ];

        let table = inner_named_columns_table.to_table();
        println!("\nTable:\n{}", table);
    }

    #[test]
    fn test_inner_named_rows_table() {
        let inner_named_rows_table = hlist![
            hlist![field!(i32, 1), field!(f32, 4.1), field!(char, '7')],
            hlist![field!(i32, 2), field!(f32, 5.2), field!(char, '8')],
            hlist![field!(i32, 3), field!(f32, 6.3), field!(char, '9')],
        ];

        let table = inner_named_rows_table.to_table();
        println!("\nTable:\n{}", table);
    }

    #[test]
    fn test_inner_named_row_iterator_to_table() {
        let inner_named_rows_table = hlist![
            hlist![field!(i32, 1), field!(f32, 4.1), field!(char, '7')],
            hlist![field!(i32, 2), field!(f32, 5.2), field!(char, '8')],
            hlist![field!(i32, 3), field!(f32, 6.3), field!(char, '9')],
        ];

        let iter = inner_named_rows_table.into_iter();
        let _proof: &dyn InnerNamedRowIteratorToTable<_> = &iter;
        let table = iter.to_table();
        println!("\nTable:\n{}", table);
    }
}

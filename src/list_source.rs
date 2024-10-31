use crate::list_item::ListItem;

pub trait ListSource {
    type ListSourceItem: ListItem;

    fn items(&self) -> impl Iterator<Item = Self::ListSourceItem>;
    fn item(&self, index: usize) -> Option<Self::ListSourceItem>;
}

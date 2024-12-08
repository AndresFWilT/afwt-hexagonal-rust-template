use crate::domain::entities::SortRequest;

pub trait SorterPort {
    fn sort(&self, request: SortRequest) -> Vec<u128>;
}

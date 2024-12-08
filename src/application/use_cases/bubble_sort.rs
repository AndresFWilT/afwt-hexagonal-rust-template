use crate::domain::entities::{SortRequest, SortResponse};
use crate::domain::ports::SorterPort;

pub struct BubbleSort;

impl SorterPort for BubbleSort {
    fn sort(&self, request: SortRequest) -> Vec<u128> {
        let mut sorted_array = request.array.clone();
        let n = sorted_array.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                if sorted_array[j] > sorted_array[j + 1] {
                    sorted_array.swap(j, j + 1);
                }
            }
        }
        sorted_array
    }
}


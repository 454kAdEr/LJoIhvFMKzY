use tokio::runtime::Runtime;

/// Sorting functionality
pub struct SortingAlgorithm;

impl SortingAlgorithm {
    /// Sorts a vector of integers using bubble sort algorithm.
    #[allow(dead_code)]
    pub fn bubble_sort(numbers: Vec<i32>) -> Vec<i32> {
        let mut sorted_numbers = numbers.clone();
        let len = sorted_numbers.len();

        for i in 0..len {
            for j in 0..len-i-1 {
                if sorted_numbers[j] > sorted_numbers[j + 1] {
                    sorted_numbers.swap(j, j + 1);
                }
            }
        }

        sorted_numbers
    }

    /// Sorts a vector of integers using quick sort algorithm.
    #[allow(dead_code)]
    pub fn quick_sort(numbers: Vec<i32>) -> Vec<i32> {
        let mut numbers = numbers.clone();
        SortingAlgorithm::quick_sort_recursive(&mut numbers)
    }

    /// Helper function for quick sort algorithm.
    #[allow(dead_code)]
    fn quick_sort_recursive(numbers: &mut [i32]) {
        if numbers.len() <= 1 {
            return;
        }

        let pivot_index = numbers.len() / 2;
        let pivot = std::mem::replace(&mut numbers[pivot_index], i32::MAX);
        let mut i = 0;
        let mut j = 0;

        while j < numbers.len() {
            if numbers[j] < pivot {
                numbers.swap(i, j);
                i += 1;
            }
            j += 1;
        }

        numbers.swap(i, pivot_index);
        SortingAlgorithm::quick_sort_recursive(&mut numbers[0..i]);
        SortingAlgorithm::quick_sort_recursive(&mut numbers[i + 1..]);
    }
}

#[tokio::main]
async fn main() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        println!("Before bubble sort: {:?}", numbers);
        let sorted = SortingAlgorithm::bubble_sort(numbers.clone());
        println!("After bubble sort: {:?}", sorted);

        println!("Before quick sort: {:?}", numbers);
        let sorted = SortingAlgorithm::quick_sort(numbers);
        println!("After quick sort: {:?}", sorted);
    });
}

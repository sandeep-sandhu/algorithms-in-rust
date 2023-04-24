// sorting algorithms

use std::fmt::Display;
use log::{error, debug};


// Merge Sort:
pub fn mergesort<T: Display + Copy + PartialOrd>(input:&Vec<T>) -> Vec<T> {
    debug!("Merge sort.");
    if input.len()< 2 {
        return input.to_vec();
    }

    let mid:usize = input.len()/2;
    let (left, right) = input.split_at(mid);
    return merge(
        mergesort(Vec::from(left).as_ref()).as_ref(),
        mergesort(Vec::from(right).as_ref()).as_ref()
    );
}

fn merge<T: Display + Copy + PartialOrd>(left:&Vec<T>, right:&Vec<T>) -> Vec<T> {
    let mut result:Vec<T>= Vec::new();
    let n:usize = left.len();
    let m:usize = right.len();
    let mut lp:usize = 0;
    let mut rp:usize = 0;
    while result.len() < n + m {
        debug!("Merge examining left array idx lp={lp}, right array idx rp={rp}, n={n}, m={m}");
        if lp < n && rp < m && left[lp] < right[rp] {
            result.push(left[lp]);
            lp += 1;
        }
        else if lp < n && rp < m && left[lp] >= right[rp]{
            result.push(right[rp]);
            rp += 1;
        }
        else if rp == m && lp < n {
            result.push(left[lp]);
            lp += 1;
        }
        else if lp == n && rp < m {
            result.push(right[rp]);
            rp += 1;
        }
    }
    return result;
}



// Quicksort:
// Recursively partition the vector into two halves, sort them independently, and then
// append left half with pivot and right half in order to get the sorted vector
pub fn quicksort<T: Sized + Copy + PartialOrd + std::fmt::Display>(input: &mut Vec<T>)
    -> &mut Vec<T> {
    debug!("Quick sorting vector of length {}", input.len());

    // base case:
    // nothing to sort if given an empty vector or single element!
    if input.len() < 2 {
        return input;
    }else if input.len() == 2 {
        if input[0] > input[1]{
            input.swap(0,1);
        }
        return input;
    }

    // recursive case:
    let partition_index: usize = _quicksort_partition(input.len());
    let mut pivot: T = input.swap_remove(partition_index);
    debug!("Selected item '{pivot}' at #{partition_index} as pivot out of {} items", input.len());

    // move items to left of vector if they are less than pivot value:
    // TODO: find out if we can eliminate making copies and new variables, use only swap
    let mut left_half: Vec<T> = input.clone();
    left_half.retain_mut(|x| x < &mut pivot);
    // move items to right of vector if they are >= than pivot value
    input.retain(|x| x >= &mut pivot);

    let mut result:Vec<T>=Vec::new();
    result = Vec::clone(quicksort(&mut left_half));
    result.push(pivot);
    result.append(quicksort(input));

    result.clone_into( input);
    return input;
}

fn _quicksort_partition(input_size: usize) -> usize{
    // partition the vector in different ways:

    // random choice of pivot:
    //let mut rng = rand::thread_rng();
    //return rng.gen_range(0..input_size);

    //choose last element as pivot:
    // return input_size-1;

    // choose middle element as the pivot:
    return input_size/2;
}


// Selection Sort:
// Find the smallest element in input vector repeatedly and
// move it into a new vector which is the resulting sorted vector
pub fn selection_sort<T: Sized + Copy + PartialOrd + std::fmt::Display>(
    input: &mut Vec<T>
) {
    debug!("Selection sorting vector of length {}", input.len());

    // nothing to sort if given an empty vector or single element!
    if input.len() < 2 {
        return;
    }

    let mut sorted_vec:Vec<T> = Vec::new();
    let mut smallest_index: usize;
    let mut iteration_counter:usize = 0;

    while input.is_empty() == false {

        smallest_index = _get_idx_of_smallest_no(input);
        iteration_counter += input.len();

        debug!("Next smallest item identified at index #{}, value = {}, adding it to sorted vector",
            smallest_index,
            input[smallest_index]);

        sorted_vec.push(input[smallest_index]);
        input.swap_remove(smallest_index);

    }

    debug!("Completed {iteration_counter} iterations in total for vector size {}", input.len());
    //now, copy sorted vector back into input:
    sorted_vec.clone_into(input);
}

fn _get_idx_of_smallest_no<T: Sized + Copy + PartialOrd + std::fmt::Display>(
    input: &mut Vec<T>
) -> usize {

    let mut smallest_no:T = input[0];
    let mut idx_of_smallest_no:usize = 0;

    for loop_counter in 0..input.len() {
        //Check whether input[loop_counter] is smaller than smallest
        if input[loop_counter] < smallest_no {

            smallest_no = input[loop_counter];
            idx_of_smallest_no = loop_counter;
        }
    }

    return idx_of_smallest_no;
}

// Insertion sort: move elements from input vector progressively to a new vector
// each time searching for the best position in target vector to insert it into
// When the input vector is empty, the target vector contains the resulting sorted vector
pub fn insertion_sort<T: Sized + Copy + PartialOrd + std::fmt::Display>(
    input: &mut Vec<T>
) {
    debug!("Insertion sorting vector of length {}", input.len());

    // nothing to sort if given an empty vector or single element!
    if input.len() < 2 {
        return;
    }

    let mut sorted_vec:Vec<T> = Vec::new();
    let mut iteration_counter:usize=0;
    let mut was_item_moved:bool;

    // add last element as-is:
    // remove it from the input vector
    match input.pop() {
        Some(last_item) => sorted_vec.push(last_item),
        None => println!("No content in input vector!")
    }
    iteration_counter += 1;

    while input.is_empty() == false {
        // keep track of whether appropriate index was found in the sorted array.
        was_item_moved = false;
        //Checking each element of sorted_vec against the first item in input vector
        for i in 0..sorted_vec.len() {
            iteration_counter += 1;
            if sorted_vec[i] > input[0] {
                // add the element 'input[0]' to position 'i' of the sorted vector:
                sorted_vec.insert(i, input[0]);
                input.swap_remove(0);
                was_item_moved = true;
                // since the sorted vector is already sorted, no need to check later elements
                // hence, terminate the for loop now
                break;
            }
        }
        if was_item_moved == false {
            // add element input[0] to the end of the sorted vector
            sorted_vec.push(input[0]);
            input.swap_remove(0);
        }
    }

    debug!("Completed {iteration_counter} iterations in total for vector size {}", input.len());

    //in the end, copy the sorted vector into input:
    sorted_vec.clone_into(input);
}

// Bubble sort: Iterate through the vector one by one and swap each element with
// the one to its right if it is bigger. At the end of the iterations, the vector will be sorted.
pub fn bubble_sort<T: Sized + Copy + PartialOrd + std::fmt::Display>(input: &mut Vec<T>) {
    debug!("Bubble sorting vector of length {}", input.len());

    // nothing to sort if given an empty vector or single element!
    if input.len() < 2 {
        error!("Nothing to sort!");
        return;
    }

    let input_size: usize = input.len();
    let mut iteration_counter: usize = 0;
    let mut element_was_swapped: bool;

    for outer_counter in 0..input_size {
        debug!("Loop no: {outer_counter}");
        element_was_swapped = false;

        // last element is already in the correct position, so skip looping till that position
        for index in 0..(input_size - outer_counter - 1) {
            iteration_counter += 1;
            let next_index = index + 1;

            if input[index] > input[next_index] {
                element_was_swapped = true;
                debug!(
                    "\tIteration {index}: Swapping element {} with {}",
                    input[index],
                    input[next_index]
                );
                input.swap(next_index,index);
            }
        }

        if element_was_swapped == false {
            debug!("Exiting early as vector is already sorted.");
            break;
        }
    }

    debug!("Completed {iteration_counter} iterations in total for vector size {input_size}");
}


// the unit tests:
#[cfg(test)]
mod tests {
    use crate::sorting;

    fn get_test_vector() -> Vec<isize> {
        return Vec::from([3, 79, 5, 96, 0, 27, 4, 8, 11, 47]);
    }

    fn get_smallest_index() -> usize {
        return 4;
    }

    fn get_sorted_vector() -> Vec<isize> {
        let mut test_vector: Vec<isize> = get_test_vector();
        test_vector.sort();
        return test_vector;
    }

    // _find_smallest
    #[test]
    fn find_smallest_ten_numbers() {
        let mut test_numbers: Vec<isize> = get_test_vector();
        let smallest_idx: usize = sorting::_get_idx_of_smallest_no(&mut test_numbers);
        assert_eq!(
            get_smallest_index(),
            smallest_idx,
            "Identifying smallest number in input vector"
        )
    }

    #[test]
    fn mergesort_ten_numbers() {
        let mut test_numbers: Vec<isize> = get_test_vector();
        let sorted_numbers: Vec<isize> = get_sorted_vector();
        let computes_answer: Vec<isize> = sorting::mergesort(&mut test_numbers);
        assert_eq!(
            sorted_numbers,
            computes_answer,
            "MergeSort - check whether input vector got sorted."
        )
    }

    #[test]
    fn quicksort_ten_numbers() {
        let mut test_numbers: Vec<isize> = get_test_vector();
        let sorted_numbers: Vec<isize> = get_sorted_vector();
        sorting::quicksort(&mut test_numbers);
        assert_eq!(
            sorted_numbers,
            test_numbers,
            "QuickSort - check whether input vector got sorted."
        )
    }

    #[test]
    fn selection_sort_ten_numbers() {
        let mut test_numbers: Vec<isize> = get_test_vector();
        let sorted_numbers: Vec<isize> = get_sorted_vector();
        sorting::selection_sort(&mut test_numbers);
        assert_eq!(
            sorted_numbers,
            test_numbers,
            "Selection sort - check whether input vector got sorted."
        )
    }

    #[test]
    fn insertion_sort_ten_numbers() {
        let mut test_numbers: Vec<isize> = get_test_vector();
        let sorted_numbers: Vec<isize> = get_sorted_vector();
        sorting::insertion_sort(&mut test_numbers);
        assert_eq!(
            sorted_numbers,
            test_numbers,
            "Insertion sort - check whether input vector got sorted."
        )
    }

    #[test]
    fn bubble_sort_ten_numbers() {
        let mut test_numbers: Vec<isize> = get_test_vector();
        let sorted_numbers: Vec<isize> = get_sorted_vector();
        sorting::bubble_sort(&mut test_numbers);
        assert_eq!(
            sorted_numbers,
            test_numbers,
            "Bubble sort - checking whether input vector got sorted"
        )
    }

    #[test]
    fn bubble_sort_single_element_vector() {
        let mut test_numbers: Vec<isize> = Vec::from([3]);
        let sorted_numbers: Vec<isize> = Vec::from([3]);
        sorting::bubble_sort(&mut test_numbers);
        assert_eq!(
            sorted_numbers,
            test_numbers,
            "Bubble sort - checking whether single element vector got sorted"
        )
    }

    #[test]
    fn bubble_sort_empty_vector() {
        let mut test_numbers: Vec<isize> = Vec::new();
        let sorted_numbers: Vec<isize> = Vec::new();
        sorting::bubble_sort(&mut test_numbers);
        assert_eq!(
            sorted_numbers,
            test_numbers,
            "Bubble sort - checking whether empty vector got sorted"
        )
    }
}

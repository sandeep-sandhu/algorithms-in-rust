use algorithms_in_rust;

use algorithms_in_rust::logging::SimpleDebugLogger;
use algorithms_in_rust::logging::SimpleInfoLogger;

static DBG_LOGGER: SimpleDebugLogger = SimpleDebugLogger;
static LOGGER: SimpleInfoLogger = SimpleInfoLogger;

// initialise the debug logger
pub fn init_debug_logger() {
    log::set_logger(&DBG_LOGGER).expect("Could not set the debug logger!");
    log::set_max_level(log::LevelFilter::Debug);
}

pub fn init_logger() {
    log::set_logger(&LOGGER).expect("Could not set the logger!");
    log::set_max_level(log::LevelFilter::Debug);
}

//
fn get_test_vector_fps() -> Vec<f32> {
    return Vec::from([3.0, 79.0, 5.5, 96.0, 0.1, 27.0, 4.0, 8.0, 11.0, 47.0]);
}

fn get_sorted_vector_fps() -> Vec<f32> {
    return Vec::from([0.1, 3.0, 4.0, 5.5, 8.0, 11.0, 27.0, 47.0, 79.0, 96.0]);
}

#[test]
fn quicksort_ten_fp_numbers() {
    println!("Quick sorting floating point numbers:");
    let mut test_vector: Vec<f32> = get_test_vector_fps();
    algorithms_in_rust::sorting::quicksort(&mut test_vector);
    algorithms_in_rust::visualize::print_vector(&test_vector);
    assert_eq!(
        get_sorted_vector_fps(),
        test_vector,
        "Quicksort - checking whether floating points vector got sorted"
    )
}

#[test]
fn selection_sort_ten_fp_numbers() {
    println!("Selection sorting floating point numbers:");
    let mut test_vector: Vec<f32> = get_test_vector_fps();
    algorithms_in_rust::sorting::selection_sort(&mut test_vector);
    algorithms_in_rust::visualize::print_vector(&test_vector);
    assert_eq!(
        get_sorted_vector_fps(),
        test_vector,
        "Selection sort - checking whether floating points vector got sorted"
    )
}

#[test]
fn insertion_sort_ten_numbers() {
    println!("Insertion sorting floating point numbers:");
    let mut test_vector: Vec<f32> = get_test_vector_fps();
    algorithms_in_rust::sorting::insertion_sort(&mut test_vector);
    assert_eq!(
        get_sorted_vector_fps(),
        test_vector,
        "Insertion sort - check whether input vector got sorted."
    )
}

#[test]
fn bubble_sort_ten_fp_numbers() {
    init_debug_logger();
    println!("Bubble sorting floating point numbers:");
    let mut test_vector: Vec<f32> = get_test_vector_fps();
    algorithms_in_rust::sorting::bubble_sort(&mut test_vector);
    algorithms_in_rust::visualize::print_vector(&test_vector);
    assert_eq!(
        get_sorted_vector_fps(),
        test_vector,
        "Bubble sort - Checking whether floating points vector got sorted"
    )
}

#[test]
fn bubble_sort_ten_chars() {
    println!("Bubble sorting character vector");
    let mut test_vector3: Vec<char> = Vec::from(['h', 's', 'w', 'i', 'z']);
    algorithms_in_rust::sorting::bubble_sort(&mut test_vector3);
    algorithms_in_rust::visualize::print_vector(&test_vector3);
    assert_eq!(
        Vec::from(['h', 'i', 's', 'w', 'z']),
        test_vector3,
        "Checking whether character vector got sorted"
    )
}
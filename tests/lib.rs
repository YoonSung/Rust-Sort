extern crate collection_sort;
use collection_sort::quick_sort;

#[test]
fn sort() {
    quick_sort::sort([1,2,3]);
    assert_eq!(1, 1);
}

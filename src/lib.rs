mod binaryheap;

pub use binaryheap::BinaryHeap;
pub use binaryheap::HeapKind;

#[cfg(test)]
mod tests {
    use super::*;

    fn insert_seed_data(heap: &mut BinaryHeap<i32>) {
        heap.insert(4);
        heap.insert(4);
        heap.insert(8);
        heap.insert(9);
        heap.insert(5);
        heap.insert(12);
        heap.insert(11);
        heap.insert(13);
    }

    #[test]
    fn test_extract_min() {
        let mut heap = BinaryHeap::new(HeapKind::Min);

        insert_seed_data(&mut heap);
        assert_eq!(heap.extract_object(), Some(4));
        assert_eq!(heap.extract_object(), Some(4));
        assert_eq!(heap.extract_object(), Some(5));

        heap.insert(1);
        assert_eq!(heap.extract_object(), Some(1));

        assert_eq!(heap.extract_object(), Some(8));
        assert_eq!(heap.extract_object(), Some(9));
        assert_eq!(heap.extract_object(), Some(11));
        assert_eq!(heap.extract_object(), Some(12));
        assert_eq!(heap.extract_object(), Some(13));
        assert_eq!(heap.extract_object(), None);
        assert_eq!(heap.extract_object(), None);

        heap.insert(100);
        assert_eq!(heap.extract_object(), Some(100));

        heap.insert(10);
        heap.insert(5);
        heap.insert(8);
        heap.insert(3);
        heap.insert(2);
        heap.insert(7);
        heap.insert(1);
        assert_eq!(heap.extract_object(), Some(1));
        assert_eq!(heap.extract_object(), Some(2));
        assert_eq!(heap.extract_object(), Some(3));
        assert_eq!(heap.extract_object(), Some(5));
        assert_eq!(heap.extract_object(), Some(7));
        assert_eq!(heap.extract_object(), Some(8));
        assert_eq!(heap.extract_object(), Some(10));
        assert_eq!(heap.extract_object(), None);
    }

    #[test]
    fn test_extract_max() {
        let mut heap = BinaryHeap::new(HeapKind::Max);

        insert_seed_data(&mut heap);
        assert_eq!(heap.extract_object(), Some(13));
        assert_eq!(heap.extract_object(), Some(12));
        assert_eq!(heap.extract_object(), Some(11));

        heap.insert(1);
        assert_eq!(heap.extract_object(), Some(9));

        assert_eq!(heap.extract_object(), Some(8));
        assert_eq!(heap.extract_object(), Some(5));
        assert_eq!(heap.extract_object(), Some(4));
        assert_eq!(heap.extract_object(), Some(4));
        assert_eq!(heap.extract_object(), Some(1));
        assert_eq!(heap.extract_object(), None);
        assert_eq!(heap.extract_object(), None);

        heap.insert(100);
        assert_eq!(heap.extract_object(), Some(100));

        heap.insert(10);
        heap.insert(5);
        heap.insert(8);
        heap.insert(3);
        heap.insert(2);
        heap.insert(7);
        heap.insert(1);
        assert_eq!(heap.extract_object(), Some(10));
        assert_eq!(heap.extract_object(), Some(8));
        assert_eq!(heap.extract_object(), Some(7));
        assert_eq!(heap.extract_object(), Some(5));
        assert_eq!(heap.extract_object(), Some(3));
        assert_eq!(heap.extract_object(), Some(2));
        assert_eq!(heap.extract_object(), Some(1));
        assert_eq!(heap.extract_object(), None);
    }

    #[test]
    fn test_peek() {
        let mut heap = BinaryHeap::new(HeapKind::Min);
        insert_seed_data(&mut heap);

        assert_eq!(heap.peek(), Some(&4));
        assert_eq!(heap.extract_object(), Some(4));
        assert_eq!(heap.extract_object(), Some(4));

        assert_eq!(heap.peek(), Some(&5));
    }
}

mod binaryheap;

pub use binaryheap::BinaryHeap;
pub use binaryheap::HeapKind;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Standard;
    use rand::seq::{IteratorRandom, SliceRandom};
    use rand::{thread_rng, Rng};
    use std::collections::HashSet;

    fn insert_seed_data(heap: &mut BinaryHeap<i32>, data: &[i32]) {
        data.iter().for_each(|item| {
            heap.insert(*item);
        });
    }

    #[test]
    fn test_get_index() {
        let mut heap = BinaryHeap::new(HeapKind::Min);
        let mut rng = thread_rng();
        let data: Vec<i32> = (&mut rng).sample_iter(Standard).take(10000).collect();
        insert_seed_data(&mut heap, &data);

        data.iter().for_each(|item| {
            let present_in_indices = heap.get_index(item).unwrap();
            for index in present_in_indices {
                assert_eq!(heap.element_at(*index), Some(item));
            }
        });
    }

    #[test]
    fn test_extract_min() {
        let mut heap = BinaryHeap::new(HeapKind::Min);
        let mut rng = thread_rng();
        let mut data: Vec<i32> = (&mut rng).sample_iter(Standard).take(10000).collect();
        insert_seed_data(&mut heap, &data);
        data.sort();
        data.into_iter().for_each(|item| {
            assert_eq!(heap.extract_object(), Some(item));
        });
        assert_eq!(heap.extract_object(), None);
        assert_eq!(heap.extract_object(), None);
    }

    #[test]
    fn test_extract_max() {
        let mut heap = BinaryHeap::new(HeapKind::Max);
        let mut rng = thread_rng();
        let mut data: Vec<i32> = (&mut rng).sample_iter(Standard).take(10000).collect();
        insert_seed_data(&mut heap, &data);
        data.sort();
        data.reverse();
        data.into_iter().for_each(|item| {
            assert_eq!(heap.extract_object(), Some(item));
        });
        assert_eq!(heap.extract_object(), None);
        assert_eq!(heap.extract_object(), None);
    }

    #[test]
    fn test_peek() {
        let mut heap = BinaryHeap::new(HeapKind::Min);
        let data = [4, 4, 8, 3, 9, 5, 12, 11, 13];
        insert_seed_data(&mut heap, &data);

        assert_eq!(heap.peek(), Some(&3));
        assert_eq!(heap.extract_object(), Some(3));
        assert_eq!(heap.extract_object(), Some(4));
        assert_eq!(heap.peek(), Some(&4));
    }

    #[test]
    fn test_remove_object() {
        let mut heap = BinaryHeap::new(HeapKind::Min);
        let mut rng = thread_rng();
        let data: Vec<i32> = (&mut rng).sample_iter(Standard).take(10000).collect();
        insert_seed_data(&mut heap, &data);
        let mut items_removed = HashSet::new();
        let number_of_items_to_remove: usize = rng.gen_range(500..4000);

        for _ in 0..number_of_items_to_remove {
            let item_to_remove = data.iter().choose(&mut rand::thread_rng()).unwrap();
            heap.remove_object(item_to_remove);
            items_removed.insert(item_to_remove);
        }

        let mut remaining_items: Vec<i32> = data
            .iter()
            .filter(|element| !items_removed.contains(element))
            .copied()
            .collect();

        remaining_items.sort();
        remaining_items.iter().for_each(|element| {
            heap.get_index(element).unwrap().iter().for_each(|index| {
                assert_eq!(heap.element_at(*index), Some(element));
            });
        });

        remaining_items.into_iter().for_each(|element| {
            assert_eq!(Some(element), heap.extract_object());
        });
    }

    #[test]
    fn test_remove_object_with_duplicates() {
        let mut heap = BinaryHeap::new(HeapKind::Min);
        let data = [4, 8, 3, 9, 5, 12, 11, 2, 13];
        insert_seed_data(&mut heap, &data);
        let mut items_removed = HashSet::new();
        while items_removed.len() != data.len() {
            let item_to_remove = data.choose(&mut rand::thread_rng()).unwrap();
            heap.remove_object(item_to_remove);
            items_removed.insert(*item_to_remove);
        }
        assert_eq!(heap.extract_object(), None);

        let data = [4, 3, 4];
        let mut heap = BinaryHeap::new(HeapKind::Min);
        insert_seed_data(&mut heap, &data);

        assert_eq!(Some(4), heap.remove_object(&4));
        assert_eq!(Some(4), heap.remove_object(&4));
        assert_eq!(None, heap.remove_object(&4));
        assert_eq!(None, heap.remove_object(&4));
        assert_eq!(Some(3), heap.remove_object(&3));
        assert_eq!(None, heap.remove_object(&5));
        assert_eq!(heap.extract_object(), None);
    }
}

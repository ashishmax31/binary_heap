#![warn(clippy::all)]
use ::core::hash::{BuildHasher, Hasher};
use hashbrown::HashMap;
use std::collections::hash_map::RandomState;
use std::collections::VecDeque;

const PARENT_VIOLATION: &str = "PARENT_VIOLATION";
const CHILDREN_VIOLATION: &str = "CHILDREN_VIOLATION";

pub enum HeapKind {
    Max,
    Min,
}

pub struct BinaryHeap<T, S = RandomState> {
    elements: VecDeque<T>,
    kind: HeapKind,
    element_indices: HashMap<u64, Vec<usize>>,
    hash_builder: S,
}

impl<T, S> BinaryHeap<T, S>
where
    T: std::cmp::PartialOrd + Clone + std::hash::Hash + std::cmp::Eq + std::fmt::Debug,
    S: BuildHasher + Default,
{
    pub fn new(heap_type: HeapKind) -> Self {
        Self {
            elements: VecDeque::new(),
            kind: heap_type,
            element_indices: HashMap::new(),
            hash_builder: S::default(),
        }
    }

    pub fn heapify(items: &[T], kind: HeapKind) -> Self {
        items.iter().fold(Self::new(kind), |mut acc, item| {
            acc.insert(item.clone());
            acc
        })
    }

    // O(log n)
    pub fn insert(&mut self, object: T) {
        self.push_back(object);
        let currently_inserted_index = self.elements.len() - 1;
        self.bubble_up(currently_inserted_index);
    }

    // Extract the highest_priority object from the heap
    // O(log n)
    pub fn extract_object(&mut self) -> Option<T> {
        self.handle_table_changes();
        let max_priority_elem = self.elements.pop_front();
        match self.elements.pop_back() {
            Some(last_entry) => {
                self.push_front(last_entry);
                self.bubble_down(0);
                max_priority_elem
            }
            None => max_priority_elem,
        }
    }

    pub fn remove_object(&mut self, object: &T) -> Option<T> {
        if let Some(present_indices) = self.get_index(object) {
            let index_to_remove = present_indices[0];
            let last_element_index = self.len() - 1;
            // If the element to be removed is the first element in the vector, then we simply call extract_object().
            // On the otherhand, if the element is the last element in the vector, we remove the element's index entry from the table
            // and then call pop_back on the vector.
            match index_to_remove {
                0 => self.extract_object(),
                x if x == last_element_index => {
                    self.remove_from_table(last_element_index, last_element_index);
                    self.elements.pop_back()
                }
                _ => {
                    self.swap_elements(index_to_remove, last_element_index);
                    self.remove_from_table(last_element_index, last_element_index);
                    let removed_element = self.elements.pop_back();
                    let res = self.check_heap_invariants_at(
                        index_to_remove,
                        self.element_at(index_to_remove).unwrap(),
                    );
                    self.ensure_heap_invariants(res, index_to_remove);
                    removed_element
                }
            }
        } else {
            None
        }
    }

    fn ensure_heap_invariants(
        &mut self,
        invariant_status: (Option<&'static str>, Option<&'static str>),
        concerned_index: usize,
    ) {
        match invariant_status {
            (Some(a), Some(b)) => {
                self.fix_invariant(concerned_index, a);
                self.fix_invariant(concerned_index, b);
            }
            (None, Some(a)) => self.fix_invariant(concerned_index, a),
            (Some(a), None) => self.fix_invariant(concerned_index, a),
            (None, None) => {}
        };
    }

    fn fix_invariant(&mut self, concerned_index: usize, invariant: &'static str) {
        match invariant {
            error if error == PARENT_VIOLATION => self.bubble_up(concerned_index),
            error if error == CHILDREN_VIOLATION => self.bubble_down(concerned_index),
            _ => panic!("Unsupported heap invariant"),
        }
    }

    fn check_heap_invariants_at(
        &self,
        disturbed_index: usize,
        element: &T,
    ) -> (Option<&'static str>, Option<&'static str>) {
        match (
            self.verify_parent(disturbed_index, element),
            self.verify_children(disturbed_index, element),
        ) {
            (true, true) => (None, None),
            (true, false) => (None, Some(CHILDREN_VIOLATION)),
            (false, true) => (Some(PARENT_VIOLATION), None),
            (false, false) => (Some(PARENT_VIOLATION), Some(CHILDREN_VIOLATION)),
        }
    }

    fn handle_table_changes(&mut self) {
        if self.elements.front().is_some() {
            self.remove_from_table(0, 0);
        }
        if self.elements.back().is_some() {
            let vec_len = self.elements.len() - 1;
            self.remove_from_table(vec_len, vec_len);
        }
    }

    fn update_table_for_element_entry(&mut self, element_index: usize) {
        let hash_value =
            Self::hash_value(&self.hash_builder, &self.element_at(element_index).unwrap());

        if let Some(element_present_at) = self.element_indices.get_mut(&hash_value) {
            //  Duplicates
            element_present_at.push(element_index);
        } else {
            // Insert the elements index in the vector [Element is unique in the vector]
            self.element_indices.insert(hash_value, vec![element_index]);
        }
    }

    fn update_table_for_swap(&mut self, ind1: usize, ind2: usize) {
        self.remove_from_table(ind1, ind2);
        self.remove_from_table(ind2, ind1);
        self.update_table_for_element_entry(ind1);
        self.update_table_for_element_entry(ind2);
    }

    pub(crate) fn get_index(&self, element: &T) -> Option<&[usize]> {
        let hash_value = Self::hash_value(&self.hash_builder, element);
        let present_indices = self.element_indices.get(&hash_value).and_then(|indicies| {
            if indicies.is_empty() {
                None
            } else {
                Some(indicies)
            }
        });
        present_indices.map(|indices| &indices[..])
    }

    fn remove_from_table(&mut self, element_ind: usize, element_was_at: usize) {
        let hash_value =
            Self::hash_value(&self.hash_builder, &self.element_at(element_ind).unwrap());
        if let Some(indices) = self.element_indices.get_mut(&hash_value) {
            let items_to_be_retained: Vec<usize> = indices
                .iter()
                .filter(|ind| **ind != element_was_at)
                .copied()
                .collect();
            indices.clear();
            assert_eq!(indices.len(), 0);
            items_to_be_retained.into_iter().for_each(|ind| {
                indices.push(ind);
            });
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.front()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn verify_priority(&self, obj1: &T, obj2: &T) -> bool {
        match self.kind {
            HeapKind::Max => obj1 >= obj2,
            HeapKind::Min => obj1 <= obj2,
        }
    }

    fn bubble_up(&mut self, start_ind: usize) {
        let mut new_element_pos = start_ind;
        while !self.verify_heap_property(new_element_pos) {
            let parent_ind = self.parent_index(new_element_pos).unwrap();
            self.swap_elements(new_element_pos, parent_ind);
            new_element_pos = parent_ind;
        }
    }

    fn bubble_down(&mut self, start_ind: usize) {
        let mut new_element_pos = start_ind;
        while !self.verify_heap_property(new_element_pos) {
            let children_indices = self.children_indices(new_element_pos);
            let priority_ind = self.index_with_priority(children_indices);
            self.swap_elements(priority_ind, new_element_pos);
            new_element_pos = priority_ind;
        }
    }

    // Verifies the heap property b/w the given node and its parent and children.
    fn verify_heap_property(&self, index: usize) -> bool {
        let current_node = self.element_at(index).unwrap();
        self.verify_parent(index, current_node) && self.verify_children(index, current_node)
    }

    fn index_with_priority(&self, indices: [Option<usize>; 2]) -> usize {
        match (indices[0], indices[1]) {
            (Some(child1_ind), Some(child2_ind)) => {
                let child1 = self.element_at(child1_ind).unwrap();
                let child2 = self.element_at(child2_ind).unwrap();
                if self.verify_priority(child1, child2) {
                    child1_ind
                } else {
                    child2_ind
                }
            }
            (Some(child1_ind), None) => child1_ind,
            (None, Some(child2_ind)) => child2_ind,
            (None, None) => panic!("Heap Internal error!"),
        }
    }

    fn verify_parent(&self, child_node_ind: usize, child: &T) -> bool {
        if let Some(parent_ind) = self.parent_index(child_node_ind) {
            let parent = self.element_at(parent_ind).unwrap();
            self.verify_priority(parent, child)
        } else {
            true
        }
    }

    fn verify_children(&self, index: usize, parent: &T) -> bool {
        self.children_indices(index)
            .iter()
            .filter_map(|item| item.as_ref())
            .all(|child_ind| {
                let child = self.element_at(*child_ind).unwrap();
                self.verify_priority(parent, child)
            })
    }

    pub(crate) fn element_at(&self, ind: usize) -> Option<&T> {
        self.elements.get(ind)
    }

    fn parent_index(&self, child_ind: usize) -> Option<usize> {
        self.element_at(child_ind).and_then(|_| match child_ind {
            ind if even(ind) => {
                let parent_ind = (ind as i32 / 2_i32) - 1_i32;
                self.element_at(parent_ind as usize)
                    .map(|_| parent_ind as usize)
            }
            _ => {
                let parent_ind = child_ind / 2;
                self.element_at(parent_ind).map(|_| parent_ind)
            }
        })
    }

    fn children_indices(&self, parent_ind: usize) -> [Option<usize>; 2] {
        let child_1_ind = ((parent_ind + 1) * 2) as i32 - 1_i32;
        let child_2_ind = (parent_ind + 1) * 2;
        let child_1 = self.element_at(child_1_ind as usize);
        let child_2 = self.element_at(child_2_ind);
        match (child_1, child_2) {
            (Some(_), Some(_)) => [Some(child_1_ind as usize), Some(child_2_ind)],
            (Some(_), None) => [Some(child_1_ind as usize), None],
            (None, Some(_)) => [None, Some(child_2_ind)],
            (None, None) => [None, None],
        }
    }

    fn hash_value(hash_builder: &S, element: &T) -> u64 {
        let mut h = hash_builder.build_hasher();
        element.hash(&mut h);
        h.finish()
    }

    fn push_back(&mut self, object: T) {
        self.elements.push_back(object);
        let currently_inserted_index = self.elements.len() - 1;
        self.update_table_for_element_entry(currently_inserted_index);
    }

    fn push_front(&mut self, object: T) {
        self.elements.push_front(object);
        self.update_table_for_element_entry(0);
    }

    fn swap_elements(&mut self, ind1: usize, ind2: usize) {
        //  1, 0
        self.elements.swap(ind1, ind2);
        // 3, 4
        self.update_table_for_swap(ind1, ind2);
        // 0, 1
    }
}

fn even(num: usize) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parent_and_child_indices() {
        let heap = BinaryHeap::<i32>::heapify(&[4, 4, 8, 9, 5, 12, 11, 13], HeapKind::Min);
        assert_eq!(heap.parent_index(0), None);
        assert_eq!(heap.parent_index(1), Some(0));
        assert_eq!(heap.parent_index(2), Some(0));
        assert_eq!(heap.parent_index(3), Some(1));
        assert_eq!(heap.parent_index(4), Some(1));
        assert_eq!(heap.parent_index(5), Some(2));
        assert_eq!(heap.parent_index(6), Some(2));
        assert_eq!(heap.parent_index(7), Some(3));
        assert_eq!(heap.parent_index(8), None);
        assert_eq!(heap.parent_index(110), None);

        assert_eq!(heap.children_indices(0), [Some(1), Some(2)]);
        assert_eq!(heap.children_indices(1), [Some(3), Some(4)]);
        assert_eq!(heap.children_indices(2), [Some(5), Some(6)]);
        assert_eq!(heap.children_indices(3), [Some(7), None]);
        assert_eq!(heap.children_indices(4), [None, None]);
    }
}

#![warn(clippy::all)]
use std::collections::VecDeque;

pub enum HeapKind {
    Max,
    Min,
}

pub struct BinaryHeap<T> {
    elements: VecDeque<T>,
    kind: HeapKind,
}

impl<T> BinaryHeap<T>
where
    T: std::cmp::PartialOrd,
{
    pub fn new(heap_type: HeapKind) -> Self {
        Self {
            elements: VecDeque::new(),
            kind: heap_type,
        }
    }

    pub fn insert(&mut self, object: T) {
        self.elements.push_back(object);
        let inserted_index = self.elements.len() - 1;
        self.bubble_up(inserted_index);
    }

    // Extract the highest_priority object from the heap
    pub fn extract_object(&mut self) -> Option<T> {
        let max_priority_elem = self.elements.pop_front();
        match self.elements.pop_back() {
            Some(last_entry) => self.elements.push_front(last_entry),
            None => return max_priority_elem,
        }
        self.bubble_down(0);
        max_priority_elem
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.front()
    }

    fn verify_priority(&self, obj1: &T, obj2: &T) -> bool {
        match self.kind {
            HeapKind::Max => obj1 >= obj2,
            HeapKind::Min => obj1 <= obj2,
        }
    }

    fn bubble_up(&mut self, start_ind: usize) {
        if !self.verify_heap_property(start_ind) {
            let parent_ind = Self::parent_index(start_ind);
            self.elements.swap(start_ind, parent_ind);
            self.bubble_up(parent_ind)
        }
    }

    fn bubble_down(&mut self, start: usize) {
        if !self.verify_heap_property(start) {
            let children_indices = Self::children_indices(start);
            let min_ind = self.min_index(children_indices);
            self.elements.swap(min_ind, start);
            self.bubble_down(min_ind)
        }
    }

    // Verifies the heap property b/w the given node and its parent and children.
    fn verify_heap_property(&self, index: usize) -> bool {
        let current_node = self.element_at(index).unwrap();
        self.verify_parent(index, current_node) && self.verify_children(index, current_node)
    }

    fn min_index(&self, indices: [usize; 2]) -> usize {
        let child1 = self.element_at(indices[0]);
        let child2 = self.element_at(indices[1]);
        match child1.zip(child2) {
            Some((child1, child2)) => {
                if self.verify_priority(child1, child2) {
                    indices[0]
                } else {
                    indices[1]
                }
            }
            None => {
                let res = child1.xor(child2);
                if res.is_none() {
                    panic!("Internal Heap data corruption")
                } else if res == child1 {
                    indices[0]
                } else {
                    indices[1]
                }
            }
        }
    }

    fn verify_parent(&self, child_node_ind: usize, child: &T) -> bool {
        match self.element_at(Self::parent_index(child_node_ind)) {
            Some(parent) => self.verify_priority(parent, child),
            None => true,
        }
    }

    fn verify_children(&self, index: usize, parent: &T) -> bool {
        Self::children_indices(index).iter().all(|child_ind| {
            if let Some(child) = self.element_at(*child_ind) {
                self.verify_priority(parent, child)
            } else {
                true
            }
        })
    }

    fn element_at(&self, ind: usize) -> Option<&T> {
        self.elements.get(ind)
    }

    fn parent_index(child_ind: usize) -> usize {
        let child_ind = child_ind as i32;
        if child_ind == 0 {
            return 0;
        }

        if child_ind % 2 == 0 {
            ((child_ind / 2) - 1) as usize
        } else {
            (child_ind / 2) as usize
        }
    }

    fn children_indices(parent_ind: usize) -> [usize; 2] {
        [(parent_ind + 1) * 2 - 1, (parent_ind + 1) * 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parent_and_child_indices() {
        let mut heap = BinaryHeap::new(HeapKind::Min);
        heap.insert(4);
        heap.insert(4);
        heap.insert(8);
        heap.insert(9);
        heap.insert(5);
        heap.insert(12);
        heap.insert(11);
        heap.insert(13);

        assert_eq!(BinaryHeap::<i32>::parent_index(0), 0);
        assert_eq!(BinaryHeap::<i32>::parent_index(1), 0);
        assert_eq!(BinaryHeap::<i32>::parent_index(2), 0);
        assert_eq!(BinaryHeap::<i32>::parent_index(3), 1);
        assert_eq!(BinaryHeap::<i32>::parent_index(4), 1);
        assert_eq!(BinaryHeap::<i32>::parent_index(5), 2);
        assert_eq!(BinaryHeap::<i32>::parent_index(6), 2);
        assert_eq!(BinaryHeap::<i32>::parent_index(7), 3);
        assert_eq!(BinaryHeap::<i32>::parent_index(8), 3);

        assert_eq!(BinaryHeap::<i32>::children_indices(0), [1, 2]);
        assert_eq!(BinaryHeap::<i32>::children_indices(1), [3, 4]);
        assert_eq!(BinaryHeap::<i32>::children_indices(2), [5, 6]);
        assert_eq!(BinaryHeap::<i32>::children_indices(3), [7, 8]);
    }
}

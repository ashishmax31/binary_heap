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
    T: std::cmp::PartialOrd + Clone,
{
    pub fn new(heap_type: HeapKind) -> Self {
        Self {
            elements: VecDeque::new(),
            kind: heap_type,
        }
    }

    pub fn heapify(items: &[T], kind: HeapKind) -> Self {
        items.iter().fold(Self::new(kind), |mut acc, item| {
            acc.insert(item.clone());
            acc
        })
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
        if !self.verify_heap_property(start_ind) {
            let parent_ind = self.parent_index(start_ind).unwrap();
            self.elements.swap(start_ind, parent_ind);
            self.bubble_up(parent_ind)
        }
    }

    fn bubble_down(&mut self, start: usize) {
        if !self.verify_heap_property(start) {
            let children_indices = self.children_indices(start);
            let priority_ind = self.index_with_priority(children_indices);
            self.elements.swap(priority_ind, start);
            self.bubble_down(priority_ind)
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

    fn element_at(&self, ind: usize) -> Option<&T> {
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
}

fn even(num: usize) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parent_and_child_indices() {
        let heap = BinaryHeap::heapify(&[4, 4, 8, 9, 5, 12, 11, 13], HeapKind::Min);
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

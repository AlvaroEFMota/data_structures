pub struct MinHeap {
    array: Vec<i32>,
    capacity: usize,
    len: usize,
}

impl MinHeap {
    pub fn new(capacity: usize) -> Self {
        // let array:Vec<i32> = Vec::with_capacity(capacity);
        Self {
            array: vec![0; capacity],
            capacity: capacity,
            len: 0,
        }
    }
    pub fn pop(&mut self) -> i32 {
        if self.len > 0 {
            let result = self.array[0];
            self.array[0] = self.array[self.len - 1];
            self.len -= 1;
            self.min_heap_remove(0);
            return result;
        } else {
            panic!("Error");
        }
    }

    pub fn push(&mut self, x: i32) {
        if self.len < self.capacity {
            if self.len > 0 {
                self.array[self.len] = x;
                self.min_heap_insert(self.len);
                self.len += 1;
            } else {
                self.array[self.len] = x;
                self.len += 1;
            }
        } else {
            panic!("Error");
        }
    }

    fn min_heap_insert(&mut self, i: usize) {
        if i > 0 {
            let parent = (i - 1) / 2;
            if self.array[i] < self.array[parent] {
                let temporary = self.array[i];
                self.array[i] = self.array[parent];
                self.array[parent] = temporary;
                self.min_heap_insert(parent);
            }
        }
    }

    fn min_heap_remove(&mut self, i: usize) {
        let mut smallest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        if left < self.len && self.array[i] > self.array[left] {
            smallest = left;
        } else {
            smallest = i;
        }

        if right < self.len && self.array[right] < self.array[smallest] {
            smallest = right;
        }

        if smallest != i {
            let temporary = self.array[smallest];
            self.array[smallest] = self.array[i];
            self.array[i] = temporary;
            self.min_heap_remove(smallest);
        }
    }

    pub fn empty(&self) -> bool {
        return self.len <= 0;
    }
}

#[cfg(test)]
mod test {
    use super::MinHeap;
    #[test]
    fn min_heap_insertion_and_removal() {
        let mut min_heap = MinHeap::new(10);
        min_heap.push(5);
        min_heap.push(9);
        min_heap.push(3);
        min_heap.push(0);
        min_heap.push(8);
        min_heap.push(7);
        min_heap.push(1);
        min_heap.push(2);
        min_heap.push(4);
        min_heap.push(6);

        assert_eq!(min_heap.pop(), 0);
        assert_eq!(min_heap.pop(), 1);
        assert_eq!(min_heap.pop(), 2);
        assert_eq!(min_heap.pop(), 3);
        assert_eq!(min_heap.pop(), 4);
        assert_eq!(min_heap.pop(), 5);
        assert_eq!(min_heap.pop(), 6);
        assert_eq!(min_heap.pop(), 7);
        assert_eq!(min_heap.pop(), 8);
        assert_eq!(min_heap.pop(), 9);
    }
}

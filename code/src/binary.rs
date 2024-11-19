use crate::Entry;

pub struct Heap<K, T> {
    keys: Vec<K>,
    aux: Vec<T>,
    len: usize,
}

impl<K, T> Heap<K, T>
where
    K: Default + Clone + Copy + PartialOrd + Ord + PartialEq + Eq,
    T: Default + Clone + Copy,
{
    pub fn with_capacity(capacity: usize) -> Self {
        let mut keys = Vec::with_capacity(capacity);
        let mut aux = Vec::with_capacity(capacity);

        keys.push(K::default());
        aux.push(T::default());

        Self { keys, aux, len: 0 }
    }

    pub fn min(&self) -> Option<Entry<K, T>> {
        if self.len == 0 {
            None
        } else {
            Some(Entry {
                key: self.keys[1],
                aux: self.aux[1],
            })
        }
    }

    fn ascend(&mut self, mut index: usize) {
        let mut parent = index >> 1;
        while parent > 0 {
            if self.keys[index] < self.keys[parent] {
                self.swap(index, parent);
            } else {
                break;
            }

            index = parent;
            parent >>= 1;
        }
    }

    fn descend(&mut self, mut index: usize) {
        let mut left_child = index << 1;
        let mut right_child;
        let mut min_child;

        while left_child < self.keys.len() {
            right_child = left_child + 1;

            min_child = if right_child < self.keys.len()
                && self.keys[left_child] > self.keys[right_child]
            {
                right_child
            } else {
                left_child
            };

            if self.keys[index] <= self.keys[min_child] {
                break;
            }

            self.swap(index, min_child);
            index = min_child;
            left_child = index << 1;
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.keys.swap(a, b);
        self.aux.swap(a, b);
    }

    pub fn insert(&mut self, key: K, aux: T) {
        let new_index = self.keys.len();
        self.keys.push(key);
        self.aux.push(aux);
        self.ascend(new_index);
        self.len += 1;
    }

    pub fn extract_min(&mut self) -> Option<Entry<K, T>> {
        if self.len == 0 {
            return None;
        }

        let result = Some(Entry {
            key: self.keys[1],
            aux: self.aux[1],
        });

        self.keys.swap_remove(1);
        self.aux.swap_remove(1);
        self.descend(1);

        self.len -= 1;

        result
    }

    pub fn print(&self)
    where
        K: std::fmt::Display,
    {
        let mut newline_index = 1;
        let mut index = 1;
        while index < self.keys.len() {
            print!("{}, ", self.keys[index]);
            if index % newline_index == 0 {
                println!();
                newline_index <<= 1;
                newline_index += 1;
            }
            index += 1;
        }
        println!();
    }

    pub fn is_empty(&self) -> bool { self.len == 0 }
}

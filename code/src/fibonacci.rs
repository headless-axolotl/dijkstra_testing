use crate::Entry;

#[derive(Debug)]
pub struct Heap<K, T> {
    keys: Vec<K>,
    aux: Vec<T>,

    left_neighbour: Vec<usize>,
    right_neighbour: Vec<usize>,
    child: Vec<usize>,
    parent: Vec<usize>,
    degree: Vec<usize>,
    is_marked: Vec<bool>,

    degree_remapping: Vec<usize>,
    empty_indeces: Vec<usize>,

    len: usize,
    min_root: usize,
}

impl<K, T> Heap<K, T>
where
    K: Clone + Copy + PartialEq + Eq + PartialOrd + Ord,
    T: Clone + Copy,
{
    pub fn new(capacity: usize) -> Self {
        let logn = (usize::BITS - capacity.leading_zeros()) as usize;
        Self {
            keys: Vec::with_capacity(capacity),
            aux: Vec::with_capacity(capacity),

            left_neighbour: Vec::with_capacity(capacity),
            right_neighbour: Vec::with_capacity(capacity),
            child: Vec::with_capacity(capacity),
            parent: Vec::with_capacity(capacity),
            degree: Vec::with_capacity(capacity),
            is_marked: Vec::with_capacity(capacity),

            degree_remapping: Vec::with_capacity(logn),
            empty_indeces: Vec::with_capacity(capacity),

            len: 0,
            min_root: 0,
        }
    }

    pub fn min(&self) -> Option<Entry<K, T>> {
        if self.min_root >= self.klen() {
            None
        } else {
            Some(Entry::new(
                self.keys[self.min_root],
                self.aux[self.min_root],
            ))
        }
    }

    fn detach(&mut self, node: usize) {
        let left_neighbour = self.left_neighbour[node];
        let right_neighbour = self.right_neighbour[node];
        self.left_neighbour[right_neighbour] = left_neighbour;
        self.right_neighbour[left_neighbour] = right_neighbour;

        self.left_neighbour[node] = node;
        self.right_neighbour[node] = node;

        let previous_parent = self.parent[node];
        self.parent[node] = node;

        if previous_parent == node {
            return;
        }

        self.degree[previous_parent] -= 1;

        if self.child[previous_parent] != node {
            return;
        }

        if right_neighbour != node {
            self.child[previous_parent] = right_neighbour;
        } else {
            self.child[previous_parent] = previous_parent;
        }
    }

    fn append(&mut self, node: usize, front: usize) {
        let end = self.left_neighbour[front];
        let node_end = self.left_neighbour[node];
        self.right_neighbour[end] = node;
        self.left_neighbour[node] = end;
        self.right_neighbour[node_end] = front;
        self.left_neighbour[front] = node_end;
    }

    fn attach(&mut self, child: usize, parent: usize) {
        self.detach(child);

        self.parent[child] = parent;
        self.degree[parent] += 1;

        if self.child[parent] == parent {
            self.child[parent] = child;
            return;
        }

        self.append(child, self.child[parent]);
    }

    fn join(&mut self, first: usize, second: usize) -> usize {
        let (child, parent) = if self.keys[first] >= self.keys[second] {
            (first, second)
        } else {
            (second, first)
        };

        self.attach(child, parent);
        parent
    }

    #[inline]
    fn klen(&self) -> usize {
        self.keys.len()
    }

    pub fn insert(&mut self, key: K, aux: T) -> usize {
        let mut new_index = self.klen();

        if !self.empty_indeces.is_empty() {
            new_index = self.empty_indeces.pop().unwrap();
            self.keys[new_index] = key;
            self.aux[new_index] = aux;
            self.left_neighbour[new_index] = new_index;
            self.right_neighbour[new_index] = new_index;
            self.child[new_index] = new_index;
            self.parent[new_index] = new_index;
            self.degree[new_index] = 0;
            self.is_marked[new_index] = false;
        } else {
            self.keys.push(key);
            self.aux.push(aux);
            self.left_neighbour.push(new_index);
            self.right_neighbour.push(new_index);
            self.child.push(new_index);
            self.parent.push(new_index);
            self.degree.push(0);
            self.is_marked.push(false);
        }

        self.append(new_index, self.min_root);

        if key < self.keys[self.min_root] {
            self.min_root = new_index
        }

        self.len += 1;
        new_index
    }

    pub fn extract_min(&mut self) -> Option<Entry<K, T>> {
        if self.len == 0 {
            return None;
        }

        let result = Some(Entry::new(
            self.keys[self.min_root],
            self.aux[self.min_root],
        ));

        self.append(self.child[self.min_root], self.min_root);

        self.child[self.min_root] = self.min_root;

        let previous_min_root = self.min_root;
        self.min_root = self.right_neighbour[self.min_root];

        self.detach(previous_min_root);

        self.empty_indeces.push(previous_min_root);

        self.consolidate();

        self.len -= 1;

        result
    }

    fn consolidate(&mut self) {
        if self.len == 0 {
            return;
        }

        let logn = (usize::BITS - self.len.leading_zeros()) as usize;
        self.degree_remapping.resize(logn, self.klen());
        for i in 0..logn {
            self.degree_remapping[i] = self.klen();
        }

        let mut root_count = 0;
        let start_root = self.min_root;
        let mut current_root = self.min_root;
        let mut next_root = self.right_neighbour[self.min_root];

        loop {
            self.parent[current_root] = current_root;

            if self.keys[current_root] < self.keys[self.min_root] {
                self.min_root = current_root;
            }

            current_root = next_root;
            next_root = self.right_neighbour[current_root];

            root_count += 1;

            if current_root == start_root {
                break;
            }
        }

        current_root = self.min_root;
        for _ in 0..root_count {
            next_root = self.right_neighbour[current_root];
            loop {
                let deg = self.degree[current_root];

                if self.degree_remapping[deg] == self.klen() {
                    self.degree_remapping[deg] = current_root;
                    break;
                }

                let other = self.degree_remapping[deg];

                self.degree_remapping[deg] = self.klen();
                current_root = self.join(current_root, other);
            }
            current_root = next_root;
        }
    }

    pub fn decrease_key(&mut self, node: usize, value: K) {
        self.keys[node] = value;
        if value < self.keys[self.parent[node]] {
            self.cut_out(node);
        }
    }

    fn cut_out(&mut self, node: usize) {
        let previous_parent = self.parent[node];

        self.detach(node);
        self.append(node, self.min_root);
        self.is_marked[node] = false;

        if self.keys[node] < self.keys[self.min_root] {
            self.min_root = node;
        }

        if self.is_marked[previous_parent] {
            self.cut_out(previous_parent);
        } else {
            self.is_marked[previous_parent] = true;
        }
    }

    pub fn print(&self) 
    where 
        K: std::fmt::Display
    {
        self.print_node(self.min_root, 0);
    }

    fn print_node(&self, index: usize, depth: usize)
    where
        K: std::fmt::Display,
    {
        let offset = (0..depth).map(|_| "--").collect::<String>();

        let mut current_node = index;
        let mut next_node = self.right_neighbour[index];

        loop {
            print!("{}", offset);
            println!("|{}-{}", current_node, self.keys[current_node]);
            if self.child[current_node] != current_node {
                self.print_node(self.child[current_node], depth + 1);
            }

            current_node = next_node;
            next_node = self.right_neighbour[current_node];

            if current_node == index {
                break;
            }
        }
    }
}



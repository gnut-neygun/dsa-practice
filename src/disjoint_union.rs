use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::FromIterator;
use std::mem;
use std::ops::Not;
use crate::disjoint_union::UnionSetError::NotExist;
use crate::disjoint_union::ValueNode::{Children, Value};

#[derive(Debug)]
struct UnionSet<T> {
    forest: HashMap<T, Node<T>>,
}

#[derive(Debug)]
enum UnionSetError {
    NotExist,
}

#[derive(Debug)]
struct Node<T> {
    parent: Cell<*mut Node<T>>,
    content: ValueNode<T>
}

#[derive(Debug)]
enum ValueNode<T>{
    Value(T),
    Children(Vec<*const Node<T>>)
}

impl<T: Eq + Hash + Clone> UnionSet<T> {
    fn new() -> UnionSet<T> {
        UnionSet {
            forest: HashMap::new()
        }
    }

    fn join(&mut self, a: T, b: T) -> Result<(), UnionSetError> {
        let node_a = self.forest.get(&a).ok_or(NotExist)?;
        let node_b = self.forest.get(&b).ok_or(NotExist)?;
        let mut parent = Node {
            parent: Cell::new(std::ptr::null_mut()),
            content: Children(vec![node_a, node_b]),
        };
        if node_a.parent.get().is_null() {
            node_a.parent.set(&mut parent as *mut _);
        } else {
            let mut current_node = node_a.parent.get();
            let mut current_parent = unsafe { (*current_node).parent.get() };
            while current_parent.is_null().not() {
                current_node = current_parent;
                current_parent = unsafe { (*current_node).parent.get() };
            }
            unsafe {
                (*current_node).parent.set(&mut parent as *mut _);
            }
        }
        if node_b.parent.get().is_null() {
            node_b.parent.set(&mut parent as *mut _);
        } else {
            let mut current_node = node_b.parent.get();
            let mut current_parent = unsafe { (*current_node).parent.get() };
            while current_parent.is_null().not() {
                current_node = current_parent;
                current_parent = unsafe { (*current_node).parent.get() };
            }
            unsafe {
                (*current_node).parent.set(&mut parent as *mut _);
            }
        }
        Ok(())
    }

    fn is_equivalent(&self, a: T, b:T) -> Result<bool, UnionSetError> {
        let node_a = self.forest.get(&a).ok_or(NotExist)?;
        let node_b = self.forest.get(&b).ok_or(NotExist)?;
        let mut current_node = node_a;
        let mut current_parent = current_node.parent.get();
        while current_parent.is_null().not() {
            current_parent = unsafe { (*current_node).parent.get() };
        }
        let mut current_node_b = node_b;
        let mut current_parent_b = current_node_b.parent.get();
        while current_parent_b.is_null().not() {
            current_node_b = unsafe { &*current_parent_b };
            current_parent_b = unsafe { (*current_node_b).parent.get() };
        }
        Ok(current_node as *const _ == current_node_b as *const _)
    }
}

impl<T: Eq + Hash + Clone> FromIterator<T> for UnionSet<T> {
    fn from_iter<Iter: IntoIterator<Item=T>>(iter: Iter) -> Self {
        UnionSet {
            forest: iter
                .into_iter()
                .map(|value| (value.clone(), Node {
                    parent: Cell::new(std::ptr::null_mut()),
                    content: Value(value.clone()),
                }))
                .collect(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_disjoint() {
        let array = [1, 2, 3, 4, 5];
        let my_union = UnionSet::from_iter(array);
        println!("{:?}", my_union);
        let a: Vec<String> = Vec::new();
    }

    #[test]
    fn test_join() {
        let mut my_union = UnionSet::from_iter([1, 2, 3, 4, 5]);
        my_union.join(1, 2).unwrap();
        println!("{:?}", my_union);
    }
}
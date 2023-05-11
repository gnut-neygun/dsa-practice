use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::DerefMut;
use std::rc::{Rc, Weak};

use crate::safe_disjoint_union::Node::{Internal, Terminal};
use crate::safe_disjoint_union::UnionSetError::NotExist;

type NodeCell<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct UnionSet<T> {
    forest: HashMap<T, NodeCell<T>>,
}

#[derive(Debug)]
enum Node<T> {
    Terminal {
        parent: Option<NodeCell<T>>,
        value: T,
    },
    Internal {
        parent: Option<NodeCell<T>>,
        children: Vec<Weak<RefCell<Node<T>>>>,
    },
}

#[derive(Debug)]
enum UnionSetError {
    NotExist,
}

impl<T: Copy + Eq + Hash> UnionSet<T> {
    fn new() -> Self {
        UnionSet {
            forest: HashMap::new(),
        }
    }

    fn join(&mut self, a: T, b: T) -> Result<(), UnionSetError> {
        let node_a = self.forest.get(&a).ok_or(NotExist)?;
        let node_b = self.forest.get(&b).ok_or(NotExist)?;
        let parent: NodeCell<T> = Rc::new(RefCell::new(Internal {
            parent: None,
            children: vec![Rc::downgrade(node_a), Rc::downgrade(node_b)],
        }));
        if let Terminal { parent: node_parent, .. } = node_a.borrow_mut().deref_mut() {
            *node_parent = match node_parent {
                None => Some(parent.clone()),
                Some(old_parent) => unreachable!()
            };
        } else { unreachable!(); }
        if let Terminal { parent: node_parent, .. } = node_b.borrow_mut().deref_mut() {
            *node_parent = match node_parent {
                None => Some(parent.clone()),
                Some(old_parent) => unreachable!()
            };
        } else { unreachable!(); }
        Ok(())
    }
}

impl<T: Copy + Eq + Hash> FromIterator<T> for UnionSet<T> {
    fn from_iter<Iter: IntoIterator<Item=T>>(iter: Iter) -> Self {
        UnionSet {
            forest: iter
                .into_iter()
                .map(|value| (value, Rc::new(RefCell::new(Terminal {
                    parent: None,
                    value,
                }))))
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

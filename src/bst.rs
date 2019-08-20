use std::cmp::Ordering;
use std::fmt::Debug;

type Result<T> = std::result::Result<T, BstError>;
#[derive(Debug)]
pub enum BstError {
    DuplicateInsert,
    ValueNotFound,
}

#[derive(Debug)]
struct Node<T: Ord + Debug> {
    value: T,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

#[derive(Debug)]
pub struct BST<T: Ord + Debug> {
    root: Option<Node<T>>,
}

impl<T: Ord + Debug> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn new_boxed(value: T) -> Box<Self> {
        Box::new(Self::new(value))
    }

    fn print(&self) {
        if let Some(node) = &self.left {
            node.print();
        }
        print!("{:?}\n", self.value);
        if let Some(node) = &self.right {
            node.print();
        }
    }
}

impl<T: Ord + Debug> BST<T> {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }
    
    pub fn insert(&mut self, value: T) -> Result<()> {
        match &mut self.root {
            Some(node) => {
                let mut current = match node.value.cmp(&value) {
                    Ordering::Less => &mut node.right,
                    Ordering::Greater => &mut node.left,
                    Ordering::Equal => return Err(BstError::DuplicateInsert),
                };

                while let Some(node) = current {
                    current = match node.value.cmp(&value) {
                        Ordering::Less => &mut node.right,
                        Ordering::Greater => &mut node.left,
                        Ordering::Equal => return Err(BstError::DuplicateInsert),
                    }
                }
                *current = Some(Node::new_boxed(value));
            },
            None => self.root = Some(Node::new(value)),
        }

        Ok(())
    }

    pub fn delete(&mut self, value: &T) -> Result<T> {
        match &mut self.root {
            node @ Some(_) => {
                let mut current = match node.as_mut().unwrap().value.cmp(&value) {
                    Ordering::Less => &mut node.as_mut().unwrap().right,
                    Ordering::Greater => &mut node.as_mut().unwrap().left,
                    Ordering::Equal => {
                        let node_inner = node.as_mut().unwrap();
                        match (&mut node_inner.left, &mut node_inner.right) {
                            (None, None) => {
                                return Ok(node.take().unwrap().value)
                            },
                            (lnode @ Some(_), None) => {
                                let lnode = *lnode.take().unwrap();
                                return Ok(node.replace(lnode).unwrap().value)
                            },
                            (None, rnode @ Some(_)) => {
                                let rnode = *rnode.take().unwrap();
                                return Ok(node.replace(rnode).unwrap().value)
                            },
                            (Some(_), rnode @ Some(_)) => {
                                let mut snode = rnode;
                                while let Some(_) = snode.as_mut().unwrap().left {
                                    snode = &mut snode.as_mut().unwrap().left;
                                }

                                let mut snode_token = snode.take().unwrap();
                                if let Some(snode_child) = snode_token.right.take() {
                                    snode.replace(snode_child);
                                };
                                std::mem::swap(&mut node.as_mut().unwrap().value, &mut snode_token.value);
                                
                                return Ok(snode_token.value)
                            },
                        }
                    },
                };

                while let node @ Some(_) = current {
                    current = match node.as_mut().unwrap().value.cmp(&value) {
                        Ordering::Less => &mut node.as_mut().unwrap().right,
                        Ordering::Greater => &mut node.as_mut().unwrap().left,
                        Ordering::Equal => {
                            let node_inner = node.as_mut().unwrap();
                            match (&mut node_inner.left, &mut node_inner.right) {
                                (None, None) => {
                                    return Ok(node.take().unwrap().value)
                                },
                                (lnode @ Some(_), None) => {
                                    let lnode = lnode.take().unwrap();
                                    return Ok(node.replace(lnode).unwrap().value)
                                },
                                (None, rnode @ Some(_)) => {
                                    let rnode = rnode.take().unwrap();
                                    return Ok(node.replace(rnode).unwrap().value)
                                },
                                (Some(_), rnode @ Some(_)) => {
                                    let mut snode = rnode;
                                    while let Some(_) = snode.as_mut().unwrap().left {
                                        snode = &mut snode.as_mut().unwrap().left;
                                    }

                                    let mut snode_token = snode.take().unwrap();
                                    if let Some(snode_child) = snode_token.right.take() {
                                        snode.replace(snode_child);
                                    };
                                    std::mem::swap(&mut node.as_mut().unwrap().value, &mut snode_token.value);
                                    
                                    return Ok(snode_token.value)
                                },
                            }
                        },
                    };
                }
                
                Err(BstError::ValueNotFound)
            },
            None => Err(BstError::ValueNotFound),
        }
    }

    pub fn contains(&mut self, value: &T) -> bool {
        match &mut self.root {
            Some(node) => {
                let mut current = match node.value.cmp(&value) {
                    Ordering::Less => &mut node.right,
                    Ordering::Greater => &mut node.left,
                    Ordering::Equal => return true,
                };

                while let Some(node) = current {
                    current = match node.value.cmp(&value) {
                        Ordering::Less => &mut node.right,
                        Ordering::Greater => &mut node.left,
                        Ordering::Equal => return true,
                    }
                }
                
                false
            },
            None => false,
        }
    }

    

    pub fn check(&self) {
        match &self.root {
            Some(node) => node.print(),
            None => {},
        }
    }
}

#[cfg(test)] 
mod test { 
    use super::*;

    #[test]
    fn insert_three_numbers() -> Result<()> {
        let mut tree = BST::new();
        tree.insert(5)?;
        tree.insert(10)?;
        tree.insert(3)?;
        println!("{:#?}", tree);
        Ok(())
    }

    #[test]
    fn insert_five_numbers() -> Result<()>  {
        let mut tree = BST::new();
        tree.insert(5)?;
        tree.insert(10)?;
        tree.insert(3)?;
        tree.insert(4)?;
        tree.insert(9)?;
        println!("{:#?}", tree);
        Ok(())
    }

    #[test]
    fn insert_duplicate() -> Result<()>  {
        let mut tree = BST::new();
        tree.insert(5)?;
        match tree.insert(5) {
            Err(BstError::DuplicateInsert) => Ok(()),
            _ => panic!("Insert duplicate didn't fail."),
        }
    }

    #[test]
    fn search_five_numbers() -> Result<()>  {
        let mut tree = BST::new();
        tree.insert(5)?;
        tree.insert(10)?;
        tree.insert(3)?;
        tree.insert(4)?;
        tree.insert(9)?;

        assert!(tree.contains(&5));
        assert!(!tree.contains(&15));
        assert!(tree.contains(&5));
        assert!(tree.contains(&3));
        assert!(tree.contains(&10));
        assert!(tree.contains(&4));
        assert!(tree.contains(&9));
        assert!(!tree.contains(&192912));

        Ok(())
    }

    #[test]
    fn search_five_words() -> Result<()>  {
        let mut tree = BST::new();
        tree.insert("hello")?;
        tree.insert("my")?;
        tree.insert("beautiful")?;
        tree.insert("world")?;
        tree.insert("!")?;

        assert!(tree.contains(&"world"));
        assert!(!tree.contains(&"ugly"));
        assert!(!tree.contains(&"ugly"));
        assert!(tree.contains(&"hello"));
        assert!(tree.contains(&"my"));
        assert!(tree.contains(&"world"));
        assert!(tree.contains(&"!"));
        assert!(tree.contains(&"beautiful"));
        assert!(!tree.contains(&"banana"));

        Ok(())
    }

    #[test]
    fn insert_and_delete_one_word() -> Result<()>  {
        let mut tree = BST::new();
        assert!(!tree.contains(&"hello"));
        tree.insert("hello")?;
        assert!(tree.contains(&"hello"));
        tree.delete(&"hello")?;
        assert!(!tree.contains(&"hello"));

        
        println!("{:#?}", tree);
        Ok(())
    }

    #[test]
    fn insert_and_delete_five_numbers() -> Result<()>  {
        let mut tree = BST::new();

        // Inserts 10 and then deletes it.
        assert!(!tree.contains(&10));
        tree.insert(10)?;
        assert!(tree.contains(&10));
        tree.delete(&10)?;
        assert!(!tree.contains(&10));

        // Inserts 5 and then deletes it.
        assert!(!tree.contains(&5));
        tree.insert(5)?;
        assert!(tree.contains(&5));
        tree.delete(&5)?;
        assert!(!tree.contains(&5));

        // Inserts 7 and 8.
        assert!(!tree.contains(&7));
        tree.insert(7)?;
        assert!(!tree.contains(&8));
        tree.insert(8)?;
        
        // Checks if BST contains both 7 and 8.
        assert!(tree.contains(&7));
        assert!(tree.contains(&8));

        // Deletes 7 and checks if 7 was deleted and if BST contains 8.
        tree.delete(&7)?;
        assert!(!tree.contains(&7));
        assert!(tree.contains(&8));

        // Deletes 8 and checks if 8 was deleted.
        tree.delete(&8)?;
        assert!(!tree.contains(&8));

        Ok(())
    }

    #[test]
    fn delete_numbers() -> Result<()>  {
        let mut tree = BST::new();

        tree.insert(100)?;
        tree.insert(50)?;
        tree.insert(70)?;
        tree.insert(80)?;
        tree.insert(200)?;
        tree.insert(68)?;
        tree.insert(60)?;
        tree.insert(59)?;
        tree.insert(54)?;
        tree.insert(55)?;
        tree.insert(30)?;

        assert!(tree.contains(&50));
        println!("BEFORE {:#?}", tree);
        tree.delete(&50)?;
        println!("AFTER {:#?}", tree);
        assert!(!tree.contains(&50));

        Ok(())
    }

    #[test]
    fn random_insert_1000000_numbers() -> Result<()>  {
        let mut tree = BST::new();

        // seed of CONG generator
        let mut jcong = 380_116_160i64;

        for _ in 0..1_000_000 {
            tree.insert(jcong)?; // Inserts random number.

            jcong = jcong.overflowing_mul(69_069).0.overflowing_add(1_234_567).0;
        }

        Ok(())
    }
}

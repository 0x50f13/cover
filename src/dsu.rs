use std::collections::LinkedList;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::{RefCell, Ref};

fn try_unwrap_option<T>(opt: &Option<T>) -> Result<&T, &'static str>{
   match opt{
     None => Err("Unwrapping Option which is None"),
     Some(object) => Ok(object),
   }
}

pub type DSUNodeRef<T> = Rc<RefCell<DSUNode<T>>>;

pub struct DSUNode<T>{
    parent: Option<DSUNodeRef<T>>,
    pub object: T,
    _id: usize,
    pub children: LinkedList<DSUNodeRef<T>>,
    pub marked: bool,
}

impl<T> DSUNode::<T>{
    pub fn new(_object: T, __id: usize) -> DSUNode<T>{
	DSUNode::<T>{
      	  parent: None,
          object: _object,
          _id: __id,
          children: LinkedList::new(),
          marked: false,
        }
    }
    pub fn owner(&self) -> Option<&DSUNodeRef<T>>{
        if !self.has_parent(){
           return None;
        }
        let mut node_ref = try_unwrap_option(&self.parent).unwrap();
        while(true){
            let node=node_ref.borrow();
            if !node.has_parent(){
               return Some(node_ref);
            }
            node_ref=try_unwrap_option(&self.parent).unwrap();
        }
        panic!("Bad DSU");
        None        
    }
    pub fn has_parent(&self) -> bool{
        match &self.parent {
              None => false,
              Some(_) => true
        }
    }
    pub fn mark(&mut self){
        self.marked=true;
    }
}

pub struct DSU<T>{
    pub nodes: Vec<DSUNodeRef<T>>,
    pub size: usize,
}

impl<T> DSU::<T>{
    pub fn new()->DSU<T>{
        DSU{
         nodes: Vec::new(),
         size: 0,
        }
    }
    pub fn add_node(&mut self, object: T)->DSUNodeRef<T>{
        let mut node_ref=Rc::new(RefCell::new(DSUNode::<T>::new(object,self.size)));
        self.nodes.push(node_ref.clone());
        self.size+=1;
        node_ref
   }
   pub fn unite(&self,_node: DSUNodeRef<T>, _owner: DSUNodeRef<T>){
       let mut node=_node.borrow_mut();
       let mut owner=_owner.borrow_mut();
       owner.children.append(&mut node.children);
       owner.children.push_back(_node.clone());
       node.children.clear();
       node.parent=Some(_owner.clone());
   }
   
}

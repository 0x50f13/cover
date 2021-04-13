use std::collections::LinkedList;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::{RefCell, Ref};

fn try_unwrap_option<T>(opt: Option<T>) -> Result<T, &'static str>{
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
    pub fn owner(&self) -> Option<DSUNodeRef<T>>{
        if !self.has_parent(){
            return None
        }
        let node_ref = try_unwrap_option(self.parent.clone()).unwrap().clone();
        let ref2 = node_ref.clone();
        let node=ref2.borrow();
        //println!("New node id:{d}",d=node._id);
        if !node.has_parent(){
            return Some(node_ref.clone());
        }
        node.owner()
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
       //println!("{d1}-->{d2}",d1=_node.borrow()._id, d2=_owner.borrow()._id);
       if(_node.borrow()._id==_owner.borrow()._id){
           //Already united
           //Just ignore the request
           return ;
       }
       let mut t_node=match _node.borrow().owner(){
            None=> _node.clone(),
            Some(__node)=>__node.clone(),
       };
       //println!("slave={d1}",d1=t_node.borrow()._id);
       let mut t_owner=match _owner.borrow().owner(){
           None=> _owner.clone(),
           Some(__owner) => __owner.clone(),
       };
       //println!("owner={d1}",d1=t_owner.borrow()._id);
       let mut r_node=t_node.try_borrow_mut();
       let mut r_owner=t_owner.try_borrow_mut();
       if r_node.is_err(){
           return ;
       }
       if r_owner.is_err(){
           return ;
       }
       let mut owner=r_owner.unwrap();
       let mut node=r_node.unwrap();
       owner.children.append(&mut node.children);
       owner.children.push_back(_node.clone());
       node.children.clear();
       node.parent=Some(t_owner.clone());
   }
   
}

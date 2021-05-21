use crate::point2d;
use crate::dsu;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::{RefCell, Ref};

type GraphNodeRef = Rc<RefCell<GraphNode>>;

pub struct Edge{
    pub src: GraphNodeRef,
    pub dst: GraphNodeRef,
    pub dist: f64,
}

pub struct GraphNode{
    pub edges: Vec<Edge>,
    pub pos: point2d::Point3D,
    pub dsu_ref: dsu::DSUNodeRef<point2d::Point3D>,
}

pub struct Graph{
    pub nodes: Vec<GraphNodeRef>,
    pub map: HashMap<point2d::Point3D, GraphNodeRef>,
    pub _dsu: dsu::DSU::<point2d::Point3D>,
}

impl Edge{
    pub fn new(_src: GraphNodeRef,_dst: GraphNodeRef, _dist: f64) -> Edge{
        Edge{
           src: _src,
           dst: _dst,
           dist: _dist,
        }
    }
}
            

impl GraphNode{
    pub fn new(point: point2d::Point3D, _dsu_ref: dsu::DSUNodeRef<point2d::Point3D>) -> GraphNode{
        GraphNode{
            edges: Vec::new(),
            pos: point,
            dsu_ref: _dsu_ref,
        }
    }
}

impl Graph{
    pub fn new()-> Graph {
       Graph{
         nodes: Vec::<GraphNodeRef>::new(),
         map: HashMap::new(),
         _dsu: dsu::DSU::<point2d::Point3D>::new(),
       }
    }
    pub fn add_node(&mut self,_x: f64, _y: f64, _t: f64){
        let pos = point2d::Point3D::new(_x,_y,_t);
        let node = Rc::new(RefCell::new(GraphNode::new(pos.copy(),self._dsu.add_node(pos.copy()))));
        let node_clone = node.clone();
        self.map.insert(pos, node_clone);
        self.nodes.push(node);
    }
    pub fn reset_dsu(&mut self){
        self._dsu=dsu::DSU::<point2d::Point3D>::new();
        println!("Resetted dsu, n={d}",d=self._dsu.nodes.len());
        for node_ref in &self.nodes{
            let mut node = node_ref.borrow_mut();
            node.dsu_ref = self._dsu.add_node(node.pos.copy());
         }
     }
}


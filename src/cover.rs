use crate::ui;
use crate::graph;
use crate::point2d;
use crate::dsu;
use crate::csv;
use std::cell::{RefCell, Ref};

fn try_unwrap_option<T>(opt: &Option<T>) -> Result<&T, &'static str>{
   match opt{
     None => Err("Unwrapping Option which is None"),
     Some(object) => Ok(object),
   }
}

fn bad_float(s: &str) -> f64{
   panic!("Bad float {f}", f=s);
   0.0
}
fn str2f64(s: &str) -> f64{
    match s.parse::<f64>(){
        Ok(v) => v,
        Err(_) => bad_float(s),
    }
}

pub fn build_eps_graph(eps: f64, _graph: &graph::Graph){
    let mut graph=&*_graph;
    let mut n_done=0;
    let mut n_counted=0.0;
    let mut n_edges=0;
    let mut s=0.0;
    //let graph = RefCell::new(graph);
    //let graph = graph.into_inner();
    for x_ref in &graph.nodes {
        let mut x=x_ref.borrow_mut();
        for y_ref in &graph.nodes {
            let y=y_ref.try_borrow().ok();
            match y {
                None => continue,
                Some(v) => {
                    n_counted+=1.0;
                    let dist = point2d::rho(&x.pos, &v.pos);
                    if dist<eps{
                        n_edges+=1;
                        let edge=graph::Edge::new((*x_ref).clone(),(*y_ref).clone(),dist);
                        x.edges.push(edge);
                    }
                    s+=dist;
                    //print!(" s={s}",s=s);
                }
            }
        }
        n_done+=1;
        ui::print_pb("Building epsilon-cover graph".to_string(),n_done,graph.nodes.len() as i32); 
    }
    println!("");
    println!("  -Iterations(should be N^2-2*N): {n}",n=n_counted);
    println!("  -Average distance: {s}",s=s/n_counted);
    println!("  -Number of edges: {n}",n=n_edges);
}

pub fn eps_graph_verify(graph: &graph::Graph){
    let mut n_processed=0;
    for x_ref in &graph.nodes{
        let x=x_ref.borrow();
        if(x.edges.len()==0){
            println!("Failing on vertex n={n}",n=n_processed);
            panic!("Isolated vertices found!");
        }
        n_processed+=1;
    }
}

pub fn build_cover(_graph: &mut graph::Graph, data: &csv::CsvData, _n: usize, mu: f64){
    _graph.reset_dsu();
    let n = data.lines.len();
    let mut n_processed = 0;
    let mut n_unions=0;
    let mut n_edges=0;
    let mut n_marked=0;
    for line in &data.lines {
        let activation = str2f64(&line.values[_n]);
        if activation>mu {
//           println!("{d1}>{d2}",d1=activation,d2=mu);
           let x=str2f64(&line.values[4]);
           let y=str2f64(&line.values[5]);
           let point = point2d::Point2D::new(x,y);
           let mut node = try_unwrap_option(&_graph.map.get(&point)).unwrap().borrow_mut();
           let mut dsu_node = node.dsu_ref.borrow_mut();
           dsu_node.mark();
           n_marked+=1;
        }
    }
    for line in &data.lines {
        let activation = str2f64(&line.values[_n]);
        if activation>mu {
           let x=str2f64(&line.values[4]);
           let y=str2f64(&line.values[5]);
           let point = point2d::Point2D::new(x,y);
           let mut node = try_unwrap_option(&_graph.map.get(&point)).unwrap().borrow_mut();
           for edge in &node.edges {
               let neighbour = edge.dst.borrow();
               let dsu_node=neighbour.dsu_ref.borrow();
               n_edges+=1;
               if dsu_node.marked {
                  drop(dsu_node);
                  n_unions+=1;
                  _graph._dsu.unite(neighbour.dsu_ref.clone(), node.dsu_ref.clone());
               }
           }
        }
        n_processed+=1;
        ui::print_pb("Building cover              ".to_string(), n_processed, n as i32);
    }
    println!("");
    println!("  -Activation points number: {d}", d=n_marked);
    println!("  -Edges processed: {d}", d=n_edges);
    println!("  -Set unions performed: {d}", d=n_unions);
}

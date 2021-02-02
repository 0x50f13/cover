use std::cell::{RefCell, Ref};

mod graph;
mod csv;
mod ui;
mod point2d;
mod cover;
mod dsu;

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
fn main() {
    println!("CoverSolver v1.0");
    let mut _dsu = dsu::DSU::<point2d::Point2D>::new();
    let mut _graph = graph::Graph::new();
    let data = csv::read_csv("data/22ht1.csv".to_string());
    let mut n_processed = 0;
    //let mut v = Vec::new();
    for line in &data.lines {
        let x=str2f64(&line.values[4]);
        let y=str2f64(&line.values[5]);
        _graph.add_node(x,y);
        n_processed+=1;
        ui::print_pb("Adding points               ".to_string(), n_processed, data.n_lines-1);
        //n_processed+=1
    }

    //_graph.reset_dsu();

    println!("");
    
    cover::build_eps_graph(50.0, &_graph);
    cover::eps_graph_verify(&_graph);
    cover::build_cover(&mut _graph, &data, 6, 0.0);
}

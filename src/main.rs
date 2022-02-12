use std::cell::{RefCell, Ref};
use fork::{daemon, Fork};
use std::process::Command;

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
    /*******CONFIG*******/
    let EPS=40.0;
    let MU=0.4;
    let N_COLUMN=7;
    let DATA="data/22ht1_normalized.csv";
    let OUTPUT_FOLDER="output/";
    /********************/
    println!("CoverSolver v1.0");
    let mut _dsu = dsu::DSU::<point2d::Point3D>::new();
    let mut _graph = graph::Graph::new();
    let data = csv::read_csv(DATA.to_string());
    let mut n_processed = 0;
    //let mut v = Vec::new();
    for line in &data.lines {
        let x=str2f64(&line.values[5]);
        let y=str2f64(&line.values[6]);
        let t=f64::from(line.n);
//        println!("y={d}",d=y);
        _graph.add_node(x,y,t);
        n_processed+=1;
        ui::print_pb("Adding points               ".to_string(), n_processed, data.n_lines-1);
        //n_processed+=1
    }

    //_graph.reset_dsu();

    println!("");
    
    cover::build_eps_graph(EPS, &_graph);
    cover::eps_graph_verify(&_graph);


    for i in N_COLUMN..data.header.len(){
        println!("Processing column {d}", d=i-N_COLUMN);
        //Delegate copy of graph to all processes as it
        //would be updated while working with data
        let mut d_graph = _graph.clone();
        if let Ok(Fork::Child) = daemon(false, false){
           cover::build_cover(&mut d_graph, &data, i, MU);
           let fname=OUTPUT_FOLDER.to_string()+&i.to_string()+&".txt".to_string();
           cover::export_cover(&_graph,&fname,&data,i);
        }
    }
}

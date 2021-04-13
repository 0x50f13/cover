//Console User Interface

pub fn print_pb(title: String, done: i32, total: i32){
    print!("\r {title} ",title=title);
    print!("[");
    let _done = done as f64;
    let _total = total as f64;
    let n_bars=((_done/_total)*50.0) as i64;
    for _ in 0..n_bars{
        print!("=");
    }
    for _ in 0..(50-n_bars){
        print!(" ");
    }
    print!("] {done}/{total}", done=done, total=total);
}



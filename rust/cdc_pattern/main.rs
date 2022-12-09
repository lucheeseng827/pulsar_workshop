use actix::prelude::*;

struct DataSource {
    data: Vec<i32>,
}

impl Actor for DataSource {
    type Context = Context<Self>;
}

struct Observer {
    data_source: Addr<DataSource>,
}

impl Actor for Observer {
    type Context = Context<Self>;
}

fn main() {
    let data_source = DataSource { data: vec![1, 2, 3] };
    let observer = Observer { data_source: data_source.start() };

    // Register the observer as a listener for changes in the data source
    data_source.do_send(AddObserver(observer.start()));
}

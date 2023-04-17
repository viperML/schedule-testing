use color_eyre::Result;
use daggy::{Dag, Walker};
use daggy::petgraph::dot::{Dot, Config};

#[derive(Copy, Clone, Debug)]
struct Buildable<'a>(&'a str);

fn main() -> Result<()> {
    let dag = &mut Dag::<Buildable, ()>::new();

    let parent = dag.add_node(Buildable("/miq/bootstrap"));

    let (a1, b1) = dag.add_child(parent, (), Buildable("/miq/gcc"));
    let (a2, b2) = dag.add_child(parent, (), Buildable("/miq/coreutils"));

    let b3 = dag.add_node(Buildable("/miq/glibc"));
    dag.add_edge(b1, b3, ())?;
    dag.add_edge(b2, b3, ())?;

    let dag = &*dag;
    eprintln!("{:?}", dag);

    let dot = Dot::with_config(dag, &[Config::EdgeNoLabel]);
    println!("{:?}", dot);


    for elem in dag.children(parent).iter(&dag) {
        eprintln!("{:?}", elem);
    }


    Ok(())
}

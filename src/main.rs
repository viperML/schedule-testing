use std::fmt::write;

use color_eyre::Result;
use daggy::petgraph::algo::toposort;
use daggy::petgraph::algo::tred::{
    dag_to_toposorted_adjacency_list, dag_transitive_reduction_closure,
};
use daggy::petgraph::graph::Node;
use daggy::petgraph::visit::{Bfs, Topo};
use derive_more::Display;

use daggy::petgraph;
use daggy::petgraph::dot::{Config, Dot};
use daggy::{Dag, NodeIndex, Walker};
use tracing::{info, warn};

#[derive(Copy, Clone, Debug)]
struct Buildable<'a>(&'a str);

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Display, PartialEq)]
#[display(fmt = "{}", name)]
struct Pkg {
    name: String,
    deps: Vec<String>,
}

impl Pkg {
    fn new<P: AsRef<str>>(pkg: P) -> Result<Self> {
        let filename = format!("./pkgs/{}.toml", pkg.as_ref());
        let contents = std::fs::read_to_string(filename)?;
        let parsed: Self = toml::from_str(&contents)?;

        Ok(parsed)
    }
}

#[derive(Clone)]
struct PkgNode {
    inner: Pkg,
    visited: bool,
}

impl std::fmt::Debug for PkgNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.inner.name)
    }
}

impl PkgNode {
    fn new<P: AsRef<str>>(pkg: P) -> Result<Self> {
        Ok(Self {
            inner: Pkg::new(pkg)?,
            visited: false,
        })
    }

    fn visit(&mut self) {
        self.visited = true;
    }
}

type PkgDag = Dag<PkgNode, ()>;

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .compact()
        .with_writer(std::io::stderr)
        .without_time()
        .init();

    let mut dag = PkgDag::new();
    let root_node = dag.add_node(PkgNode::new("libc")?);
    info!(?dag);

    let max_cycles = 10;
    let mut cycle = 0;
    let mut size: usize = 1;

    while size > 0 && cycle <= max_cycles {
        let old_dag = dag.clone();
        let search = petgraph::visit::Dfs::new(&old_dag, root_node);

        cycle_dag(&mut dag, root_node)?;

        size = search
            .iter(&old_dag)
            .fold(0, |acc, n| if !old_dag[n].visited { acc + 1 } else { acc });
        info!(?size);

        cycle = cycle + 1;
    }

    println!("{:?}", Dot::with_config(&dag, &[Config::EdgeNoLabel],));

    let toposort = toposort(&dag, None).unwrap();
    let sort = toposort
        .iter()
        .map(|&n| dag.node_weight(n).unwrap())
        .rev()
        .collect::<Vec<_>>();
    info!(?sort);

    Ok(())
}

fn cycle_dag(dag: &mut PkgDag, node: NodeIndex) -> Result<()> {
    let old_dag = dag.clone();
    info!("Cycling at node {:?}", old_dag[node]);

    create_childen(dag, node)?;

    for (_, child) in old_dag.children(node).iter(&old_dag) {
        cycle_dag(dag, child)?;
    }

    Ok(())
}

fn create_childen(dag: &mut PkgDag, node: NodeIndex) -> Result<()> {
    let old_dag = dag.clone();

    if dag[node].visited {
        return Ok(());
    }

    info!("Creating children at: {:?}", old_dag[node]);

    dag[node].visit();

    for elem in &old_dag.node_weight(node).unwrap().inner.deps {
        info!("I want to create {:?}", elem);

        let target_child = Pkg::new(elem)?;

        for child in Topo::new(&old_dag).iter(&old_dag) {
            let p = &old_dag[child].inner;
            warn!("Examining G component {:?}", p);

            if p == &target_child.clone() {
                dag.add_edge(node, child, ())?;

                return Ok(());
            }
        }

        dag.add_child(
            node,
            (),
            PkgNode {
                inner: target_child,
                visited: false,
            },
        );
    }

    Ok(())
}

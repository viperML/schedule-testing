use dagga::Node;

use color_eyre::Result;

fn main() -> Result<()> {
    let dag = dagga::Dag::<(), &str>::default()
        .with_node(
            Node::new(())
                .with_name("/miq/store/AAAB")
                .with_result("/miq/store/AAAB")
                .with_read("/miq/store/AAAA"),
        )
        .with_node(
            Node::new(())
                .with_name("/miq/store/AAAA")
                .with_result("/miq/store/AAAA"), // .with_read("/miq/store/AAAB")
        )
        .with_node(
            Node::new(())
                .with_name("/miq/store/AAAD")
                .with_result("/miq/store/AAAD")
                .with_read("/miq/store/AAAA"),
        )
        .with_node(
            Node::new(())
                .with_name("/miq/store/AAAE")
                .with_result("/miq/store/AAAE")
                .with_read("/miq/store/AAAD")
                .with_read("/miq/store/AAAB"),
        );

    // let result = dag.build_schedule()?;
    let sched = dag.build_schedule()?;
    let batches: Vec<Node<_, _>> = sched.batches.into_iter().flat_map(|x| x).collect();
    eprintln!("{:?}", batches);

    let result = dagga::dot::DagLegend::new(batches.iter());
    dagga::dot::save_as_dot(&result, "./main")?;

    Ok(())
}

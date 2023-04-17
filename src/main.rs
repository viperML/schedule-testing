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
    let result = sched.batched_names();

    eprintln!("{:?}", result);

    let result2: Vec<&str> = result.into_iter().flat_map(|x| x).collect();

    eprintln!("{:?}", result2);

    Ok(())
}

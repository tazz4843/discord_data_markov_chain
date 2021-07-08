use markov::Chain;
use pbr::ProgressBar;

pub fn create_and_feed_chain(items: Vec<String>) -> Chain<String> {
    let mut chain = Chain::of_order(1);
    let mut pb = ProgressBar::new(items.len() as u64);

    for item in items {
        chain.feed_str(item.as_str());
        pb.inc();
    }
    pb.finish();

    chain
}

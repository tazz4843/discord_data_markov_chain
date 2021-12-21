use markov::Chain;
use pbr::ProgressBar;

pub fn create_and_feed_chain(items: Vec<String>, to_skip: Vec<&str>) -> Chain<String> {
    let mut chain = Chain::of_order(1);
    let mut pb = ProgressBar::new(items.len() as u64);

    'outer: for item in items {
        pb.inc();

        for item_skip in to_skip.iter() {
            if item.contains(item_skip) {
                continue 'outer;
            }
        }

        chain.feed_str(item.as_str());
    }
    pb.finish();

    chain
}

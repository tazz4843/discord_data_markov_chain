use crate::chain::create_and_feed_chain;
use crate::messages::get_messages_from_dir;
use std::path::Path;

pub fn entrypoint(path: &Path, count: u32, to_skip: Vec<&str>) {
    let items = get_messages_from_dir(&path);

    println!("found {} messages", items.len());

    let chain = create_and_feed_chain(items, to_skip);

    for _ in 0..count {
        println!("{}\n", chain.generate_str());
    }
}

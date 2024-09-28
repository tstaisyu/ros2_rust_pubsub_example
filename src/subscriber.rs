use rclrust::{rclrust_info, Node};

fn main() {
    let ctx = rclrust::init().unwrap();
    let node = Node::new(ctx, "rust_subscriber").unwrap();
    let subscription = node.create_subscription::<std_msgs::msg::String>(
        "topic", rclrust::QOS_PROFILE_DEFAULT, |msg| {
            rclrust_info!(node, "I heard: '{}'", msg.data);
        },
    ).unwrap();

    node.spin().unwrap();
}


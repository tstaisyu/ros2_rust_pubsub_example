use rclrust::{rclrust_info, Node};

fn main() {
    let ctx = rclrust::init().unwrap();
    let node = Node::new(ctx, "rust_publisher").unwrap();
    let publisher = node.create_publisher::<std_msgs::msg::String>("topic", rclrust::QOS_PROFILE_DEFAULT).unwrap();

    let mut count = 0;
    node.spin_once(move || {
        let mut msg = std_msgs::msg::String::default();
        msg.data = format!("Hello, world! {}", count);
        publisher.publish(msg).unwrap();
        count += 1;
    }).unwrap();
}


mod publisher;
mod subscriber;

fn main() {
    std::thread::spawn(|| {
        publisher::publish();
    });

    subscriber::subscribe();
}


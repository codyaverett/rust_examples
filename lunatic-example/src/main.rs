use lunatic::{Channel, Process};

fn main() {
    let channel: Channel<()> = Channel::new(0);

    for _ in 0..20_000 {
        Process::spawn((), process).unwrap();
    }

    channel.receive();
}

fn process(_: ()) {
    loop {
        Process::sleep(100);
    }
}

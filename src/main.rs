fn main() {}

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::{thread, time};

    #[test]
    fn spawn_zenoh_router() {
        let mut child = Command::new("zenohd").spawn().unwrap();
        thread::sleep(time::Duration::from_secs(1));
        child.kill().unwrap();
    }
}

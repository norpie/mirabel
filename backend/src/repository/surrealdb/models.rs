use serde::Deserialize;

#[derive(Deserialize)]
struct SDBCount(Vec<Count>);

impl SDBCount {
    fn count(&self) -> u64 {
        self.0.first().map_or(0, |c| c.count)
    }
}

#[derive(Deserialize)]
struct Count {
    count: u64,
}

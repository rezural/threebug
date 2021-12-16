use serde::*;

pub mod ipc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: u64,
}

impl From<u64> for Entity {
    fn from(bits: u64) -> Self {
        Self { id: bits }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use async_trait::async_trait;

#[async_trait]
pub trait Sensor {
    async fn read(&self) -> Vec<u8>;
}

#[async_trait]
pub trait Actuator {
    async fn execute(&self, command: Vec<u8>) -> bool;
}

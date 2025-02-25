use crate::execution_client::execution_client::ExecutionClient;

pub trait Strategy {
    fn client(&self) -> ExecutionClient;
}

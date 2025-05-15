use sc_service::{Configuration, TaskManager, TFullClient};
use bitstream_runtime::RuntimeApi;

pub async fn new_full(
    config: Configuration,
) -> sc_service::error::Result<(TaskManager, TFullClient)> {
    sc_service::build_full(config).await
}

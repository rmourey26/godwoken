use anyhow::Result;

use crate::godwoken_rpc::GodwokenRpcClient;

use std::{fs::write, path::Path};

pub fn dump_mem_block(godwoken_rpc_url: &str, output: &Path) -> Result<()> {
    let mut godwoken_rpc_client = GodwokenRpcClient::new(godwoken_rpc_url);
    let mem_block = godwoken_rpc_client.dump_mem_block()?;
    write(output, mem_block)?;
    Ok(())
}

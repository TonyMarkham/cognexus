use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};

pub struct PluginState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl PluginState {
    pub fn new() -> Self {
        Self {
            ctx: WasiCtx::builder().build(),
            table: ResourceTable::new(),
        }
    }
}

impl WasiView for PluginState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}

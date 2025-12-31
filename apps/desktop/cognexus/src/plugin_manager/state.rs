//! WASI state for plugin execution.

use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};

pub struct State {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl State {
    pub fn new() -> Self {
        Self{
            ctx: WasiCtx::builder().build(),
            table: ResourceTable::new(),
        }
    }
}

impl WasiView for State {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView{
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }    
}

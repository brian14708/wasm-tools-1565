use std::error::Error;

use wasmtime::{
    component::{bindgen, Component, Resource},
    Config, Engine, Store,
};
use wasmtime_wasi::{async_trait, ResourceTable, Subscribe, WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    world: "my-world",
    path: "../wit",
    async: true,
    with: {
        "wasi": wasmtime_wasi::bindings,
    }
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut cfg = Config::new();
    cfg.async_support(true);
    let engine = Engine::new(&cfg).unwrap();
    let file = std::env::args().nth(1).ok_or("Missing file argument")?;
    let component = Component::from_file(&engine, file)?;

    let mut store = Store::new(
        &engine,
        MyState {
            ctx: WasiCtxBuilder::new().inherit_stdout().build(),
            table: ResourceTable::new(),
        },
    );

    let mut linker = wasmtime::component::Linker::new(&engine);
    wasmtime_wasi::add_to_linker_async(&mut linker)?;
    fn get_host(state: &mut MyState) -> &mut MyState {
        state
    }
    MyWorld::add_to_linker_imports_get_host(&mut linker, get_host)?;

    let instance = linker.instantiate_async(&mut store, &component).await?;
    let world = MyWorld::new(&mut store, &instance)?;
    let res = world.call_run(&mut store).await?;
    println!("res: {}", res);

    Ok(())
}

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

#[async_trait]
impl MyWorldImports for MyState {
    async fn my_sleep(&mut self) -> Resource<Pollable> {
        let res = self.table.push(MySleep).unwrap();
        wasmtime_wasi::subscribe(&mut self.table, res).unwrap()
    }
}

struct MySleep;

#[async_trait]
impl Subscribe for MySleep {
    async fn ready(&mut self) {
        println!("in ready");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("in ready after sleep");
    }
}

use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("TornadoRouter", "abis/TornadoRouter.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("src/abi/TornadoRouter.rs")
}

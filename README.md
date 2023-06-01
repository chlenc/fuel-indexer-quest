# Fuel Indexer Quest

This repository contains an example project demonstrating the usage of the Fuel Indexer. The project consists of two folders: `contract` and `hello_world`.

## Getting Started

Follow the steps below to get started with the Fuel Indexer Quest.

### 1. Clone the repository

Clone the repository to your local machine and navigate to the project folder:

```shell
git clone https://github.com/chlenc/fuel-indexer-quest.git
cd fuel-indexer-quest
```

### 2. Build and deploy the indexer

In this step, you will build and deploy the Fuel Indexer. Perform the following actions:

- Switch to version 0.15.0 of the indexer.
- Run a Postgres container:

```shell
docker run -d -p 5432:5432 --name my-postgres -e POSTGRES_PASSWORD=mysecretpassword postgres
```

- Start the Fuel Indexer service, connecting it to the beta-3 network:

```shell
fuel-indexer run --run-migrations --fuel-node-host beta-3.fuel.network --fuel-node-port 80 --postgres-host 127.0.0.1 --postgres-port 5432 --postgres-password mysecretpassword --postgres-user postgres
```

- Deploy the indexer by opening a new terminal tab in the `hello_world` directory:

```shell
cd ./hello_world
# You should change `start_block` in your `spark_indexer.manifest.yaml` to current not to wait too long
# You can find the current block number on the response of the command below:
# curl -X POST https://beta-3.fuel.network/graphql -d '{"query":"query {blocks(last: 1) { nodes {header {height}}}}"}' -H 'Content-Type: application/json'
forc index build
# If you encounter any build errors, try running the following two lines:
# export AR=/opt/homebrew/opt/llvm/bin/llvm-ar 
# export CC=/opt/homebrew/opt/llvm/bin/clang
forc index deploy
```

### 3. Call the contract's function

In this step, you will call the contract's function. Follow these steps:

- Open a new terminal tab in the `contract` directory:

```shell
cd contract
echo "SECRET=<YOUR PRIVATE KEY>" >> .env
forc build
cargo test
```

That's it! You have successfully set up and executed the Fuel Indexer Quest.

For more information and detailed documentation, please refer to the Fuel Indexer's official documentation.

## Resources

- [Fuel Indexer Documentation](https://fuellabs.github.io/fuel-indexer)

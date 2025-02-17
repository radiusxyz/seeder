# Seeder

:warning: Under Construction
> This crate is actively being developed. Breaking changes will occur until mainnet when we will start [Semantic Versioning](https://semver.org/).

Liveness component of [Radius Block Building Solution](https://github.com/radiusxyz/radius-docs-bbs/blob/main/docs/radius_block_building_solution.md) written in Rust programming language.

Seeder is responsible for managing the tx_orderer RPC URL. As a simple key-value storage, it stores the tx_orderer address as a key and corresponding RPC URLs as a value. A tx_orderer who wishes to register on the Seeder must be registered on Liveness Contract prior to registering on the Seeder.

## Registration
When registering, a tx_orderer sends a signed message of its address, external RPC URL, cluster RPC URL and the cluster ID it receives as a result of registering on Liveness contract. 

> External RPC URL handles user transactions, whereas cluster RPC URL is only for inter-cluster messages whose signature must be verified.

When Seeder receives the registration request, it first verifies the message signature, proceeds to check if the address is registered on Liveness Contract and finally, checks if the external RPC URL of the requesting tx_orderer is accessible via '/health' endpoint. Only after these procedures a tx_orderer address and its RPC URLs are registered on Seeder and become available for other entities such as Secure RPC, TxOrderer and Rollups.

## Deregistration
A tx_orderer sends a signed message of its address of its address and the cluster ID it belongs to. Seeder verifies the message signature and checks if the address is deregistered from Liveness Contract. Seeder simply removes the address of the tx_orderer, making its RPC URLs unavailable afterwards.

## Contributing
We appreciate your contributions to our project. Visit [issues](https://github.com/radiusxyz/seeder/issues) page to start with or refer to the [Contributing guide](https://github.com/radiusxyz/radius-docs-bbs/blob/main/docs/contributing_guide.md).

## Getting Help
Our developers are willing to answer your questions. If you are first and bewildered, refer to the [Getting Help](https://github.com/radiusxyz/radius-docs-bbs/blob/main/docs/getting_help.md) page.

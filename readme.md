<div align="center">
	<img src="https://raw.githubusercontent.com/sandipndev/bdk-api/main/static/banner.png"/>
</div>

# Bitcoin Development Kit - API

### Objective

This project aims at tech-savvy users new to Bitcoin get a feel of how wallets, transactions and mining work in Bitcoin.
We use a local [bitcoind regtest blockchain](https://developer.bitcoin.org/examples/testing.html#regtest-mode) for every deployment with the ability to mine a new block at will. 
There are API endpoints that talk with [BitcoinDevKit's bdk](https://bitcoindevkit.org) library in order to create xprvs, sync and broadcast transactions.
You can create multiple master xprvs and with their mnemonics, restore them back.
You can also get coins during mining the same. It all happens in one instance.

### Quirks

1. Our code ensures no data is saved in the servers but to ensure full privacy, [self-deployment](#deployment) is better.
2. Please note that a centralized API is highly discouraged and should never be used with Bitcoin.

### Deployment

<!-- TODO -->

### License

This project is released under [MIT License](https://opensource.org/licenses/MIT).

This project was started by [sandipndev](https://sandipan.dev) during [Summer of Bitcoin](https://summerofbitcoin.org), special thanks to his mentor [Steve Myers](https://notmandatory.org) for the motivation in making this project happen.

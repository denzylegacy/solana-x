# Solana X
Decentralized Social Platform
<div align="center">
  <img src="docs/assets/solana-x_logo-rounded.png" alt="Yoruichi Logo" width="200" style="border-radius: 50%; object-fit: cover;"/>

  # Solana X
  
  [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

  A powerful and flexible centralized trading system for automated cryptocurrency trading.
</div>

A decentralized social platform built on Solana blockchain, implementing a simplified version of Twitter functionality as a dApp (decentralized application).

## Features

- Create, read, update and delete tweets on Solana blockchain
- Connect with Solana wallets
- Paginated tweet feeds
- Real-time blockchain interactions
- Decentralized data storage

## Tech Stack

[![Vue.js](https://img.shields.io/badge/Vue.js-v3.x-4FC08D?style=for-the-badge&logo=vue.js&logoColor=white)](https://vuejs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-v4.x-3178C6?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![JavaScript](https://img.shields.io/badge/JavaScript-ES6+-F7DF1E?style=for-the-badge&logo=javascript&logoColor=black)](https://developer.mozilla.org/en-US/docs/Web/JavaScript)
[![Rust](https://img.shields.io/badge/Rust-1.69+-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Solana](https://img.shields.io/badge/Solana-Latest-14F195?style=for-the-badge&logo=solana&logoColor=white)](https://solana.com/)

## Development Environment

> The Solana development environment is more tailored for Linux and Mac users. So, if you're using Windows, please follow [this guide for setting up Solana on Windows](https://github.com/buildspace/buildspace-projects/blob/main/Solana_And_Web3/en/Section_2/Resources/windows_setup.md) which uses Windows Subsystem for Linux (WSL).

## Development Cycle

To develop and test the application, follow these steps:

1. Start the local Solana validator:
   ```shell
   solana-test-validator
   ```

2. In a separate terminal session, build the project:
   ```shell
   anchor build
   ```

3. Deploy to the local network:
   ```shell
   anchor deploy
   ```

4. Run tests:
   ```shell
   anchor run test
   ```

## Anchor Commands

```shell
anchor test
```

The `anchor test` command performs the following operations in sequence:
1. Starts a local Solana validator
2. Builds your program
3. Deploys it to the local network
4. Runs your tests
5. Shuts down the validator when tests complete

```shell
anchor localnet
```

The `anchor localnet` command is similar to `anchor test` but with key differences:
1. Starts a local Solana validator
2. Builds your program
3. Deploys it to the local network
4. Does NOT run any tests
5. Does NOT terminate the local validator

This is particularly useful during development when you want to keep a local validator running while you make changes to your frontend or when you want to interact with your program manually.


# Snow Token Issuer Smart Contract

Welcome to the Snow Token Issuer project! This smart contract allows users to issue a new token named "SNOW" on the MultiversX blockchain. The contract is written in Rust and compiled to WebAssembly (WASM) using the MultiversX SDK.

## Overview

The `SnowTokenIssuer` contract provides a simple interface for users to mint SNOW tokens by calling a single endpoint with an EGLD payment. The tokens are minted with the following properties:
- **Token Name**: Snow Token
- **Ticker**: SNOW
- **Decimals**: 8

## Prerequisites

To work with this project, you'll need the following installed on your system:

- **Rust and Cargo**: Install via [rustup.rs](https://rustup.rs/).
- **MultiversX SDK**: Follow the [installation guide](https://multiversx.com/builders/tutorials/in-depth/part-1-install-required-dependencies) to set up the MultiversX development environment.

## Installation

1. **Clone the Repository**: Begin by cloning the project repository to your local machine.

   ```bash
   git clone <your-github-repo-url>
   cd snow_token_issuer


Build the Smart Contract: Compile the smart contract to WebAssembly using the provided build script.

./build.sh
Ensure the build script has execution permissions. If not, make it executable:

chmod +x build.sh

Deployment
Deploy the smart contract on the MultiversX blockchain using the CLI tools:

Prepare Your Wallet: Ensure you have access to a wallet PEM file with sufficient EGLD balance for deployment.

Deploy the Contract: Run the following command, replacing <path-to-wallet.pem> with the path to your wallet file.

mxpy contract deploy --project=./ --recall-nonce --pem=<path-to-wallet.pem> --gas-limit=50000000


This command will output the address of the deployed contract, which you can use for further interactions.

Usage
Once deployed, you can interact with the contract using the issueTokenSnow endpoint:

Endpoint: issueTokenSnow
Payment: Requires a payment in EGLD to mint SNOW tokens.
Example Command
To call the issueTokenSnow endpoint and mint SNOW tokens, use:

mxpy contract call <contract-address> --recall-nonce --pem=<path-to-wallet.pem> --gas-limit=5000000 --function="issueTokenSnow" --value=<amount-in-egld>

Replace <contract-address> with the deployed contract address and <amount-in-egld> with the amount of EGLD you want to send.

Verification
After minting, you can verify the issuance of SNOW tokens on the MultiversX explorer under the "ESDT Tokens" tab. This will confirm the token creation and its allocation to your address.

Contributing
Contributions are welcome! Feel free to open issues or submit pull requests. For major changes, please open an issue first to discuss what you would like to change.

License
This project is licensed under the MIT License - see the LICENSE file for details.

Contact
For questions, suggestions, or contributions, please contact [your-email@example.com].

Thank you for your interest in the Snow Token Issuer project!


### Key Sections Explained

- **Overview**: Provides a brief introduction to the project.
- **Prerequisites**: Lists the software and tools needed to work with the project.
- **Installation**: Details the steps to clone and build the project.
- **Deployment**: Explains how to deploy the contract on the MultiversX blockchain.
- **Usage**: Provides instructions on how to interact with the deployed contract.
- **Verification**: Guides users on how to verify token issuance.
- **Contributing**: Encourages community contributions.
- **License**: Specifies the project's license.
- **Contact**: Offers a way for users to reach you for further inquiries.

By following this detailed README, users will have a comprehensive guide to working with your smart contract project.

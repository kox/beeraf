# Beeraf Program

The beeraf program is a Solana-based decentralized application (dApp) that allows users to create raffles, buy tickets, and determine winners on-chain in a transparent and decentralized way. 

It is built using the Anchor framework, a powerful framework for Solana programs.

## Table of Contents

* Overview
* Features
* Getting Started
    * Prerequisites
    * Installation
* Program Architecture
    * Modules
    * Program Functions
* Usage
    * Initialize
    * Create a Raffle
    * Buy a Ticket
    * Solve a Raffle
    * Scratch a Ticket
* Error Handling
* Events
* Deployed
* Contributing
* License

## Overview

The beeraf program provides a decentralized raffle system where users can create raffles, buy tickets, and determine the winners in a fair and transparent manner. All operations are conducted on the Solana blockchain, ensuring security and immutability.

## Features

    Initialize Treasury: Set up a treasury account where fees collected from raffles will be stored.
---    
    Create Raffles: Create new raffles with specific parameters like ticket price, number of tickets, and associated NFTs.
---    
    Buy Tickets: Participants can buy tickets for a raffle, which are minted as NFTs.
---    
    Solve Raffles: Determine the winner of a raffle by generating a random number based on the number of tickets sold.
---    
    Scratch Tickets: Verify if a ticket is a winner, transfer the prize, and burn the winning ticket NFT.

## Getting Started

### Prerequisites

Before using the beeraf program, ensure you have the following installed:

    Solana CLI
    Anchor CLI
    Rust and Cargo (typically installed with Anchor)
    Node.js and NPM (for JavaScript/TypeScript testing)

### Installation

Clone the Repository:

```bash
git clone https://github.com/your-username/beeraf.git
cd beeraf
``` 

Install Dependencies:

```bash
anchor build
``` 

Deploy to Devnet (or your preferred cluster):

```bash
anchor deploy --provider.cluster devnet
```

## Program Architecture

### Modules

    constants: Defines constants used throughout the program.
---    
    contexts: Contains the context structures for the various instructions.
---    
    error: Defines custom errors for the program.
---    
    state: Manages the program's state, such as accounts and PDAs.

### Program Functions

The beeraf program includes the following functions:

1. initialize

    Purpose: Initializes the treasury account and sets up the fee structure.
    Arguments:
        fee: u64: The fee amount to be collected from each transaction.
    Context: Initialize

2. create_raffle

    Purpose: Creates a new raffle with a specified ticket price, mint authority, and NFT details.
    Arguments:
        args: CreateRaffleArgs: Contains parameters like the raffle name, URI, ticket price, and raffle fee.
    Context: CreateRaffle

3. buy_ticket

    Purpose: Allows users to buy a ticket for a raffle by minting an NFT representing the ticket.
    Arguments:
        args: BuyTicketArgs: Contains the raffle ID and number of tickets to buy.
    Context: BuyTicket

4. solve_raffle

    Purpose: Determines the winner of the raffle by generating a valid number considering the number of tickets sold.
    Arguments:
        sig: Vec<u8>: A cryptographic signature used to verify the winner.
    Context: SolveRaffle

5. scratch_ticket

    Purpose: Checks if a ticket is a winner, transfers the prize to the user, and burns the winning ticket to recover rent.
    Context: ScratchTicket

## Usage

Below are step-by-step instructions for using each function.

1. Initialize

Initialize the treasury account:

```rust
let tx = program.methods.initialize(fee);
```

2. Create a Raffle

Create a new raffle with specific parameters:

```rust
let tx = program.methods.create_raffle(args);
```

3. Buy a Ticket

Buy a ticket for a raffle:

```rust
let tx = program.methods.buy_ticket(args);
```

4. Solve a Raffle

Determine the winner of a raffle:

```rust
let tx = program.methods.solve_raffle(sig);
```

5. Scratch a Ticket

Check if a ticket is a winner and distribute the prize:

```rust
let tx = program.methods.scratch_ticket();
```

## Error Handling

The beeraf program defines custom error codes in the error module to handle various error conditions. Refer to the error module for a list of possible errors and their meanings.

## Events

    WinnerEvent: Emitted when a raffle is resolved and a winner is determined.

## Deployed
    Program Id: 9kqdw16Bf66qL53XSzG21TZjDEWPfawuyBTML1vVPqTs


## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License.



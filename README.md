
# Solana Calculator Program

## Overview

This readme provides an explanation of the Solana program code for a simple calculator. The program allows users to perform addition and subtraction operations on a calculator account. The account stores a single floating-point number, `result`, which is updated based on the provided operation and operands.

## Code Structure

### Dependencies

The program uses external crates and Solana program dependencies. Notable dependencies include:

-   `borsh`: A library for serialization and deserialization.
-   `solana_program`: Solana's Rust library for program development.

### `CalAcct` Struct

-   The `CalAcct` struct represents the calculator account and contains a single field, `result`, which stores a floating-point number.

### Entry Point

-   The entry point for the Solana program is defined using the `entrypoint!` macro, and it is named `process_instruction`.

### `process_instruction` Function

-   This function is the core logic of the calculator program.
-   It takes the program's ID, a list of accounts, and instruction data as input.
-   It checks if the provided account's owner matches the program's ID. If not, it initializes the account data with a default `CalAcct` instance with `result` set to 0.0.
-   It deserializes the `CalAcct` instance from the account data.
-   It checks if the instruction data is at least 1 byte long.
-   It extracts an operation code from the instruction data (0 for addition, 1 for subtraction) and performs the corresponding operation.
-   It serializes the updated `CalAcct` instance and stores it back in the account data.

### Helper Functions

-   Two helper functions, `perform_addition` and `perform_subtraction`, are defined to perform addition and subtraction operations on the `CalAcct` instance, updating the `result` field accordingly.

## Usage

To use this Solana calculator program, follow these steps:

1.  Build and deploy the program to a Solana cluster.
2.  Create a Solana account to store the calculator data.
3.  Send transactions to the program, specifying the operation and operands in the instruction data.

## Example Instructions

-   To perform addition:
    -   Set the first byte of the instruction data to 0.
    -   Append two 8-byte chunks representing the operands as little-endian bytes (total of 16 bytes).
-   To perform subtraction:
    -   Set the first byte of the instruction data to 1.
    -   Append two 8-byte chunks representing the operands as little-endian bytes (total of 16 bytes).

## Error Handling

-   The program includes error handling to check for invalid instruction data and operation codes.

# Code Sharing Smart Contract on Stellar (Soroban)

A decentralized smart contract for storing and retrieving code snippets on the **Stellar Soroban** blockchain.

---

## Project Title
**Code Sharing**

## Project Tagline
**Share code on-chain, retrieve it anytime, and build transparent developer collaboration on Stellar.**

---

## Table of Contents
- [Project Description](#project-description)
- [What it does](#what-it-does)
- [Why this project](#why-this-project)
- [Features](#features)
- [Smart Contract Functions](#smart-contract-functions)
- [Data Structure](#data-structure)
- [Deployed Smart Contract Link](#deployed-smart-contract-link)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [How It Works](#how-it-works)
- [Example Workflow](#example-workflow)
- [Local Setup](#local-setup)
- [Build Instructions](#build-instructions)
- [Deploy Instructions](#deploy-instructions)
- [Future Improvements](#future-improvements)
- [Use Cases](#use-cases)
- [Author](#author)
- [License](#license)

---

## Project Description
**Code Sharing** is a **basic decentralized application logic layer** built as a **Soroban smart contract** on the **Stellar blockchain**.

This project allows users to store code snippets directly on-chain with useful metadata such as:

- Author name
- Title of the snippet
- Programming language
- Code content

The purpose of this project is to demonstrate how blockchain technology can be used beyond tokens and payments — specifically for **developer tools**, **knowledge sharing**, and **permanent code storage**.

This is a beginner-friendly Soroban project and can serve as a strong base for a more advanced Web3 code-sharing platform.

---

## What it does
This smart contract provides a simple **on-chain code snippet storage system**.

Users can:

- Share code snippets
- Store code metadata permanently in contract storage
- Retrieve a snippet using its unique ID
- Track the total number of uploaded code snippets

Every time a new snippet is uploaded, the contract:

1. Increments a counter
2. Assigns a new unique ID
3. Stores the snippet in Soroban contract storage

---

## Why this project
Most code-sharing platforms are centralized. This project explores how code snippets can be stored in a **decentralized and transparent** environment.

### Why this is useful:
- Prevents silent modification of stored snippets
- Makes shared code permanently accessible
- Demonstrates non-financial blockchain utility
- Serves as a foundation for a decentralized developer platform

This project is especially useful as:
- A **Soroban learning project**
- A **hackathon project**
- A **starter Web3 developer portfolio project**

---

## Features

### Core Features
- **Upload Code Snippets**
  - Store code snippets on-chain.

- **Retrieve Snippets**
  - Fetch any snippet using its unique ID.

- **Track Total Snippets**
  - Count how many snippets have been uploaded.

### Metadata Features
Each code snippet stores:
- **ID**
- **Author**
- **Title**
- **Programming Language**
- **Code Content**

### Blockchain Features
- **On-chain Storage**
- **Permanent Record**
- **Transparent Data Retrieval**
- **Smart Contract Based Logic**

### Developer-Friendly Features
- Beginner-friendly Rust + Soroban structure
- Simple and easy to understand contract architecture
- Great base for future dApp integration

---

## Smart Contract Functions

### `share_code(author, title, language, content)`
Stores a new code snippet on-chain.

#### Parameters
| Parameter | Type | Description |
|----------|------|-------------|
| `author` | `String` | Name of the code author |
| `title` | `String` | Title of the code snippet |
| `language` | `String` | Programming language used |
| `content` | `String` | The actual code content |

#### Returns
- `u64` → Unique ID of the newly stored snippet

#### Purpose
This function is used whenever a user wants to upload a new code snippet to the blockchain.

---

### `get_code(id)`
Retrieves a code snippet by its unique ID.

#### Parameters
| Parameter | Type | Description |
|----------|------|-------------|
| `id` | `u64` | Unique snippet ID |

#### Returns
- `CodeSnippet` → Full code snippet data

#### Purpose
This function is used to fetch a previously uploaded code snippet.

---

### `total_codes()`
Returns the total number of code snippets stored in the contract.

#### Returns
- `u64` → Total uploaded snippets count

#### Purpose
This function is useful for tracking the total number of code entries shared on-chain.

---

## Data Structure

The contract uses a structured data model to store each snippet.

### `CodeSnippet`
```rust
pub struct CodeSnippet {
    pub id: u64,
    pub author: String,
    pub title: String,
    pub language: String,
    pub content: String,
}
```
<img width="1470" height="836" alt="Screenshot 2026-03-27 at 1 58 28 PM" src="https://github.com/user-attachments/assets/f904e41a-caf4-490a-8eab-4310445ba397" />



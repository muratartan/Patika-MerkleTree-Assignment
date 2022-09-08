# Patika.dev Community Gaming Solana & Rust Practicum
## Hands on Task: Merkle Tree Root Calculator on Rust

Firstly, we should understand what is Merkle Tree and how it works well. In order to do this, please research about [merkle tree](https://en.wikipedia.org/wiki/Merkle_tree) and 
watch [the video](https://www.youtube.com/watch?v=n6nEPaE7KZ8) carefully.

Our main purpose is that take inputs from txt files, hash them and calculate the root.
Example txt file looks like:
```txt 
3
patika
izmir
denizli
adana
mersin
deniz
gemi
ucak
```
The first line is **_n_** which is $2^n$ where the total number of leaves bottom of the tree. Then, if **_n_** is 3 like above we have **8** naked leaves in our tree. All of them is non hashed and they are written to text file one under the other.
![Merkle Tree](https://upload.wikimedia.org/wikipedia/commons/9/95/Hash_Tree.svg)

## Instructions

1. Fork or clone this repository
2. Add dependencies to Cargo.toml which you would like to use
3. Read data from txt file and use it as is required so, store the **_n_** as **u32** and store splitted **String** inputs to a **Vector of Strings**
4. Hash all inputs with SHA256 from [sha-2](https://docs.rs/sha2/latest/sha2/) crate



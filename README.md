# Steganography Playground

This crate provides functionality to create image steganography challenges based on different methods. This can be useful for CTF challenge creation aswell as testing your own skills.

## Current Coverage
If a method is on this table and is not implemented, it will eventually be added in the future. More methods may also be added to this table in future aswell. The order of each method may also not be the order they are added.
| Method       | Implementation Status   |
|--------------|-------------------------|
| LSB          | ✅ Implemented         |
| Single-Layer | ✅ Implemented         |
| PVD          | ✅ Implemented         |
| APVD         | ❌ Not Implemented Yet |
| BPCS         | ❌ Not Implemented Yet |
| DCT          | ❌ Not Implemented Yet |
| DWT          | 🚧 In Progress         |
| DFT          | ❌ Not Implemented Yet |

## Usage
A challenge can be created by using the `steg::StegBuilder` struct, a builder pattern struct that by default will create a challenge with a random method and hidden message derived from a seed. The seed and method can be set by the user aswell. For example, the snippet below produces a challenge with a specific seed.
```rust
let challenge = StegBuilder::new()
    .with_seed(450)
    .build();
```
The resulting challenge (`steg::StegChallenge`) will contain the embedded image and other relevant metadata for the associated challenge.


## Possible Future Features
- More steganography methods which do not use images, and instead use files like PDFs and others may be added in future aswell
- Ability to use a custom image instead of the current randomly generated image from noise
- Ability to use a custom hidden message in the image instead of the current randomly generated one
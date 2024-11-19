# Cryptography challenges

## List of challenges
- [x] Convert hex to base64
- [x] Fixed XOR
- [ ] Single-byte XOR cipher
- [ ] Detect single-character XOR
- [ ] Implement repeating-key XOR
- [ ] Break repeating-key XOR
- [ ] AES in ECB mode
- [ ] Detect AES in ECB mode
- [ ] Implement PKCS#7 padding
- [ ] Implement CBC mode
- [ ] The CBC padding oracle
- [ ] Implement CTR mode
- [ ] Break fixed-nonce CTR mode using substitutions
- [ ] Recover the key from repeating-nonce CTR
- [ ] Implement the MT19937 Mersenne Twister RNG
- [ ] Break MT19937 seed
- [ ] Clone an MT19937 RNG
- [ ] Create the MT19937 stream cipher
- [ ] Break the MT19937 stream cipher
- [ ] AES key recovery from nonce
- [ ] Implement RSA
- [ ] Implement an E=3 RSA broadcast attack
- [ ] Implement Diffie-Hellman
- [ ] Implement a MITM attack on Diffie-Hellman
- [ ] Implement an authenticated key exchange
- [ ] Break SRP with a zero key
- [ ] Implement simple DSA signing
- [ ] Break a bad DSA implementation
- [ ] Implement elliptic curve operations

## Convert hex to base64

The string 
```
49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
```

Should produce :
```
SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
```


### Code
```rust
use base64::*;
use hex::FromHex;
use anyhow::*;
use prelude::BASE64_STANDARD;


fn hexto64(hex: &str) -> Result<String, Error>{

    let byte = hex::decode(hex)?;

    let result = BASE64_STANDARD.encode(byte);

    Ok(result)
}
```

## Fixed XOR

 Write a function that takes two equal-length buffers and produces their XOR combination.

If your function works properly, then when you feed it the string: 


```
1c0111001f010100061a024b53535009181c
```

... after hex decoding, and when XOR'd against: 

```
686974207468652062756c6c277320657965
```

... should produce: 

```
746865206b696420646f6e277420706c6179
```
### Code
```rust
use base64::*;
use hex::FromHex;
use anyhow::*;

fn fixed_xor(input1: &str, input2: &str) -> Result<String, Error>{

    let mut byte1 = hex::decode(input1)?;
    let byte2 = hex::decode(input2)?;

    for i in 0..byte1.len(){
        byte1[i] ^= byte2[i];
    }

    let result = hex::encode(byte1);
    Ok(result)
}
```

## Single-byte XOR cipher

The hex encoded string: 

```
1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
```

 ... has been XOR'd against a single character. Find the key, decrypt the message.

You can do this by hand. But don't: write code to do it for you.

How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score. 
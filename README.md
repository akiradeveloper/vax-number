# vax-number

[![Crates.io](https://img.shields.io/crates/v/vax-number.svg)](https://crates.io/crates/vax-number)

**Disclaimer: This is a joke program.**

Covid19 is a dangerous virus and
the vaccination is believed to be a
powerful solution against the terrible desease.

As well as the number of shots has been taken,
the name of the vaccine provider is important.
So the people who took shots likely show their
vaccination status in a series of alphabets
each represents the provider name.
An example is "PPMP" which means
"Phizer-Phizer-Moderna-Phizer".

In my country Japan,
most people has taken three shots already and they are taking more shots every 6 months according to the government's plan.
So, in 10 years, it will be 20 shots per person.

The Problem is, 20 alphabets is too long for Twitter account name.
Imagine my Twitter account has a name "AkiraPMPMPMPMPMPMPMPMPMPM". This is insane.

So let's compress the vaccination status into an integer called Vax Number.

## How To Install

- `cargo install --path .` to install the binary.

## How To Use

- `vax-number encode VAX_STATUS` to encode a vaccination status into a vax-number.
- `vax-number decode VAX_NUMBER` to decode a vax-number.

```
$ vax-number encode PPMP
649
$ vax-number decode 649
PPMP
```

## Features

- Support no_std
- encode/decode

## Limitations

- Use 3 bits for one shot for wider support of vaccine providers. Maximum 7 vax providers can be supported.
  - Phizer and Moderna are only supported in the first release.
  - Feel free to request for other providers.
- At most 21 shots is supported for performance reason.

## Author

Akira Hayakawa (ruby.wktk@gmail.com)

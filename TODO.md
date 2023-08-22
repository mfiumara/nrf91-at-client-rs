# TODO

- Make some simple multi-platform options
- Threading model
- Create some core API's
- Enable native compilation
- Add testing in qemu

- Make bootloader
- Create kernel primitives

## Core API's

- Logging
- Time
- Shell

## Threading model & runtime

- RTIC
- smol
    - https://github.com/smol-rs/futures-lite
    - https://github.com/smol-rs/async-task (requires alloc)
- async-std
    - https://github.com/async-rs/async-std
    - only alloc required


We could create our own custom threading model, but for starters we could begin with something like RTIC (https://rtic.rs/2/book/en/).

There's also https://github.com/smol-rs/smol which is designed for no_std?

I do not like the syntax of RTIC though and that everything needs to be implemented in an #app attribute. It's not simple for the end-user.

We can also use asynchronous programming paradigm with async/await. This could be more suitable even than a traditional threading model.

We could implement our own threading model purely in software without hardware acceleration. This should simplify the kernel development and later we can implement CPU specific hardware acceleration.

The most simple threading model in OS is 1:1 1:1 (kernel-level threading). 

Most RTOSs cannot handle multi-core platforms, but we see a trend in more and more multi-core microcontroller systems. This must be supported by the kernel. 

## Testing

https://github.com/knurling-rs/defmt/tree/704bee6ebfa153aad9dba1fcee5ba0ec6b77f3a8/firmware/defmt-test

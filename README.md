# μSBE
μSBE (sometimes written 'usbe' or microSBE) is a small Simple Binary Encoding implementation written in C99.
It provides a reflective API (what the SBE wiki calls an OTF API) that is faster, smaller and easier to embed than existing approaches. 

The difference between the existing "OTF API" and the usbe api is conceptually similar to the difference between a tree-walking-interpreter and a bytecode-executing-vm in interpreted language world. For futher explaination see BOOK.md or book/minitable.md.

## Speed Improvment Benchmarks
[TODO]

## What is μSBE Useful For?
μSBE is best suited to embedded use cases, examples include:
- Wrapping the μSBE kernel in your language of choice to create a new SBE implementation.
- Applications that need to process messages but which do not know the schema at compile time (e.g., network sniffers, node spoofers)
- Embedded applications without std (bare metal, microcontrollers, no_std Rust)

### How to build the Kernel [TODO]
### How to build the python example implementation [TODO]
### How to build the rust example implementation [TODO]

## A note on the style of this repo
This repo is an experiment in a form of semi-literate programming, as well as the home of a useful program. Literate programming is one of Donald Knuth's big ideas. I will not be doing it his way. In Knuthian literate programming the code is expositional literature first and a useful program second. This is a useful program first.

There is a mostly-linear way to "read" this repo, and if someone does that, I'd be delighted. 

The approach was inspired by Crafting Interpreters — but this is not a real book, just an **attempt** at a light form of literate programming: code that makes an effort to transfer its theory. For more information on this see BOOK.md

## How to build the book [TODO]


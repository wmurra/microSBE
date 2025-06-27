# μSBE
μSBE (sometimes written 'usbe' or microSBE) is a small Simple Binary Encoding implementation written in C99.
It provides a reflective API (what the SBE wiki calls an OTF API) that is faster, smaller and easier to embed than existing approaches. 

The difference between the existing "OTF API" and the usbe api is conceptually similar to the difference between a tree-walking-interpreter and a bytecode-executing-vm in interpreted language world. For futher explaination see BOOK.md or book/minitable.md.

## What is this useful For?:
μSBE is best suited to embedded usecases examples include:
- Wrapping the μSBE kernel in your language of choice to creat a new SBE implementation.
- Applications that need to process messages but which do not know the schema at compile time (ie network sniffers, node spoofers)
- Embedded applications without std (bare metal, microcontrollers, no_std Rust)

## How to build the program [TODO]
### How to buid the Kernel [TODO]
### How to build the python example implementation [TODO]
### How to build the rust example implementation [TODO]

## A note on the style of this repo
This repo is also an experiment in a form of semi-literate programming. Literate programming is one of Donald Knuth's big ideas, and I will not be doing it his way. In Knuthian literate programming the code is expositional literature first and a useful program second. I do not intend this to be amazingly expositional, but there is a mostly-linear way to "read" this repo, and if someone does, I'd be delighted. for more information on this see BOOK.md

## How to build the book [TODO]


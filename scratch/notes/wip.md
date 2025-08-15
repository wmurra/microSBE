ok so two things I need to understand inside and out to get this done 
    1. the SBE protocol, 
    2. the MiniTable and MiniDescriptor approach from upb

the sbe protocol part seems more tractable right now, so I think I will attempt to just make
a simple tree walking decoder for sbe and I am going to do that in rust for simplicity's sake.

ok so I am correctly pulling in the sbe_repo as well as the sbe tool. pretty interesting bazel stuff
if you specify a build_file for an HTTP archive you can inject BAZEL build semantics into a non bazel
repo. for a lot of cases this does not really work without a massive fight but this is a good use 
case, I am pulling in the sbe_repo so I can make use of the example sbe xml schemas. I got that 
working and figured out how to correctly run the sbe-all.jar tool

The docs for the tool are pretty far out of date, I took a guess that "Rust" would work as a target 
language and it did, but that isn't documented anywhere.

also the way you pass cli args to a jar is like -Dargument_namespace.argument and it comes before 
the jar but after java which is interesting. 

pretty odd stuff
I am now going to move on to trying to build a rust project that links in the library that sbe is 
generating for me then build up a message, spit it out to binary and lastly decode that binary back 
to a message. 

[NOTE] when I was trying to get this to work I temporarily gave up on hermeticity for the java binary
as a result if someone else wanted to build they would need to have java installed, not good.
consider it tech debt, I need to go back and make the java binary part of the hermetic toolchain.

reading the docs for rules_rust I see this rust-analyzer rule and I got excited, definitely make 
sure we set that up so that rust analyzer can work in vscode. 

There is this command
```shell
$: bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
```

I just ran it, I suppose this is gonna create a json

to get the language server to actually work you need to run vscode in wsl and regenerate the json 
ok now thats working, lets focus on encoding a single message to bytes and decoding it

I found some kick ass reference material here 
https://speice.io/2019/09/binary-format-shootout/

https://github.com/speice-io/marketdata-shootout/blob/master/src/sbe_runner.rs

I wasted a lot of time, I sort of just stared at the code-gen'd rust code for a while and felt like 
a moron for not knowing how to use it, then I found this: "simple-binary-encoding/rust/benches"
it's a perfect template example of using the rust stubs, so I am going to read THIS and it should
answer all my questions, the only odd thing is they are using criterion which I assume is a 
benchmarking lib so I'll just ignore those parts. 

ok so I adapted the benchmark to an example of just using the rust library and it makes a lot more 
sense now

here is an actually good reference for the XML format 
https://github.com/aeron-io/simple-binary-encoding/wiki/FIX-SBE-XML-Primer 
https://www.fixtrading.org/standards/sbe-online 
https://raw.githubusercontent.com/FIXTradingCommunity/fix-simple-binary-encoding/master/v1-0-RC4/doc/02FieldEncoding.md 
https://www.fixtrading.org/packages/simple-binary-encoding-technical-specification-final 

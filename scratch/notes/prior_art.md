# Reflective Runtime
Most IDLs have _some_ reflective runtime.
What do I mean by this? I cannot sum it up any better than this blurb from the "Dynamic Reflection" section of the Cap'n Proto docs:
>Sometimes you want to write generic code that operates on arbitrary types, iterating over the fields or looking them up by name. For example, you might want to write code that encodes arbitrary Cap’n Proto types in JSON format. This requires something like “reflection”, but C++ does not offer reflection. Also, you might even want to operate on types that aren’t compiled into the binary at all, but only discovered at runtime.
 
The solution for this is a reflective API. Most IDLs solve this by allowing for the user to depend on the __compiler__ as a library, then the intermediate representation (IR) (Usually a tree-like data structure which the compiler needed to be able to generate anyway) can be used at runtime as a model for serialization / deserialization. Protobuf sometimes refers to this as descriptor walking. I will refer to this strategy as the IR-tree-walking strategy. An alternative, and less common strategy is to flatten the tree into a series of
 
### Protobuf
Sometimes refers to this as descriptor-walking: you can load a compiled FileDescriptorSet at runtime and dynamically decode or construct any message described within it. This pattern of use is very well supported in protobuf, it's also pretty slow.
- C++ API docs – DynamicMessageFactory / DescriptorPool
https://protobuf.dev/reference/cpp/api/#dynamicmessagefactory
- Here is a google groups discussion of the technique:[Source](https://groups.google.com/g/protobuf/c/oKLb32LLIiM)
 
 
### Cap'n Proto
there is a Dynamic API that accepts a packed schema blob while the program is running and uses it to read, traverse, and write arbitrary messages.
- Official C++ guide – section “Dynamic Reflection” [Source](https://capnproto.org/cxx.html)
- Cap’n Proto issue #634 – some guy complaining about the dynaic API being bloat, this shows that you opt out of it being included (not an opt in) [Source](https://github.com/capnproto/capnproto/issues/634)
 
### Simple Binary Encoding (SBE)
Heavily discouraged... but it is possible. The wiki includes a section called Cpp OTF User Guide that shows you how to use the IR to decode "on the fly" (OTF). SBE prefers to use this OTF terminology over Reflective API but the idea is the same.
 
## Performance References
Reflection-based decoding is usually far slower  (often 5-50×) than statically generated code, because every field access involves table look-ups, type switches, and boundary checks. I have found two exceptions to this rule:
 
1. u(micro)pb
    Google's own C only, small implementation of protobuf intended to be used for embedded applications
    - Direct quote from the readme, "fast reflection-based parsing: messages loaded at runtime parse just as fast as compiled-in messages." [Source: Google udp README.md](https://chromium.googlesource.com/chromium/src/third_party/protobuf/+/HEAD/upb/README.md)
 
2. FlatBuffers – Reflection API is only marginally slower thanks to zero-copy design
    - Quote: “Because reflection still accesses data in-place without unpacking, its performance is close to code-generated access.”
    [Source: FlatBuffers guide, section “Reflection and Schema Evolution”](https://google.github.io/flatbuffers/flatbuffers_guide_use_cpp.html#reflection)
   
## Serialized Intermediate Representations
In the world of IDLs, a serialized IR is a thing... I don't understand why. Caching is the first thing that comes to mind but that seems... wrong. If the _proper_ use of an idl is code gen / static API then the idl_source_code -> parse_tree -> intermediate_representation pipeline should A. Only happen at compile time so who cares if its slow and B. It should be fast. So I am not sure why exactly serialized IRs are a "thing" in IDL-land but they are [note this is a few days later, now I understand, the in memory representation of the IR is good to have for a bunch of reasons a big one if for embedded dynamic runtimes which don't want to ship with a json parser but still want a reflective api], here are some excepts to back that up. 1. from the SBE documentation
> Generating Serialized IR
> Generating serialized IR is straight forward with the SBE Tool. Simply set `sbe.generate.ir` to true. The SBE tool will  to write the serialized IR file using the same name as the input file with the `.sbeir` suffix. ...
> Using Serialized IR as Input to SBE Tool
> Serialized IR can act as input to the SBE Tool for code generation. This is done by using the file extension `.sbeir` instead of `.xml`. ... Therefore it is possible to use serialized IR as a means to pass around "compiled" representations.
 
## Mini tables
 
Ok so if you insist on doing a dynamic runtime there is a much faster way than tree walking. Take a technique from the interpreted-language world. in interpreted languages they make a distinction between a tree walk interpreter and a bytecode based vm. Tree walk interpreters are just like tree-walk codecs, they operate directly over the IR and they have the same disadvantage - lots of indirection, and the same major advantage - simplicity. They are usually used for language prototypes, Ruby used a tree walk interpreter until 2007, lua used one until 2000. So, do any existing reflective runtimes implement a similar approach, (flatten the tree into a linear tape of ops and just operate over the tape)? after some research I have only found one such example, upb.
 
   
[upb](https://github.com/protocolbuffers/protobuf/tree/main/upb) is the no std, no deps, tiny reflective runtime for protobuf. It's main use-case is as a core library that can be wrapped by interpreted languages, the main python, php and ruby implementations for protobuf are all just upb wrappers. It's readme boasts:
 
> Like the main protobuf implementation in C++, it supports:
> - a generated API (in C)
> - reflection
> - binary & JSON wire formats
> - text format serialization
> - all standard features of protobufs (oneofs, maps, unknown fields, extensions, etc.)
> - full conformance with the protobuf conformance tests
> ...
> **fast reflection-based parsing**: messages loaded at runtime parse **just as fast as compiled-in messages.**
 
It accomplishes this speed using what it calls "mini_tables", which if I understand correctly, have alot in common with bytecode.
Here is a doc that explains how to use upb as a library, it serves as a good tutorial intro to the upb internals:
https://github.com/protocolbuffers/protobuf/blob/main/docs/upb/wrapping-upb.md
and here is a wonderful design doc that covers some of the rational behind upb: https://gitlab.sternula.com/mmtp-pub/cpp-mms-agent/-/blob/7bc3ce2fa9c0adda4f6b49c99d13d004562f46af/ProtoBuf/docs/upb/design.md

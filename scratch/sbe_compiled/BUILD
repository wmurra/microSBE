cc_binary(
    name = "hello",
    srcs = ["hello_bazel.c"],
    copts = [
        "-std=c99",
        "-ffreestanding",  # Tells compiler not to assume stdlib
    ],
)


# This genrule uses the binary @sbe_tool AND the sbe_repo. It points the sbe_tool at the example 
# files from the sbe_repo builds the necessary rust stubs. 
genrule(
    name = "gen_rust_sbe_example_code",
    srcs = ["@sbe_repo//:example_schema_all", "@sbe_repo//:example_schema_main"],
    outs = [
        "baseline/src/boolean_type.rs",
        "baseline/src/boost_type.rs",
        "baseline/src/booster_codec.rs",
        "baseline/src/car_codec.rs",
        "baseline/src/engine_codec.rs",
        "baseline/src/group_size_encoding_codec.rs",
        "baseline/src/lib.rs",
        "baseline/src/message_header_codec.rs",
        "baseline/src/model.rs",
        "baseline/src/optional_extras.rs",
        "baseline/src/var_ascii_encoding_codec.rs",
        "baseline/src/var_string_encoding_codec.rs",
        ],
    tools = ["@sbe_tool//file"],
    cmd = """\
    java -Dsbe.xinclude.aware=true -Dsbe.output.dir=$(RULEDIR) -Dsbe.target.language=Rust \
    -jar $(location @sbe_tool//file) \
    $(location @sbe_repo//:example_schema_main)\
    """,
)

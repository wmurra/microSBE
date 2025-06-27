cc_binary(
    name = "hello",
    srcs = ["hello_bazel.c"],
    copts = [
        "-std=c99",
        "-ffreestanding",  # Tells compiler not to assume stdlib
    ],
)
# Dangling Pointer Bug in Rust

This example demonstrates a common error in Rust: creating a dangling pointer.  A dangling pointer points to memory that has been freed or deallocated. Accessing memory through a dangling pointer results in undefined behavior, which can lead to crashes, unexpected results, or security vulnerabilities.

The `bug.rs` file contains the buggy code, while `bugSolution.rs` provides a corrected version that avoids the dangling pointer problem.  This is a crucial concept in memory management in Rust, and understanding it is vital for writing safe and reliable code.
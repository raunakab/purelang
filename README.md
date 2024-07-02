# PureLang
A `rust` implementation of Dr. William Bowman's book on compiler construction.
The book can be found [here](https://www.students.cs.ubc.ca/~cs-411/2021w2/book_top.html).

## Structure
Please observe the following directory structure for the compiler:
```sh
/src
|-- /imperative_abstractions
|-- /register_allocation
|-- /structured_control_flow
|-- /x64
```
Each `phase` exposes a `compile: Source -> Target` function.
This function, as the type suggests, compiles the program through its stack.
Each phase is responsible for a different 'meta' purpose; i.e., the `register_allocator` phase is responsible for assigning registers to `abstract locations`, etc.

Each directory inside of each phase corresponds to a language (as well as the corresponding method to compile it to the next language in the stack).
Generally, this looks like the below:
```rust
mod phase1 {
    mod language1 {
        use crate::phase1::language2 as target;

        pub struct Language1(..);

        impl Language1 {
            fn pass1(self) -> target::Language2 {
                // ...
            }
        }
    }

    mod language2 {
        use crate::phase2::language3 as target;

        pub struct Language2(..);

        impl Language2 {
            fn pass2(self) -> target::Language3 {
                // ...
            }
        }
    }

    pub type Source = language1::Language1;

    pub type Target = crate::phase2::Source;

    pub fn compile(p: Source) -> Target {
        p
            .pass1()
            .pass2()
    }
}
```
Notice the `compile: Source -> Target` function in `phase1`.

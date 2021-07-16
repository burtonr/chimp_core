# Chimp Core (with code and cli)
This is only a proving ground for a different project in the works. 

Here, I plan to try things out with Rust and Clap to get a better understanding of the concepts in both.

Once I've figured things out, I can implement them in the _actual_ project (that doesn't exist yet).

I expect this to change drastically with no real rhyme or reason.

## Use

Using Cargo "features" to modify the included functionality.

```shell
cargo build --features code,issue
```

The thought here is that on the final project, a "coose your own binary" type tool will include the features the specific user wants.

> ex: "Code" feature with "github" support + "Issue" feature with "jira" support... etc


In a separate crate, implement the defined trait(s) as follows:

```rust
use chimp_core::chimp_code::SCM;

pub struct GitLab {
  pub owner: String
}

impl SCM for GitLab {
  fn clone(&self) -> String {
    format!("Cloning repo owned by: {}", self.owner)
  }
}
```

## TODO
Thinking that the features could be used to define what commands are available, but dynamic libraries will contain the various implementations of those commands. 

This will allow multiple providers to be developed independently, and swapped out at runtime with a config change (or even a CLI flag/param)

Read more [Plugins in Rust by Michael-F-Bryan](https://adventures.michaelfbryan.com/posts/plugins-in-rust/)
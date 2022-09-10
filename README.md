#### installation
      https://www.rust-lang.org/en-US/install.html
      https://www.rust-lang.org/tools/install
      https://ostechnix.com/how-to-fix-rust-error-linker-cc-not-found-on-linux/
      
#### update rust to latest stable version

      $ rustc --version
      rustc 1.27.2 (58cc626de 2018-07-18)
      $ cargo --version
      cargo 1.27.0 (1e95190e5 2018-05-27)

      $ rustup update stable
      info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
      ...

      $ rustc --version
      rustc 1.28.0 (9634041f0 2018-07-30)
      $ cargo --version
      cargo 1.28.0 (96a2c7d16 2018-07-13)

#### creating and running project

      creating project :                  cargo new <project name>
      running project : proj name>        cargo run

      
#### modules 

      https://learning-rust.github.io/docs/d3.modules.html

#### references 

      https://blog.thoughtram.io/references-in-rust/

#### todo

      https://www.hackertouch.com/rust-programming-language.html

#### on ubuntu
      To install Rust, run the following in your terminal, then follow the onscreen instructions.
      curl https://sh.rustup.rs -sSf | sh
      
      source $HOME/.cargo/env
      
      restart system to take effect


#### hello world
      create hello.rs
      fn main(){
        println!("hello world");
      }
      compile : 
      mithun@linux:~/Documents/rustl/example$ rustc hello.rs
      run : 
      mithun@linux:~/Documents/rustl/example$ ./hello

#### format string
      fn main(){
            let x = 5 + /**/ 5;
            println!("the result is {}", x)
      }
      
      output : 10

#### running project using cargo : (build and run project)
      
######      creating project
            mithun@linux:~/Documents/rustl/example$ cargo new hello_world --bin
            Created binary (application) `hello_world` project
      
######      project structure      
            mithun@linux:~/Documents/rustl/example$ cd hello_world/
            mithun@linux:~/Documents/rustl/example/hello_world$ ls
            Cargo.toml  src(main.rs)
      
######      opening project in visual code
            mithun@linux:~/Documents/rustl/example/hello_world$ code .
      
######      building project
            mithun@linux:~/Documents/rustl/example/hello_world$ cargo build
            Compiling hello_world v0.1.0 (file:///home/mithun/Documents/rustl/example/hello_world)
            Finished dev [unoptimized + debuginfo] target(s) in 0.79s

#####       running project
            mithun@linux:~/Documents/rustl/example/hello_world$ cargo run
            Finished dev [unoptimized + debuginfo] target(s) in 0.06s                   
            Running `target/debug/hello_world`
            Hello, world!
            
            
            
### links
      https://doc.rust-lang.org/reference/index.html
      https://doc.rust-lang.org/std/index.html
      https://doc.rust-lang.org/rust-by-example/index.html

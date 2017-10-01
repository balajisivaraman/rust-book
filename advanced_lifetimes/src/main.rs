struct Context<'a>(&'a str);

// this doesn't work w/o lifetime subtyping
// because Rust doesn't know of any relationship b/w 'c and 's
// the reference context has lifetime 'c and the string slice has 's
// this will only be valid if 's > 'c
// below syntax means that 's lives at least as long as 'c
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    // below doesn't work because after lifetime elision
    // we see that the the string slice has the same lifetime as Parser
    // and the Context within the parser also has the same lifetime
    // even though &str will outlive both, Rust cannot see that
    // this is because Parser goes out of scope at the end of this fn
    // and we also took ownership of the incoming context
    Parser { context: &context }.parse()
}

struct Ref<'a, T: 'a>(&'a T);
struct StaticRef<T: 'static>(&'static T);

trait Foo {}

struct Bar<'a> {
    x: &'a i32,
}

impl<'a> Foo for Bar<'a> {}

fn main() {
    println!("Hello, world!");
    let num = 5;
    let obj = Box::new(Bar { x: &num }) as Box<Foo>;
}

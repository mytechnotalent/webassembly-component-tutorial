#[allow(warnings)]
mod bindings;

// The comments that follow the `use` declaration below
// correlate the rust module path segments with their
// `world.wit` counterparts:
use bindings::exports::docs::subtractor::subtract::Guest;
//            <- items bundled with `export` keyword
//                     <- package namespace
//                           <- package
//                                  <- interface name

struct Component;

impl Guest for Component {
    fn subtract(x: u32, y: u32) -> u32 {
        x - y
    }
}

bindings::export!(Component with_types_in bindings);
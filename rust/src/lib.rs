mod computer;

use gdnative::prelude::*;
use rust_lisp::parse;

use crate::computer::Computer;

/// The HelloWorld "class"
#[derive(NativeClass)]
#[register_with(Self::register_signals)]
#[inherit(Node)]
pub struct HelloWorld {
    computer: Computer,
}

// You may add any number of ordinary `impl` blocks as you want. However, ...
impl HelloWorld {
    /// The "constructor" of the class.
    fn new(_owner: &Node) -> Self {
        HelloWorld {
            // computer: Computer::new(1_000_000),
            computer: Computer::new(100_000),
        }
    }
}

// Only __one__ `impl` block can have the `#[methods]` attribute, which
// will generate code to automatically bind any exported methods to Godot.
#[methods]
impl HelloWorld {
    #[export]
    fn _ready(&mut self, _owner: &Node) {}

    #[export]
    fn _process(&mut self, owner: &Node, _delta: f32) {
        if let Some(res) = (&mut self.computer).progress() {
            godot_print!("RESULT: {:?}", res);
            let (expr, err) = match res {
                Ok(expr) => (format!("{}", expr), "".to_string()),
                Err(err) => ("".to_string(), format!("{}", err)),
            };
            let args = &[Variant::new(expr), Variant::new(err)];
            owner.emit_signal("computation_complete", args);
        }
    }

    #[export]
    fn parse(&mut self, _owner: &Node, text: String) {
        godot_print!("Starting {}", text);
        self.computer.start(&text);
    }
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder
            .signal("computation_complete")
            .with_param("output", VariantType::GodotString)
            .with_param("error", VariantType::GodotString)
            .done()
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the new `HelloWorld` type we just declared.
    handle.add_class::<HelloWorld>();
}
// Macro that creates the entry-points of the dynamic library.
godot_init!(init);

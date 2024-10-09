struct Initialized;

struct State {
    state: Option<Initialized>,
}

impl State {
    fn new() -> Self {
        State { state: None }
    }

    fn initialize(&mut self) {
        self.state = Some(Initialized);
    }

    fn is_initialized(&self) -> bool {
        self.state.is_some()
    }
}

trait Action {
    fn perform(&self);
}

struct SayHello;
struct SayGoodBye;

impl Action for SayHello {
    fn perform(&self) {
        println!("Hello!");
    }
}
impl Action for SayGoodBye {
    fn perform(&self) {
        println!("Good Bye!");
    }
}

fn greet(action: &dyn Action) {
    action.perform();
}

pub fn main() {
    let mut machine = State::new();
    println!("Is initialized: {}", machine.is_initialized()); // false
    machine.initialize();
    println!("Is initialized: {}", machine.is_initialized()); // true

    let hello = SayHello;
    let bye = SayGoodBye;
    greet(&hello);
    greet(&bye);
}
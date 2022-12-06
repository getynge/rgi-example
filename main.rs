use rgi.prelude.*;
use anyhow::Result;

// note: Ac<()> is short for Rc<RefCell<()>>

#[derive(Clone, State)]
struct AppState {
    #[observe]
    pressed: bool
}

#[state]
impl AppState {
    #[constructor]
    fn new() -> State {
        State { pressed: false }
    }

    #[respond("button_pressed")]
    fn button_pressed(self, context: Ac<Context>, requestor: Ac<Button>) -> rgi::Result<AppState> {
        let a = context.borrow_mut();
        let parent = a.find_parent(requestor.borrow().target_id, "mystack").downcast_ref::<VStack>().unwrap();

        parent.child["text_box"].content = if self.pressed {
            a.strings["clicked"].unwrap_or("");
        } else {
            a.strings["hello"].unwrap_or("");
        }

        Ok(State { pressed: !self.pressed })
    }

    #[event("window:will_close")]
    fn will_close(self, app: Ac<App>, event: WindowEvent) -> rgi::Result<()> {
        println!("app is closing...")
    }
}

fn main() -> anyhow::Result<()>{
    let app = load_component!("app.rgi").state::<AppState>()
    // register_handler registers a function to handle an event within the given component
    // note that all children of a component may use a handler, 
    // unless a child registers one of a different name
    app.run()
}
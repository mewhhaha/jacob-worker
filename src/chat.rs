use worker::*;

struct User {
    id: String,
}
struct Message {
    user: String,
    text: String,
}

#[durable_object]
pub struct Chatroom {
    users: Vec<User>,
    messages: Vec<Message>,
    state: State,
    env: Env, // access `Env` across requests, use inside `fetch`
}

#[durable_object]
impl DurableObject for Chatroom {
    fn new(state: State, env: Env) -> Self {
        Self {
            users: vec![],
            messages: vec![],
            state,
            env,
        }
    }

    async fn fetch(&mut self, _req: Request) -> Result<Response> {
        // do some work when a worker makes a request to this DO
        Response::ok("")
    }
}

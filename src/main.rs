extern crate domafic;

use domafic::{DomNode, KeyIter};
use domafic::web_render::{run, JsIo};
use domafic::tags::*;

struct State {
}

impl State {
    fn new() -> Self {
        State {
        }
    }
}

enum Msg {
    //RemoveItToGetCompilerFail,
}

fn main() {

    let update = |state: &mut State, msg: Msg, keys: KeyIter, _: &JsIo<Msg>| {
    };

    let render = |state: &State| {
        div(())
    };

    run("body", update, render, State::new());
}


use glfw::{Action, Context, Key};
use gl::*;
use std::io;
use crate::handle::*;

pub struct CompiledSuccesfully;

pub struct CompiledWithErrors;

pub struct NotCompiled;

pub struct Shader<State = NotCompiled> {
    handle: Handle,
    state: std::marker::PhantomData<State>,
}


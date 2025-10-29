use glfw::{Action, Context, Key};
use gl::*;
use std::io;

pub enum Handle<HandleType = gl::types::GLuint> {
    Uninitialized,
    Initialized(HandleType),
}

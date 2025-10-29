use gl::*;
use std::{io, marker::PhantomData};
use crate::handle::*;


pub enum ShaderState {
    Compiled,
    NotCompiled,
    FailedCompile,
}

pub struct Shader {
    handle: Handle<gl::types::GLuint>,
    state: ShaderState,
    kind: gl::types::GLenum,
}
impl Shader {
    pub fn new(sk: gl::types::GLenum) -> Self {
        let mut handle: Handle = Handle::new();
        unsafe {
            handle.store(CreateShader(sk));

        }
        
        Shader {
            handle: handle,
            state: ShaderState::NotCompiled,
            kind: sk,
        }
    }

    pub fn shader_source(&mut self, source_code: String) {
        unsafe {


            

            ShaderSource(self.handle.read().unwrap(), );
        }
    }

}

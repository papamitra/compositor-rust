
use std;
use glium::{self, Surface};
use glium::glutin::{self, GlContext};

use std::rc::Rc;

error_chain!{}

pub struct Context {
    context: Rc<glium::backend::Context>,
    program: glium::Program,
}

impl Context {
    pub fn new<B>(backend: B) -> Result<Self> where B: glium::backend::Backend + 'static {
        let context = unsafe { glium::backend::Context::new(backend, true, Default::default())}.map_err(|err| "failed to create Context")?;
        let program = Context::create_program(context.clone())?;

        Ok(Context{ context: context, program: program})
    }

    fn create_program(context: Rc<glium::backend::Context>) -> Result<glium::Program> {

        let vertex_shader_src = r#"
            attribute vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            void main() {
                gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let program = glium::Program::from_source(&context, vertex_shader_src, fragment_shader_src, None)
            .chain_err(|| "failed to create program")?;

        Ok(program)
    }

    pub fn redraw(&self) {
        let mut target = glium::Frame::new(self.context.clone(), self.context.get_framebuffer_dimensions());
        target.clear_color(0.0, 1.0, 1.0, 1.0);

        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }

        implement_vertex!(Vertex, position);

        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [ 0.0,  0.5] };
        let vertex3 = Vertex { position: [ 0.5, -0.25] };
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = glium::VertexBuffer::new(&self.context, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        target.draw(&vertex_buffer, &indices, &self.program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    }

}

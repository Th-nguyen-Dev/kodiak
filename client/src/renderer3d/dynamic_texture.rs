use crate::renderer:: {
    Framebuffer, Layer, RenderLayer, Renderer, Texture
};
use glam::{Vec3, UVec2};

pub const CUBEMAP_DIRECTIONS: [(Vec3, Vec3); 6] = [
    (Vec3::X, Vec3::NEG_Y),      // Right face - Y is flipped
    (Vec3::NEG_X, Vec3::NEG_Y),  // Left face - Y is flipped
    (Vec3::Y, Vec3::Z),          // Top face - this is correct
    (Vec3::NEG_Y, Vec3::NEG_Z),  // Bottom face - this is correct
    (Vec3::Z, Vec3::NEG_Y),      // Front face - Y is flipped
    (Vec3::NEG_Z, Vec3::NEG_Y)   // Back face - Y is flipped
];

#[derive(Layer)]
pub struct DynamicTextureLayer<L> {
    #[layer]
    pub inner: L,
    framebuffer: Framebuffer,
    resolution: u32,
}

impl<L> DynamicTextureLayer<L> {
    pub fn new(renderer: &Renderer, inner: L, resolution: u32) -> Self {
        let mut framebuffer = Framebuffer::new_with_cubemap(
            renderer,
            UVec2 { x: resolution , y: resolution }, 
            [1,1,1,1]
        );
        let viewport = UVec2::new(resolution, resolution);
        framebuffer.set_viewport(renderer, viewport);

        Self {
            inner,
            framebuffer,
            resolution,
        }
    }
}

impl<L, P> RenderLayer<P> for DynamicTextureLayer<L>
where
    L: RenderLayer<P>
{
    fn render(&mut self, renderer: &Renderer, params: P) {
        let binding = self.framebuffer.bind(renderer);
        binding.clear();
        self.inner.render(renderer, params);
        drop(binding);
    }


}

impl<L> DynamicTextureLayer<L> {
    pub fn render_to_face<P>(&mut self, renderer: &Renderer, params: P, face: u32) 
    where 
        L: RenderLayer<P>
    {
        let binding = self.framebuffer.bind_to_cubemap_face(renderer, face);
        binding.clear();
        self.inner.render(renderer, params);
        drop(binding);
    }

    pub fn to_2Dtexture<P>(&mut self, renderer: &Renderer, params: P) -> Texture 
    where 
        L: RenderLayer<P>
    {
        self.render(renderer, params);
        self.framebuffer.as_texture().clone()

    }

    pub fn to_cube_texture<P>(&mut self, renderer: &Renderer, params: P, face: usize) -> Texture 
    where 
        L: RenderLayer<P>
    {
        let faceU32 = face as u32;
        self.render_to_face(renderer, params, faceU32);
        self.framebuffer.as_texture().clone()
    }
}
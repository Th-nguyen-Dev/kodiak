use crate::renderer:: {
    Framebuffer, Layer, RenderLayer, Renderer, Texture
};
use glam::{Vec3, UVec2};

pub const CUBEMAP_DIRECTIONS: [(Vec3, Vec3); 6] = [
    (Vec3::X, Vec3::Y),      // Right face (positive X)
    (Vec3::NEG_X, Vec3::Y),  // Left face (negative X)
    (Vec3::Y, Vec3::NEG_Z),  // Top face (positive Y)
    (Vec3::NEG_Y, Vec3::Z),  // Bottom face (negative Y)
    (Vec3::Z, Vec3::Y),      // Front face (positive Z)
    (Vec3::NEG_Z, Vec3::Y)   // Back face (negative Z)
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
        let mut framebuffer = Framebuffer::new(renderer, [1, 1, 1, 1], true);
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

impl<L> DynamicTextureLayer<L> 
{
    pub fn to_2Dtexture<P>(&mut self, renderer: &Renderer, params: P) -> Texture 
    where 
        L: RenderLayer<P>
    {
        self.render(renderer, params);
        self.framebuffer.as_texture().clone()

    }

    pub fn to_cubeTexture<P>(&mut self, renderer: &Renderer, params: P, face: usize, mut texture: Texture) -> Texture 
    where 
        L: RenderLayer<P>
    {
        self.render(renderer, params);
        texture = self.framebuffer.bind_to_cubemap(renderer, texture, face);

        texture
    }
}
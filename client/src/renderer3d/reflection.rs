use crate::renderer::{Framebuffer, Layer, RenderLayer, Renderer, Texture};
use glam::{UVec2, Vec3};

/// Defines the directions for each face of a cubemap.
pub const CUBEMAP_DIRECTIONS: [(Vec3, Vec3); 6] = [
    (Vec3::X, Vec3::NEG_Y),     // Right face - Y is flipped
    (Vec3::NEG_X, Vec3::NEG_Y), // Left face - Y is flipped
    (Vec3::Y, Vec3::Z),         // Top face - this is correct
    (Vec3::NEG_Y, Vec3::NEG_Z), // Bottom face - this is correct
    (Vec3::Z, Vec3::NEG_Y),     // Front face - Y is flipped
    (Vec3::NEG_Z, Vec3::NEG_Y), // Back face - Y is flipped
];

#[derive(Layer)]
/// A layer that renders content to a dynamic texture.
/// 
/// This wrapper allows rendering the inner layer to a texture (including cubemaps),
/// which can then be used for various effects like reflections or environment maps.
pub struct ReflectionLayer<L> {
    /// The inner layer to render to the dynamic texture.
    #[layer]
    pub inner: L,
    framebuffer: Option<Framebuffer>,
}

impl<L> ReflectionLayer<L> {
    /// Creates a new dynamic texture layer.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The renderer to use
    /// * `inner` - The inner layer
    /// * `resolution` - The resolution of the texture
    ///
    /// # Returns
    ///
    /// A new `DynamicTextureLayer` instance
    pub fn new(renderer: &Renderer, inner: L, resolution: u32) -> Self {
        let mut framebuffer = Framebuffer::new_with_cubemap(
            renderer,
            UVec2 {
                x: resolution,
                y: resolution,
            },
            [255, 255, 255, 1],
        );
        let viewport = UVec2::new(resolution, resolution);
        framebuffer.set_viewport(renderer, viewport);

        Self {
            inner,
            framebuffer: Some(framebuffer),
        }
    }

    /// Creates a new dynamic texture layer that is disabled.
    ///
    /// # Arguments
    ///
    /// * `_renderer` - The renderer to use
    /// * `inner` - The inner layer
    ///
    /// # Returns
    ///
    /// A new `DynamicTextureLayer` instance that is disabled
    pub fn new_disable(_renderer: &Renderer, inner: L) -> Self {
        Self {
            inner,
            framebuffer: None,
        }
    }
}

impl<L, P> RenderLayer<P> for ReflectionLayer<L>
where
    L: RenderLayer<P>,
{
    fn render(&mut self, renderer: &Renderer, params: P) {
        self.inner.render(renderer, params);
    }
}

impl<L> ReflectionLayer<L> {
    /// Renders the inner layer to a specific face of the cubemap.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The renderer to use.
    /// * `params` - The parameters to pass to the inner layer's render function.
    /// * `face` - The index of the cubemap face to render to (0-5).
    pub fn render_to_face<P>(&mut self, renderer: &Renderer, params: P, face: u32)
    where
        L: RenderLayer<P>,
    {
        if let Some(ref mut framebuffer) = self.framebuffer {
            let binding = framebuffer.bind_to_cubemap_face(renderer, face);
            binding.clear();
            self.inner.render(renderer, params);
            drop(binding);
        }
    }

    /// Renders the inner layer to a specific face of the cubemap.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The renderer to use.
    /// * `params` - The parameters to pass to the inner layer's render function.
    /// * `face` - The index of the cubemap face to render to (0-5).
    pub fn as_cube_texture<P>(&self) -> Option<&Texture>
    where
        L: RenderLayer<P>,
    {
        if let Some(ref framebuffer) = self.framebuffer {
            return Some(framebuffer.as_texture());
        }
        return None;
    }
}

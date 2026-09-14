use smithay::backend::renderer::pixman::PixmanRenderer;

pub struct MyRenderer {
    pub pixman: PixmanRenderer,
}

impl MyRenderer {
    pub fn new() -> Result<Self, smithay::backend::renderer::pixman::PixmanError> {
        println!("MyDE: initializing Pixman renderer...");

        let pixman = PixmanRenderer::new()?;

        println!("MyDE: Pixman renderer initialized.");

        Ok(Self { pixman })
    }
}

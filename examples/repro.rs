use crevice::std140::AsStd140;
use ggez::{
    conf::WindowMode,
    event::{self, EventHandler},
    glam::Vec2,
    graphics::{
        Canvas, Color, DrawParam, Image, Shader, ShaderBuilder, ShaderParams, ShaderParamsBuilder,
    },
    Context, GameResult,
};

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("", "")
        .window_mode(WindowMode {
            width: 256.0,
            height: 256.0,
            ..Default::default()
        })
        .build()?;

    let shader = ShaderBuilder::new()
        .vertex_code(VERTEX_SHADER)
        .build(&ctx)?;

    let shader_params = ShaderParamsBuilder::new(&RunningTime { value: 0.0 }).build(&mut ctx);

    let world = World {
        shaders: [Some(shader), None],
        shader_params: [Some(shader_params), None],
        sprites: [
            Image::from_color(&ctx, 32, 32, Color::MAGENTA),
            Image::from_color(&ctx, 32, 32, Color::YELLOW),
        ],
        ui_bg: Image::from_color(&ctx, 256, 32, Color::WHITE),
    };

    event::run(ctx, event_loop, world);
}

/// Container for everything to be drawn. The `shaders`, `shader_params` and `sprites` arrays can
/// be considered to be a trivial ECS-like structure: index 0 for all 3 represents one "entity" with
/// its sprite, shader, and params, then index 1 represents another entity.
struct World {
    shaders: [Option<Shader>; 2],
    shader_params: [Option<ShaderParams<RunningTime>>; 2],
    sprites: [Image; 2],
    /// The "UI" for the game, simply a white rectangle to be drawn at the top of the window.
    ui_bg: Image,
}

impl EventHandler for World {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // If either of the 2 functions below are commented out, the other runs without panicking.
        // While both are active however, there is a panic.
        self.draw_world(ctx)?;
        self.draw_ui(ctx)?;

        Ok(())
    }

    /// Nothing in this function affects the panic, it just makes it obvious that the shader and
    /// params are active and working.
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let running_time = ctx.time.time_since_start().as_secs_f32();

        if let Some(params) = self.shader_params[0].as_mut() {
            params.set_uniforms(
                ctx,
                &RunningTime {
                    value: running_time,
                },
            );
        }

        Ok(())
    }
}

impl World {
    fn draw_world(&self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        for index in 0..2 {
            if let Some(shader) = &self.shaders[index] {
                canvas.set_shader(shader);

                if let Some(params) = &self.shader_params[index] {
                    canvas.set_shader_params(params);
                }
            } else {
                canvas.set_default_shader();
            }

            canvas.draw(
                &self.sprites[index],
                Vec2::new(index as f32 * 64.0 + 32.0, 128.0),
            );
        }

        canvas.finish(ctx)
    }

    fn draw_ui(&self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, None);

        canvas.draw(&self.ui_bg, DrawParam::new());

        canvas.finish(ctx)
    }
}

#[derive(AsStd140)]
/// Simple `Uniform` to pass as a shader param.
struct RunningTime {
    value: f32,
}

/// Simple vertex shader which takes the `RunningTime` struct as a uniform.
const VERTEX_SHADER: &str = "
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
     @location(0) uv: vec2<f32>,
     @location(1) color: vec4<f32>,
 }
 
 struct GgezDrawUniforms {
     color: vec4<f32>,
     src_rect: vec4<f32>,
     transform: mat4x4<f32>,
 }
 
 struct RunningTime {
  value: f32,
 }

 @group(0) @binding(0)
 var<uniform> ggez_uniforms: GgezDrawUniforms;
 
 @group(3) @binding(0)
 var<uniform> running_time: RunningTime;
 
 @vertex
 fn vs_main(
     @location(0) position: vec2<f32>,
     @location(1) uv: vec2<f32>,
     @location(2) color: vec4<f32>,
 ) -> VertexOutput {
     var out: VertexOutput;
 
     out.position = ggez_uniforms.transform * vec4<f32>(position, 0.0, 1.0);
     
     out.position.y += sin(running_time.value * 3.0) / 2.0;
 
     out.uv = uv;
     out.color = color;
     
     return out;
 } 
";

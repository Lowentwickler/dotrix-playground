use dotrix::pbr::{self, Model, Light, Material};

use dotrix::{Dotrix,
    Application,
    CubeMap,
    Pipeline,
    Id,
    Assets,
    Globals,
    Camera,
    Renderer,
    World
};
use dotrix::assets::{ Mesh, Shader };
use dotrix::ecs::{ Mut, Const, System,RunLevel };
use dotrix::renderer::{
    BindGroup,
    Binding,
    DepthBufferMode,
    PipelineLayout,
    PipelineOptions,
    Sampler,
    Stage,
    UniformBuffer
};

use dotrix::math::{Point3, Mat4};

pub const PIPELINE_LABEL: &str = "triangle";




fn main() {
    Dotrix::application("shader test!")
        .with_system(System::from(startup).with(RunLevel::Startup))
        .with_system(System::from(render).with(RunLevel::Render))
        .with_service(Camera{distance:13.5,
                            y_angle:0.5,
                            xz_angle:0.0,
                            target:Point3::new(0.0,1.5,0.0),
                            ..Default::default()}
                        )
        .with(pbr::extension)
        .run();
}


#[derive(Default)]
pub struct SkyBox {
    pub view_range: f32,
    pub uniform: UniformBuffer,
}

/// Skybox startup system
pub fn startup(
    mut assets: Mut<Assets>,
    mut world: Mut<World>,
    renderer: Const<Renderer>,
) {

    // generate mesh
    let mut mesh = Mesh::default();
    mesh.with_vertices(&[
        // front
        // WRONG:
        // [-1.0, -0.0, 1.0], [1.0, -0.0, 1.0], [1.0, 0.0, 1.0],
        // Correct:
        [-1.0, -1.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0],
    ]);
    mesh.with_indices(&[
        0, 1, 2,
    ]);

    mesh.load(&renderer);

    assets.store_as(mesh, PIPELINE_LABEL);

    // prepare shader
    let mut shader = Shader {
        name: String::from(PIPELINE_LABEL),
        code: String::from(include_str!("skybox.wgsl")),
        ..Default::default()
    };

    shader.load(&renderer);

    assets.store_as(shader, PIPELINE_LABEL);

    world.spawn(Some(
        (
            SkyBox::default(),
            Pipeline::default(),
        )
    ));
}

/// SkyBox rendering system
pub fn render(
    mut renderer: Mut<Renderer>,
    mut assets: Mut<Assets>,
    camera: Const<Camera>,
    globals: Const<Globals>,
    world: Const<World>,
) {
    let query = world.query::<(&mut SkyBox, &mut Pipeline)>();

    for (skybox, pipeline) in query {
        let proj_view_mx = camera.proj.as_ref().unwrap() * camera.view_matrix_static();

        let uniform = Uniform {
            proj_view: proj_view_mx.into(),
            scale: Mat4::from_scale(skybox.view_range).into()
        };

        if pipeline.shader.is_null() {
            pipeline.shader = assets.find::<Shader>(PIPELINE_LABEL)
                .unwrap_or_else(Id::default);
        }

        // check if model is disabled or already rendered
        if !pipeline.cycle(&renderer) { continue; }

        renderer.load_uniform_buffer(&mut skybox.uniform, bytemuck::cast_slice(&[uniform]));

        let mesh = assets.get(
            assets.find::<Mesh>(PIPELINE_LABEL)
                .expect("...........")
        ).unwrap();

        if !pipeline.ready() {
            if let Some(shader) = assets.get(pipeline.shader) {

                renderer.bind(pipeline, PipelineLayout {
                    label: String::from(PIPELINE_LABEL),
                    mesh,
                    shader,
                    bindings: &[
                        BindGroup::new("Globals", vec![
                            Binding::Uniform("Triangle", Stage::Vertex, &skybox.uniform),
                        ]),

                    ],
                    options: PipelineOptions {
                        depth_buffer_mode: DepthBufferMode::Disabled,
                        ..Default::default()
                    }
                });
            }
        }

        renderer.run(pipeline, mesh);
    }
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
struct Uniform {
    proj_view: [[f32; 4]; 4],
    scale: [[f32; 4]; 4],
}

unsafe impl bytemuck::Zeroable for Uniform {}
unsafe impl bytemuck::Pod for Uniform {}

extern crate nmg_vulkan as nmg;

use nmg::alg;
use nmg::entity;
use nmg::render;
use nmg::components;
use nmg::components::Component;
use nmg::input;
use nmg::debug;

struct Demo {
    objects: Vec<entity::Handle>,
    light: Option<entity::Handle>,
}

impl nmg::Start for Demo {
    fn start(
        &mut self,
        entities:   &mut entity::Manager,
        components: &mut components::Container,
    ) {
        // Instantiate three entities
        let object_0 = entities.add();
        let object_1 = entities.add();
        let object_2 = entities.add();

        // Add transform components
        components.transforms.register(object_0);
        components.transforms.register(object_1);
        components.transforms.register(object_2);

        // Add draw components (using first model)
        components.draws.register(object_0, 0);
        components.draws.register(object_1, 0);
        components.draws.register(object_2, 0);

        // Update demo state
        self.objects.push(object_0);
        self.objects.push(object_1);
        self.objects.push(object_2);

        /* Add point light */

        let light = entities.add();
        components.transforms.register(light);

        components.lights.register(light);
        components.lights.build()
            .point_with_radius(8.0)
            .intensity(2.0)
            .for_entity(light);

        self.light = Some(light);

        /* Set up camera */

        let camera = entities.add();
        components.transforms.register(camera);
        components.cameras.register(camera);

        let camera_position = alg::Vec3::new(-1.0, 0.5, -0.1);
        let target_position = alg::Vec3::new( 0.0, 0.0,  2.0);
        let camera_orientation = alg::Quat::look_at(
            camera_position,
            target_position,
            alg::Vec3::up(),
        );

        components.transforms.set(
            camera,
            camera_position,
            camera_orientation,
            alg::Vec3::one(),
        );
    }
}

impl nmg::Update for Demo {
    #[allow(unused_variables)]
    fn update(
        &mut self,
        time:  f64,
        delta: f64,
        metadata: nmg::Metadata,
        screen_width:  u32,
        screen_height: u32,
        entities:   &mut entity::Manager,
        components: &mut components::Container,
        input: &input::Manager,
        debug: &mut debug::Handler,
    ) {
        let angle = time as f32;

        /* Animate objects */

        components.transforms.set(
            self.objects[0],
            alg::Vec3::new(0., 0., 2.),
            alg::Quat::axis_angle(alg::Vec3::new(-0.5, 1.0, 0.5), angle * 2.),
            alg::Vec3::one(),
        );

        components.transforms.set(
            self.objects[1],
            alg::Vec3::new(-0.8, -1.1, 3.),
            alg::Quat::axis_angle_raw(alg::Vec3::up(), angle),
            alg::Vec3::new(0.9, 0.9, 1.),
        );

        components.transforms.set(
            self.objects[2],
            alg::Vec3::new(1.6, 0.8, 4.),
            alg::Quat::axis_angle_raw(alg::Vec3::fwd(), angle),
            alg::Vec3::new(0.8, 1.2, 1.),
        );

        // Animate light
        components.transforms.set_position(
            self.light.unwrap(),
            alg::Vec3::new(
                0.0,
                1.0 * angle.sin(),
                1.0 * angle.cos(),
            ) + alg::Vec3::fwd() * 1.0,
        );
    }
}

impl nmg::FixedUpdate for Demo {
    #[allow(unused_variables)]
    fn fixed_update(
        &mut self,
        time: f64,
        fixed_delta: f32,
        metadata: nmg::Metadata,
        screen_height: u32,
        screen_width: u32,
        entities: &mut entity::Manager,
        components: &mut components::Container,
        input: &input::Manager,
        debug: &mut debug::Handler,
    ) { }
}

fn main() {
    let demo = Demo { objects: Vec::new(), light: None };
    let model_data = get_models();
    nmg::go(model_data, demo)
}

fn get_models() -> Vec<render::ModelData> {
    let pyramid = render::ModelData::new_with_normals(
        vec![
            render::Vertex::new_raw( 0.0,  0.5,  0.0, 0., 0., 0., 1., 1., 0.),
            render::Vertex::new_raw(-0.5, -0.5, -0.5, 0., 0., 0., 1., 0., 1.),
            render::Vertex::new_raw( 0.5, -0.5, -0.5, 0., 0., 0., 1., 0., 0.),
            render::Vertex::new_raw( 0.5, -0.5,  0.5, 0., 0., 0., 1., 1., 0.),
            render::Vertex::new_raw(-0.5, -0.5,  0.5, 0., 0., 0., 1., 1., 1.),
        ], vec![
            0u32, 2u32, 1u32,
            0u32, 3u32, 2u32,
            0u32, 4u32, 3u32,
            0u32, 1u32, 4u32,
            2u32, 4u32, 1u32,
            3u32, 4u32, 2u32,
        ],
        render::NormalMode::Smooth,
    );

    vec![pyramid]
}

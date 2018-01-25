extern crate nmg_vulkan as nmg;

use nmg::alg;
use nmg::entity;
use nmg::render;
use nmg::graphics;
use nmg::components;
use nmg::components::Component;
use nmg::debug;

/* In debug mode, this demo will render in wireframe, with physics markers.
 * In release mode, it will render nothing!
 */

struct Demo {
    first:  Option<entity::Handle>,
    second: Option<entity::Handle>,
    third:  Option<entity::Handle>,
    last_target: alg::Vec3,
}

impl nmg::Start for Demo {
    fn start(
        &mut self,
        entities: &mut entity::Manager,
        components: &mut components::Container,
    ) {
        let first = entities.add();
        components.transforms.register(first);
        components.softbodies.register(first);
        components.softbodies.init_limb(first, 10.0, 1.0, alg::Vec3::one());

        let second = entities.add();
        components.transforms.register(second);
        components.softbodies.register(second);
        components.softbodies.init_limb(second, 10.0, 1.0, alg::Vec3::one());

        let third = entities.add();
        components.transforms.register(third);
        components.softbodies.register(third);
        components.softbodies.init_limb(third, 10.0, 1.0, alg::Vec3::one());

        /* Create joints */

        components.softbodies.add_joint(
            first, second,
            (-45.0, 45.0),
            (-45.0, 45.0),
            (-45.0, 45.0),
        );

        components.softbodies.add_joint(
            second, third,
            (-45.0, 45.0),
            (-45.0, 45.0),
            (-45.0, 45.0),
        );

        /* Add planes */

        components.softbodies.add_plane(
            alg::Plane::new(alg::Vec3::up(), 0.0)
        );

        components.softbodies.add_plane(
            alg::Plane::new(-alg::Vec3::one(), 2.0)
        );

        components.softbodies.add_plane(
            alg::Plane::new(alg::Vec3::right(), 3.0)
        );

        components.softbodies.add_plane(
            alg::Plane::new(alg::Vec3::fwd(), 3.0)
        );

        components.softbodies.add_plane(
            alg::Plane::new(-alg::Vec3::right(), 3.0)
        );

        self.first = Some(first);
        self.second = Some(second);
        self.third = Some(third);
    }
}

impl nmg::Update for Demo {
    #[allow(unused_variables)]
    fn update(
        &mut self,
        time: f64,
        delta: f64,
        metadata: nmg::Metadata,
        screen_height: u32,
        screen_width: u32,
        entities: &mut entity::Manager,
        components: &mut components::Container,
        debug: &mut debug::Handler,
    ) -> render::SharedUBO {
        /* Debug */

        debug.clear_lines();

        // Ground plane
        debug.add_cross(
            alg::Vec3::zero(),
            4.0,
            graphics::Color::gray(),
        );

        // Draw softbodies
        components.softbodies.draw_all_debug(debug);

        let shared_ubo = {
            let camera_position =
                  alg::Mat::rotation(0.0, 90_f32.to_radians(), 0.0)
                * alg::Mat::translation(-3.0, 3.0, -6.0)
                * alg::Vec3::one();

            let target = {
                let new_target = (
                      components.transforms.get_position(self.first.unwrap())
                    + components.transforms.get_position(self.second.unwrap())
                    + components.transforms.get_position(self.third.unwrap())
                ) * 0.33;

                self.last_target.lerp(new_target, delta as f32)
            };

            self.last_target = target;

            let view = alg::Mat::look_at_view(
                camera_position,
                target,
                alg::Vec3::up(),
            );

            let projection = {
                alg::Mat::perspective(
                    60.0,
                    screen_width as f32 / screen_height as f32,
                    0.01,
                    8.0,
                )
            };

            render::SharedUBO::new(view, projection)
        };

        shared_ubo
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
        debug: &mut debug::Handler,
    ) { }
}

fn main() {
    let demo = Demo {
        first:  None,
        second: None,
        third:  None,
        last_target: alg::Vec3::zero(),
    };

    nmg::go(vec![], demo)
}

use alg;
use entity;
use components;

pub struct Transform {
          position: alg::Vec3,
    local_position: alg::Vec3,
          orientation: alg::Quat,
    local_orientation: alg::Quat,
          scale: alg::Vec3,
    local_scale: alg::Vec3,

    parent: Option<usize>,
    children: Vec<usize>,
    cached_transform: alg::Mat4,
}

impl Transform {
    fn blank(child_hint: usize) -> Transform {
        Transform {
                  position: alg::Vec3::zero(),
            local_position: alg::Vec3::zero(),
                  orientation: alg::Quat::id(),
            local_orientation: alg::Quat::id(),
                  scale: alg::Vec3::one(),
            local_scale: alg::Vec3::one(),

            parent: None,
            children: Vec::with_capacity(child_hint),
            cached_transform: alg::Mat4::id(),
        }
    }

    /// Set/update transform with respect to parent
    fn update_cached(&mut self, manager: &Manager) {
        debug_assert!(self.parent.is_some());
        let parent = manager.instances[self.parent.unwrap()].as_ref().unwrap();

        // Rebuild cached transform for this instance
        let transform =
            parent.cached_transform
            * alg::Mat4::transform(
                self.local_position,
                self.local_orientation,
                self.local_scale,
            );

        /* Assign transform data */

        let scale = transform.to_scale();
        self.scale = scale;

        self.orientation = transform.to_rotation_raw(scale).to_quat();
        self.position = transform * alg::Vec3::zero();
        self.cached_transform = transform;
    }

    /// Recursively call `update_cached()` on all children
    fn update_children(&self, manager: &mut Manager) {
        for child_index in &self.children {
            let child = unsafe {
                let ptr = manager.instances.as_mut_ptr()
                    .offset(*child_index as isize);
                (*ptr).as_mut().unwrap()
            };

            child.update_cached(manager);
            child.update_children(manager);
        }
    }
}

// Data layout assumes that almost all entities will have this component
pub struct Manager {
    instances: Vec<Option<Transform>>,
}

impl components::Component for Manager {
    fn register(&mut self, entity: entity::Handle) {
        debug_assert!(self.positions.len() == self.orientations.len());
        debug_assert!(self.orientations.len() == self.scales.len());

        let i = entity.get_index() as usize;

        // Resize array to fit new entity
        loop {
            if i >= self.positions.len() {
                self.positions.push(alg::Vec3::zero());
                self.orientations.push(alg::Quat::id());
                self.scales.push(alg::Vec3::one());

                continue;
            }

            break;
        }
    }

    // TODO: This currently only returns the length of the underlying data
    // structure, not the count of the registered entities
    fn count(&self) -> usize {
        self.positions.len()
    }
}

impl Manager {
    pub fn new(hint: usize) -> Manager {
        Manager {
            instances: Vec::with_capacity(hint),
        }
    }

    pub fn set(
        &mut self,
        entity: entity::Handle,
        position: alg::Vec3,
        orientation: alg::Quat,
        scale: alg::Vec3,
    ) {
        let i = entity.get_index() as usize;

        debug_assert!(i < self.positions.len());

        self.positions[i] = position;
        self.orientations[i] = orientation;
        self.scales[i] = scale;
    }

    pub fn get(&self, entity: entity::Handle) -> (
        alg::Vec3,
        alg::Quat,
        alg::Vec3,
    ) {
        let i = entity.get_index() as usize;
        debug_assert!(i < self.positions.len());

        (
            self.positions[i],
            self.orientations[i],
            self.scales[i],
        )
    }

    pub fn get_position(&self, entity: entity::Handle) -> alg::Vec3 {
        let i = entity.get_index() as usize;
        debug_assert!(i < self.positions.len());

        self.positions[i]
    }

    pub fn get_orientation(&self, entity: entity::Handle) -> alg::Quat {
        let i = entity.get_index() as usize;
        debug_assert!(i < self.positions.len());

        self.orientations[i]
    }

    pub fn get_scale(&self, entity: entity::Handle) -> alg::Vec3 {
        let i = entity.get_index() as usize;
        debug_assert!(i < self.positions.len());

        self.scales[i]
    }

    pub fn set_position(
        &mut self,
        entity: entity::Handle,
        position: alg::Vec3,
    ) {
        let i = entity.get_index() as usize;
        debug_assert!(i < self.positions.len());
        self.positions[i] = position;
    }

    pub fn set_orientation(
        &mut self,
        entity: entity::Handle,
        orientation: alg::Quat,
    ) {
        let i = entity.get_index() as usize;
        debug_assert!(i < self.orientations.len());
        self.orientations[i] = orientation;
    }

    pub fn set_scale(
        &mut self,
        entity: entity::Handle,
        scale: alg::Vec3,
    ) {
        let i = entity.get_index() as usize;
        debug_assert!(i < self.scales.len());
        self.scales[i] = scale;
    }

    /* "Unsafe" methods for components with similar data layouts.
     * These technically invalidate the ECS model but are used
     * for performance purposes.
     */

    pub(super) fn get_position_raw(&self, index: usize) -> alg::Vec3 {
        self.positions[index]
    }

    pub(super) fn get_orientation_raw(&self, index: usize) -> alg::Quat {
        self.orientations[index]
    }

    pub(super) fn set_position_raw(
        &mut self,
        index: usize,
        value: alg::Vec3,
    ) {
        self.positions[index] = value;
    }

    pub(super) fn set_orientation_raw(
        &mut self,
        index: usize,
        value: alg::Quat,
    ) {
        self.orientations[index] = value;
    }
}

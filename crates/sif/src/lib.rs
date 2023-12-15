use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Property {}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(usize);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entity {
    id: EntityId,
    properties: Vec<Property>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeometryId(usize);

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vector2(f64, f64, f64);

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vector3(f64, f64, f64);

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Plane {
    normal: Vector3,
    distance: f64,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct BrushPlane {
    plane: Plane,
    texture: TextureId,
    tangent: Vector3,
    binormal: Vector3,
    offset: Vector2,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Geometry {
    Brush {
        id: GeometryId,
        planes: Vec<BrushPlane>,
    },
    Mesh {
        id: GeometryId,
        vertices: Vec<Vector3>,
        normals: Vec<Vector3>,
        tangents: Vec<Vector3>,
        indices: Vec<usize>,
        texture: TextureId,
    },
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureId(usize);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Texture {
    id: TextureId,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SceneTree {
    id: EntityId,
    children: Vec<SceneTree>,
}

trait Modifier: Debug {
    fn modify(&mut self, scene: &mut SceneData);
}

#[derive(Debug, Default, Clone)]
pub struct SceneData {
    entities: Vec<Entity>,
    geometry: Vec<Geometry>,
    textures: Vec<Texture>,
    scene_tree: Option<SceneTree>,
}

#[derive(Debug, Default)]
pub struct Scene {
    data: SceneData,
    modifiers: Vec<Box<dyn Modifier>>,
}

impl Scene {
    pub fn build(&mut self) -> SceneData {
        let mut data = self.data.clone();
        for modifier in &mut self.modifiers {
            modifier.modify(&mut data);
        }
        data
    }
}

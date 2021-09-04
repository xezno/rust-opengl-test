// ============================================================================
//
// model.rs
//
// Purpose: Loads meshes from a GLTF file.
//
// ============================================================================

use glam::*;

use gl::types::*;

use super::{camera::Camera, scene::LoadedScene, transform::Transform};
use crate::render::{material::Material, mesh::Mesh, shader::Shader, texture::Texture};

pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub texcoord: Vec2,
    // pub tangent: Vec3,
    // pub bitangent: Vec3,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub transform: Transform,
    pub material: Material,

    pub diffuse_texture: Texture,
}

impl Model {
    pub fn new(gltf_path: &str) -> Model {
        let mut model = Model {
            meshes: Vec::new(),
            transform: Transform::default(),
            material: Material::default(),

            diffuse_texture: Texture::default(),
        };

        log::info!("Loading gltf from '{}'", gltf_path);

        let (gltf, buffers, _) = gltf::import(&std::path::Path::new(gltf_path)).unwrap();

        let scene = gltf.default_scene().unwrap();
        process_gltf_node(scene.nodes().next().unwrap(), &mut model, &gltf, &buffers);

        return model;
    }

    pub fn render(&self, scene: &LoadedScene, shader: &mut Shader, camera: &mut Camera) {
        for mesh in &self.meshes {
            shader.bind();
            {
                // Calc model matrix
                let mut model_mat = Mat4::from_translation(self.transform.position);
                model_mat *= Mat4::from_scale(self.transform.scale);
                model_mat *= Mat4::from_quat(self.transform.rotation);

                // Submit shader uniforms
                shader.set_mat4("uProjViewMat", &camera.proj_view_mat);
                shader.set_mat4("uModelMat", &model_mat);
                shader.set_vec3("uCamPos", &camera.position);

                // Submit scene uniforms
                shader.set_vec3(
                    "lightingInfo.vLightDir",
                    &scene.light.direction.to_euler(EulerRot::XYZ).into(),
                );
                shader.set_vec3("lightingInfo.vLightColor", &scene.light.color);

                // Submit material uniforms
                shader.set_f32("materialInfo.fSpecular", self.material.specular);

                shader.set_i32("tDiffuseTex", 0);
            }
            self.diffuse_texture.bind();
            mesh.render();
        }
    }
}

fn process_gltf_node(
    node: gltf::Node,
    model: &mut Model,
    gltf: &gltf::Document,
    buffers: &[gltf::buffer::Data],
) -> () {
    if node.mesh().is_some() {
        process_gltf_mesh(&node.mesh().unwrap(), model, gltf, buffers);
    }

    for child in node.children() {
        process_gltf_node(child, model, gltf, buffers);
    }
}

fn process_gltf_mesh(
    mesh: &gltf::Mesh,
    model: &mut Model,
    gltf: &gltf::Document,
    buffers: &[gltf::buffer::Data],
) -> () {
    let mut gl_vertices: Vec<GLfloat> = Vec::new();
    let mut gl_normals: Vec<GLfloat> = Vec::new();
    let mut gl_texcoords: Vec<GLfloat> = Vec::new();
    let mut gl_indices: Vec<GLuint> = Vec::new();
    for primitive in mesh.primitives() {
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

        let positions = reader.read_positions().unwrap().collect::<Vec<[f32; 3]>>();
        let normals = reader.read_normals().unwrap().collect::<Vec<[f32; 3]>>();
        let texcoords = reader
            .read_tex_coords(0)
            .unwrap()
            .into_f32()
            .collect::<Vec<[f32; 2]>>();

        let indices = reader
            .read_indices()
            .unwrap()
            .clone()
            .into_u32()
            .collect::<Vec<GLuint>>();

        let mesh_name = mesh.name().unwrap_or("Unnamed");
        log::trace!("Mesh {} index count: {}", mesh_name, indices.len());
        log::trace!("Mesh {} has {:?} positions", mesh_name, positions.len());

        let start_index: u32 = (gl_vertices.len() / 3) as u32;

        for i in 0..positions.len() {
            let position = positions[i];
            let normal = normals[i];
            let texcoord = texcoords[i];

            gl_vertices.push(-position[2]); // Flip height
            gl_vertices.push(position[1]);
            gl_vertices.push(position[0]);

            gl_normals.push(-normal[2]); // Flip height
            gl_normals.push(normal[1]);
            gl_normals.push(normal[0]);

            gl_texcoords.push(texcoord[0]);
            gl_texcoords.push(texcoord[1]);
        }

        for i in 0..indices.len() {
            gl_indices.push(indices[i] + start_index);
        }
    }
    let mesh = Mesh::new(gl_vertices, gl_normals, gl_texcoords, gl_indices);
    model.meshes.push(mesh);
}

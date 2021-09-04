// ============================================================================
//
// model.rs
//
// Purpose: Loads meshes from an OBJ file.
//
// ============================================================================

use glam::*;

use gl::types::*;
use gltf::{mesh::util::ReadTexCoords, Gltf};

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

        let mut gl_vertices: Vec<GLfloat> = Vec::new();
        let mut gl_normals: Vec<GLfloat> = Vec::new();
        let mut gl_texcoords: Vec<GLfloat> = Vec::new();
        let mut gl_indices: Vec<GLuint> = Vec::new();

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                let mut positions = reader.read_positions().unwrap();
                let mut normals = reader.read_normals().unwrap();

                let mut indices = reader.read_indices().unwrap().clone().into_u32();

                let texcoords = reader.read_tex_coords(0).unwrap();

                log::trace!("Mesh has {:?} positions", positions.len());

                for _ in 0..positions.len() {
                    let wrapped_position = positions.nth(0);
                    let wrapped_normal = normals.nth(0);
                    let wrapped_texcoord = texcoords.clone().into_f32().nth(0);

                    if !wrapped_position.is_none() {
                        let position = wrapped_position.unwrap();
                        gl_vertices.push(position[0]);
                        gl_vertices.push(position[1]);
                        gl_vertices.push(-position[2]); // Flip height

                        if !wrapped_normal.is_none() {
                            let normal = wrapped_normal.unwrap();
                            gl_normals.push(normal[0]);
                            gl_normals.push(normal[1]);
                            gl_normals.push(-normal[2]); // Flip height
                        }

                        if !wrapped_texcoord.is_none() {
                            let texcoord = wrapped_texcoord.unwrap();

                            gl_texcoords.push(texcoord[0]);
                            gl_texcoords.push(texcoord[1]);
                        }
                    }
                }

                for _ in 0..indices.len() {
                    let wrapped_index = indices.nth(0);
                    if !wrapped_index.is_none() {
                        let index = wrapped_index.unwrap();
                        gl_indices.push(index as GLuint);
                    }
                }
            }
        }

        let mesh = Mesh::new(gl_vertices, gl_normals, gl_texcoords, gl_indices);
        model.meshes.push(mesh);

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

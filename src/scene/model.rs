// ============================================================================
//
// model.rs
//
// Purpose: Loads meshes from a GLTF file.
//
// ============================================================================

use glam::*;

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
}

impl Model {
    pub fn new(gltf_path: &str) -> Model {
        let mut model = Model {
            meshes: Vec::new(),
            transform: Transform::default(),
        };

        log::info!("Loading gltf from '{}'", gltf_path);

        let (gltf, buffers, _) = gltf::import(&std::path::Path::new(gltf_path)).unwrap();

        let scene = gltf.default_scene().unwrap();
        process_gltf_node(
            gltf_path,
            scene.nodes().next().unwrap(),
            &mut model,
            &gltf,
            &buffers,
        );

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
                shader.set_f32("materialInfo.fSpecular", 0.0);
                shader.set_i32("tDiffuseTex", 0);
            }
            mesh.diffuse_texture.bind();
            mesh.render();
        }
    }
}

fn process_gltf_node(
    gltf_path: &str,
    node: gltf::Node,
    model: &mut Model,
    gltf: &gltf::Document,
    buffers: &[gltf::buffer::Data],
) -> () {
    if node.mesh().is_some() {
        process_gltf_mesh(gltf_path, &node.mesh().unwrap(), model, gltf, buffers);
    }

    for child in node.children() {
        process_gltf_node(gltf_path, child, model, gltf, buffers);
    }
}

fn process_gltf_mesh(
    gltf_path: &str,
    mesh: &gltf::Mesh,
    model: &mut Model,
    _gltf: &gltf::Document,
    buffers: &[gltf::buffer::Data],
) -> () {
    for primitive in mesh.primitives() {
        let mut gl_vertices: Vec<f32> = Vec::new();
        let mut gl_normals: Vec<f32> = Vec::new();
        let mut gl_texcoords: Vec<f32> = Vec::new();
        let mut gl_indices: Vec<u32> = Vec::new();

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
            .collect::<Vec<u32>>();

        let mesh_name = mesh.name().unwrap_or("Unnamed");
        log::trace!("Mesh {} index count: {}", mesh_name, indices.len());
        log::trace!("Mesh {} has {:?} positions", mesh_name, positions.len());

        for i in 0..positions.len() {
            let position = positions[i];
            let normal = normals[i];
            let texcoord = texcoords[i];

            // Triangle order swap - gltf uses a different winding order?
            gl_vertices.push(-position[2]); // Flip height
            gl_vertices.push(position[1]);
            gl_vertices.push(position[0]);

            // We're Z-up, so switch Y with Z
            gl_normals.push(normal[0]);
            gl_normals.push(-normal[2]); // Flip height
            gl_normals.push(normal[1]);

            // Texcoords remain the same
            gl_texcoords.push(texcoord[0]);
            gl_texcoords.push(texcoord[1]);
        }

        for i in 0..indices.len() {
            gl_indices.push(indices[i]);
        }

        let diffuse = primitive
            .material()
            .pbr_metallic_roughness()
            .base_color_texture();

        let diffuse_texture: Texture;

        if diffuse.is_some() {
            // Get image data from buffer view
            let image_source = diffuse.unwrap().texture().source().source();
            match image_source {
                gltf::image::Source::Uri { uri, .. } => {
                    let gltf_dir = std::path::Path::new(gltf_path);
                    let texture_path = gltf_dir.with_file_name(uri);

                    diffuse_texture = Texture::new(texture_path.to_str().unwrap());
                }
                gltf::image::Source::View { .. } => {
                    todo!();
                }
            }
        } else {
            diffuse_texture = Texture::new("content/textures/missing.png");
        }

        let mesh = Mesh::new(
            gl_vertices,
            gl_normals,
            gl_texcoords,
            gl_indices,
            diffuse_texture,
        );
        model.meshes.push(mesh);
    }
}

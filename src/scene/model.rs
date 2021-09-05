// ============================================================================
//
// model.rs
//
// Purpose: Loads meshes from a GLTF file.
//
// ============================================================================

use glam::*;
use gltf::material::NormalTexture;

use super::{camera::Camera, scene::LoadedScene, transform::Transform};
use crate::render::{mesh::Mesh, shader::Shader, texture::Texture};

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

    pub fn render(
        &self,
        scene: &LoadedScene,
        shader: &mut Shader,
        proj_view_mat: &glam::Mat4,
        cam_pos: &Vec3,
    ) {
        for mesh in &self.meshes {
            shader.bind();
            {
                // Calc model matrix
                let mut model_mat = Mat4::from_translation(self.transform.position);
                model_mat *= Mat4::from_scale(self.transform.scale);
                model_mat *= Mat4::from_quat(self.transform.rotation);

                // Submit shader uniforms
                shader.set_mat4("uProjViewMat", proj_view_mat);
                shader.set_mat4("uModelMat", &model_mat);
                shader.set_vec3("uCamPos", cam_pos);

                // Submit scene uniforms
                shader.set_vec3(
                    "lightingInfo.vLightDir",
                    &scene.sun_light.direction.to_euler(EulerRot::XYZ).into(),
                );
                shader.set_vec3("lightingInfo.vLightColor", &scene.sun_light.color);

                // Submit material uniforms
                shader.set_f32("materialInfo.fSpecular", 0.0);

                shader.set_i32("materialInfo.tDiffuseTex", 0);
                mesh.diffuse_texture.bind(Some(gl::TEXTURE0));

                shader.set_i32("materialInfo.tNormalTex", 1);
                mesh.normal_texture.bind(Some(gl::TEXTURE1));

                shader.set_i32("materialInfo.tOrmTex", 2);
                mesh.orm_texture.bind(Some(gl::TEXTURE2));

                shader.set_i32("materialInfo.tEmissiveTex", 3);
                mesh.emissive_texture.bind(Some(gl::TEXTURE3));
            }

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
        let mut gl_tangents: Vec<f32> = Vec::new();

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

        let mut tangents = vec![[1.0; 4]; positions.len()];

        if reader.read_tangents().is_some() {
            tangents = reader.read_tangents().unwrap().collect::<Vec<[f32; 4]>>();
        }

        for i in 0..positions.len() {
            let position = positions[i];
            let normal = normals[i];
            let texcoord = texcoords[i];
            let tangent = tangents[i];

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

            // Flip Y and Z for tangents
            gl_tangents.push(tangent[0]);
            gl_tangents.push(-tangent[2]); // Flip height
            gl_tangents.push(tangent[1]);
        }

        for i in 0..indices.len() {
            gl_indices.push(indices[i]);
        }

        let pbr_material = primitive.material().pbr_metallic_roughness();

        let diffuse = pbr_material.base_color_texture();
        let orm = pbr_material.metallic_roughness_texture();
        let normal = primitive.material().normal_texture();
        let emissive = primitive.material().emissive_texture();

        let diffuse_texture: Texture = process_gltf_texture(gltf_path, diffuse);
        let orm_texture: Texture = process_gltf_texture(gltf_path, orm);
        let normal_texture: Texture = process_gltf_normal_map(gltf_path, normal);
        let emissive_texture: Texture = process_gltf_texture(gltf_path, emissive);

        let mesh = Mesh::new(
            gl_vertices,
            gl_normals,
            gl_texcoords,
            gl_indices,
            gl_tangents,
            diffuse_texture,
            orm_texture,
            normal_texture,
            emissive_texture,
        );
        model.meshes.push(mesh);
    }
}

fn process_gltf_texture(gltf_path: &str, info: Option<gltf::texture::Info>) -> Texture {
    let texture: Texture;

    if info.is_some() {
        // Get image data from buffer view
        let image_source = info.unwrap().texture().source().source();
        match image_source {
            gltf::image::Source::Uri { uri, .. } => {
                let gltf_dir = std::path::Path::new(gltf_path);
                let texture_path = gltf_dir.with_file_name(uri);

                texture = Texture::new(texture_path.to_str().unwrap());
            }
            gltf::image::Source::View { .. } => {
                todo!();
            }
        }
    } else {
        texture = Texture::new("content/textures/missing.png");
    }

    return texture;
}

fn process_gltf_normal_map(gltf_path: &str, normal: Option<NormalTexture>) -> Texture {
    let normal_texture: Texture;

    if normal.is_some() {
        // Get image data from buffer view
        let image_source = normal.unwrap().texture().source().source();
        match image_source {
            gltf::image::Source::Uri { uri, .. } => {
                let gltf_dir = std::path::Path::new(gltf_path);
                let texture_path = gltf_dir.with_file_name(uri);

                normal_texture = Texture::new(texture_path.to_str().unwrap());
            }
            gltf::image::Source::View { .. } => {
                todo!();
            }
        }
    } else {
        normal_texture = Texture::new("content/textures/missing.png");
    }

    return normal_texture;
}

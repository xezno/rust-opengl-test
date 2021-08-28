// ============================================================================
//
// model.rs
//
// Purpose: Loads meshes from an OBJ file.
//
// ============================================================================

use crate::camera::Camera;
use crate::mesh::Mesh;
use crate::shader::Shader;
use crate::transform::Transform;

use glam::*;

use gl::types::*;

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
    pub fn new(obj_path: &str) -> Model {
        let mut model = Model {
            meshes: Vec::new(),
            transform: Transform::IDENTITY,
        };

        println!("Loading OBJ from '{}'", obj_path);

        let obj_file_contents = std::fs::read_to_string(obj_path).unwrap();
        let obj_file = wavefront_obj::obj::parse(obj_file_contents).unwrap();

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<GLuint> = Vec::new();

        for object in obj_file.objects {
            for geometry in object.geometry {
                for shape in geometry.shapes {
                    if let wavefront_obj::obj::Primitive::Triangle(a, b, c) = shape.primitive {
                        for key in &[a, b, c] {
                            let p = object.vertices[key.0];
                            let position = Vec3::new(p.x as f32, p.y as f32, p.z as f32);
                            let vertex_index = vertices.len() as GLuint;

                            let t = object.tex_vertices[key.1.unwrap()];
                            let texcoord = Vec2::new(t.u as f32, t.v as f32);

                            let n = object.normals[key.2.unwrap()];
                            let normal = Vec3::new(n.x as f32, n.y as f32, n.z as f32);

                            let vertex = Vertex {
                                position,
                                normal,
                                texcoord,
                            };

                            vertices.push(vertex);
                            indices.push(vertex_index);
                        }
                    } else {
                        println!("unsupported non-triangle shape");
                    }
                }
            }
        }

        let mut gl_vertices: Vec<GLfloat> = Vec::new();
        let mut gl_normals: Vec<GLfloat> = Vec::new();

        for vertex in vertices {
            gl_vertices.push(vertex.position.x);
            gl_vertices.push(vertex.position.z);
            gl_vertices.push(vertex.position.y);

            gl_normals.push(vertex.normal.x);
            gl_normals.push(vertex.normal.z);
            gl_normals.push(vertex.normal.y);
        }

        let mesh = Mesh::new(gl_vertices, gl_normals);
        model.meshes.push(mesh);

        return model;
    }

    pub fn draw_this(&self, shader: &mut Shader, camera: &mut Camera) {
        for mesh in &self.meshes {
            shader.use_this();
            {
                // Submit camera matrix
                shader.set_mat4("uProjViewMat", &camera.proj_view_mat);

                // Submit model matrix
                let mut model_mat = Mat4::from_translation(self.transform.position);
                model_mat *= Mat4::from_scale(self.transform.scale);
                model_mat *= Mat4::from_quat(self.transform.rotation);

                shader.set_mat4("uModelMat", &model_mat);
            }

            mesh.draw_this();
        }
    }
}

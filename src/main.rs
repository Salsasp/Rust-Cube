use crossterm::{cursor, execute, terminal, style::Print};
use std::{f32::consts::PI, io::{stdout, Write}};
const SCREEN_WIDTH: u32 = 50;
const SCREEN_HEIGHT: u32 = 50;

#[derive(Clone)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    one: f32
}

fn clear_screen() {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
}

fn move_cursor(x: u16, y: u16) {
    execute!(stdout(), cursor::MoveTo(x, y)).unwrap();
}

fn draw_point(x: i32, y: i32, ch: char) {
    if x >= 0 && y >= 0 {
        move_cursor(x as u16, y as u16);
        print!("{}", ch);
    }
}

fn perspective_matrix(fovY:f32, aspect:f32, near:f32, far:f32) -> [[f32;4];4] {
    let matrix = [
        [1.0/(aspect*(fovY/2.0).tan()), 0.0, 0.0, 0.0],
        [0.0, 1.0/(fovY/2.0).tan(), 0.0, 0.0],
        [0.0, 0.0, -((far + near)/(far - near)), -((2.0*far*near)/(far - near))],
        [0.0, 0.0, -1.0, 0.0]
    ];
    return matrix;
}

fn x_rotation_matrix(theta:f32) -> [[f32;4];4] {
    let matrix = [
        [1.0,0.0,0.0,0.0],
        [0.0,theta.cos(),-theta.sin(),0.0],
        [0.0,theta.sin(),theta.cos(),0.0],
        [0.0,0.0,0.0,1.0]
    ];
    return matrix;
}

fn y_rotation_matrix(theta:f32) -> [[f32;4];4] {
    let matrix = [
        [theta.cos(),0.0,theta.sin(),0.0],
        [0.0,1.0,0.0,0.0],
        [-theta.sin(),0.0,theta.cos(),0.0],
        [0.0,0.0,0.0,1.0]
    ];
    return matrix;
}

fn z_rotation_matrix(theta:f32) -> [[f32;4];4] {
    let matrix = [
        [theta.cos(),-theta.sin(),0.0,0.0],
        [theta.sin(),theta.cos(),0.0,0.0],
        [0.0,0.0,1.0,0.0],
        [0.0,0.0,0.0,1.0]    
    ];
    return matrix;
}

fn zero_matrix_4x4() -> [[f32;4];4] {
    let matrix = [
        [0.0,0.0,0.0,0.0],
        [0.0,0.0,0.0,0.0],
        [0.0,0.0,0.0,0.0],
        [0.0,0.0,0.0,0.0]
    ];
    return matrix;
}

fn zero_matrix_3x3() -> [[f32;3];3] {
    let matrix = [
        [0.0,0.0,0.0],
        [0.0,0.0,0.0],
        [0.0,0.0,0.0]
    ];
    return matrix;
}

fn matrix_mult_4x4(matrix1:[[f32;4];4], matrix2:[[f32;4];4]) -> [[f32;4];4] {
    let mut res: [[f32;4];4] = zero_matrix_4x4();
    for k in 0..4 {
        for i in 0..4 {
            for j in 0..4 {
                res[k][i] +=
                matrix1[k][j] * matrix2[j][i]
            }
        }
    }
    return res;
}

fn matrix_mult_3x3(matrix1:[[f32;3];3], matrix2:[[f32;3];3]) -> [[f32;3];3] {
    let mut res: [[f32;3];3] = zero_matrix_3x3();
    for k in 0..3 {
        for i in 0..3 {
            for j in 0..3 {
                res[k][i] +=
                matrix1[k][j] * matrix2[j][i]
            }
        }
    }
    return res;
}

fn matrix_mult_vertices_4x4(vec3: &mut Vec<Vertex>, matrix: [[f32; 4]; 4]) {
    for i in 0..vec3.len() {
        let x = vec3[i].x;
        let y = vec3[i].y;
        let z = vec3[i].z;
        let w = vec3[i].one;

        let new_x = matrix[0][0] * x + matrix[0][1] * y + matrix[0][2] * z + matrix[0][3] * w;
        let new_y = matrix[1][0] * x + matrix[1][1] * y + matrix[1][2] * z + matrix[1][3] * w;
        let new_z = matrix[2][0] * x + matrix[2][1] * y + matrix[2][2] * z + matrix[2][3] * w;
        let new_w = matrix[3][0] * x + matrix[3][1] * y + matrix[3][2] * z + matrix[3][3] * w;

        vec3[i].x = new_x / new_w;
        vec3[i].y = new_y / new_w;
        vec3[i].z = new_z / new_w;
        //draw_point((new_x/new_w) as i32, (new_y/new_w) as i32, '#');
    }
}

fn lerp_vectors(vecs: &mut Vec<Vertex>,lx:i32, rx:i32, ly:i32, ry:i32) {
    for i in 0..vecs.len() {
        let mapped_x = lx as f32 + vecs[i].x * ((rx - lx) as f32);
        let mapped_y = ly as f32 + vecs[i].y * ((ry - ly) as f32);
        vecs[i].x = mapped_x;
        vecs[i].y = mapped_y;
    }
}

fn print_matrix(matrix: [[f32;4];4]) {
    for i in 0..4 {
        for j in 0..4 {
            print!("{} ",matrix[i][j]);
        }
        print!("\n");
    }
}

fn main() {
    let aspect = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;
    let fov:f32 = 60.0;
    let near:f32 = 0.1;
    let far:f32 = 50.0;
    let mut vertices: Vec<Vertex> = vec!(
        Vertex {x:-1.0, y:-1.0, z:1.0, one:1.0}, // front-bottom-left
        Vertex {x:1.0, y:-1.0, z:1.0, one:1.0},
        Vertex {x:1.0, y:1.0, z:1.0, one:1.0},
        Vertex {x:-1.0, y:1.0, z:1.0, one:1.0},
        Vertex {x:-1.0, y:-1.0, z:-1.0, one:1.0},
        Vertex {x:-1.0, y:1.0, z:-1.0, one:1.0},
        Vertex {x:1.0, y:-1.0, z:-1.0, one:1.0},
        Vertex {x:1.0, y:1.0, z:-1.0, one:1.0}
    );
    //print!("Testing matrix mult: \n");
    let mut theta = 0.0;
    let rotation_rate = 0.05;
    let original_vertices = vertices.clone();
    while true {
        std::thread::sleep(std::time::Duration::from_millis(50)); // ~60fps
        clear_screen();
        let mut transformed_vertices = original_vertices.clone();
        if theta > 2.0*PI {theta = 0.0;}
        // Transformation matrix representing 90deg
        // rotation across all axises at once
        let mut transformation = 
        matrix_mult_4x4(
            x_rotation_matrix(theta),
                    y_rotation_matrix(theta)
        );
        transformation = matrix_mult_4x4(transformation, z_rotation_matrix(theta));
        transformation = matrix_mult_4x4(transformation, perspective_matrix(fov, aspect, near, far));
        matrix_mult_vertices_4x4(&mut transformed_vertices, transformation);
        lerp_vectors(&mut transformed_vertices, 0, SCREEN_WIDTH as i32, 0, SCREEN_HEIGHT as i32);
        let min_x = transformed_vertices.iter()
            .map(|v| v.x)
            .fold(f32::INFINITY, |min_val, x| min_val.min(x)).abs();
        let min_y = transformed_vertices.iter()
            .map(|v| v.y)
            .fold(f32::INFINITY, |min_val, y| min_val.min(y)).abs();
        for i in 0..transformed_vertices.len() {
            draw_point((transformed_vertices[i].x + min_x) as i32, (transformed_vertices[i].y + min_y) as i32, '#');
            //println!("Theta: {}",theta);
            //println!("{} {} \n", (transformed_vertices[i].x + min_x) as i32,(transformed_vertices[i].y + min_y) as i32);
        }
        for vec in transformed_vertices.iter() {
            //println!("{} {} {} {} \n", vec.x,vec.y,vec.z,vec.one);
        }
        //print_matrix(transformation);
        theta+=rotation_rate;
    }
}

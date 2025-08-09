use std::{thread, time::Duration};
const SCREEN_WIDTH: usize = 90;
const SCREEN_HEIGHT: usize = 60;
const CAM_DISTANCE:f32 = 50.0;
const K1:f32 = 20.0; // Depth scaling constant
const CUBE_WIDTH: u32 = 15;
const OFFSET_X:i32 = CUBE_WIDTH as i32;
const OFFSET_Y:i32 = CUBE_WIDTH as i32;
const ROTATION_SPEED: f32 = 0.2;

#[derive (Clone, Copy, Debug)]
struct CubeInfo {
    A:f32,
    B:f32,
    C:f32,
    x:f32,
    y:f32,
    z:f32,
    z_recip:f32, // 1 over z
    xp:u32, // x pixel coord
    yp:u32, // y pixel coord
    idx:usize // Index used to access relevant buffer areas.
}             // This probably shouldn't be a field, but I'm too lazy to refactor the code :)

struct Screen {
    zbuffer:[f32; SCREEN_WIDTH*SCREEN_HEIGHT],
    pixbuffer:[char; SCREEN_WIDTH*SCREEN_HEIGHT],
}

// All rotation calculations are flattened to the specific equation for an xyz rotation
// rather than using the actual matrices, thus skipping the annoying vector math!

fn calculate_x(cube:&CubeInfo, i:f32, j:f32, k:f32) -> f32 {
    return j * cube.A.sin() * cube.B.sin() * cube.C.cos() - k * cube.A.cos() * cube.B.sin() * cube.C.sin()
    + j * cube.A.cos() * cube.C.sin() + k * cube.A.sin() * cube.C.sin() + i * cube.B.cos() * cube.C.cos();
}

fn calculate_y(cube:&CubeInfo, i:f32, j:f32, k:f32) -> f32 {
    return j* cube.A.cos() * cube.C.cos() + k * cube.A.sin() * cube.C.cos() -
    j * cube.A.sin() * cube.B.sin() * cube.C.sin() + k * cube.A.cos() * cube.B.sin() * cube.C.sin() -
    i * cube.B.cos() * cube.C.sin();
}

fn calculate_z(cube:&CubeInfo, i:f32, j:f32, k:f32) -> f32 {
    return k * cube.A.cos() * cube.B.cos() - j * cube.A.sin() * cube.B.cos() + i * cube.B.sin();
}

fn calculate_surface(screen:&mut Screen,cube:&mut CubeInfo, cube_x:f32, cube_y:f32, cube_z:f32, c:char) {
    cube.x = calculate_x(cube, cube_x, cube_y, cube_z);
    cube.y = calculate_y(cube, cube_x, cube_y, cube_z);
    cube.z = calculate_z(cube, cube_x, cube_y, cube_z) + CAM_DISTANCE; 

    cube.z_recip = 1.0/cube.z;

    // Buffer coordinates
    cube.xp = (SCREEN_WIDTH as f32 / 2.0 + (OFFSET_X as f32) + K1 * cube.z_recip * cube.x) as u32;
    cube.yp = (SCREEN_HEIGHT as f32 / 2.0 + (OFFSET_Y as f32) + K1 * cube.z_recip * cube.y) as u32;

    cube.idx = (cube.xp + (cube.yp * SCREEN_WIDTH as u32)) as usize; // Index for the pixel-buffer
    if cube.idx >= 0 as usize && (cube.idx as u32) < (SCREEN_WIDTH as u32 * SCREEN_HEIGHT as u32) {
        if cube.z_recip > screen.zbuffer[cube.idx] as f32 {
            screen.zbuffer[cube.idx] = cube.z_recip;
            screen.pixbuffer[cube.idx] = c;
        }
    }
}

fn main() {
    print!("\x1b[2J"); // ANSI escape code that clears an ANSI compatible terminal
    let mut A = 0.0;
    let mut B = 0.0;
    let mut C = 0.0;
    loop {
        // Reset the screen buffers every loop
        let tmp1:[char; SCREEN_WIDTH * SCREEN_HEIGHT] = [' '; SCREEN_WIDTH * SCREEN_HEIGHT];
        let tmp2:[f32; SCREEN_WIDTH * SCREEN_HEIGHT] = [0.0; SCREEN_WIDTH * SCREEN_HEIGHT];
        let mut screen:Screen = Screen {
            zbuffer: tmp2,
            pixbuffer: tmp1,
        };
        // Reset the cube info every loop
        let mut cube:CubeInfo = CubeInfo {
            A: A.clone(),
            B: B.clone(),
            C: C.clone(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            z_recip: 0.0,
            xp: 0,
            yp: 0,
            idx: 0,
        };
        // Rust doesn't support C style for loops so I'm stuck with this abomination :(
        let mut cube_x = -(CUBE_WIDTH as f32); let mut cube_y = -(CUBE_WIDTH as f32);
        while cube_x < CUBE_WIDTH as f32 {
            while cube_y < CUBE_WIDTH as f32 {
                calculate_surface(&mut screen, &mut cube, cube_x, cube_y, -(CUBE_WIDTH as f32), '*');
                calculate_surface(&mut screen, &mut cube, CUBE_WIDTH as f32, cube_y, cube_x, '&');
                calculate_surface(&mut screen, &mut cube, -(CUBE_WIDTH as f32), cube_y, -cube_x, '$');
                calculate_surface(&mut screen, &mut cube, -cube_x, cube_y, CUBE_WIDTH as f32, '#');
                calculate_surface(&mut screen, &mut cube, cube_x, -(CUBE_WIDTH as f32), -cube_y, '@');
                calculate_surface(&mut screen, &mut cube, cube_x, CUBE_WIDTH as f32, cube_y, '+');
                cube_y += ROTATION_SPEED;
            }
            cube_y = -(CUBE_WIDTH as f32);
            cube_x += ROTATION_SPEED;
        }
        print!("\x1b[H"); // x1b is the 'esc' key, and [H is an ANSI command that returns cursor to home
        for k in 0..SCREEN_WIDTH*SCREEN_HEIGHT {
            let tmp:char;
            if (k % SCREEN_WIDTH) != 0 {tmp = screen.pixbuffer[k]} else {tmp = '\n'};
            print!("{}",tmp);
        }
        A += 0.05;
        B += 0.05;
        C += 0.01;
        thread::sleep(Duration::new(0,500));
    }
}

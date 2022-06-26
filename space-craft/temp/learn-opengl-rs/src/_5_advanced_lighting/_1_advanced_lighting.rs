#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use common::{process_events, loadTexture};
use shader::Shader;
use camera::Camera;
use camera::Camera_Movement::*;

use cgmath::{Matrix4, vec3,  Deg, perspective, Point3};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

pub fn main_5_1() {
    let mut blinn = false;
    let mut blinnKeyPressed = false;

    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    let mut firstMouse = true;
    let mut lastX: f32 = SCR_WIDTH as f32 / 2.0;
    let mut lastY: f32 = SCR_HEIGHT as f32 / 2.0;

    // timing
    let mut deltaTime: f32; // time between current frame and last frame
    let mut lastFrame: f32 = 0.0;

    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    // tell GLFW to capture our mouse
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shader, planeVBO, planeVAO, floorTexture) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        // build and compile shaders
        // ------------------------------------
        let shader = Shader::new(
            "src/_5_advanced_lighting/shaders/1.advanced_lighting.vs",
            "src/_5_advanced_lighting/shaders/1.advanced_lighting.fs");

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let planeVertices: [f32; 48] = [
            // positions         // normals      // texcoords
             10.0, -0.5,  10.0,  0.0, 1.0, 0.0,  10.0,  0.0,
            -10.0, -0.5,  10.0,  0.0, 1.0, 0.0,   0.0,  0.0,
            -10.0, -0.5, -10.0,  0.0, 1.0, 0.0,   0.0, 10.0,

             10.0, -0.5,  10.0,  0.0, 1.0, 0.0,  10.0,  0.0,
            -10.0, -0.5, -10.0,  0.0, 1.0, 0.0,   0.0, 10.0,
             10.0, -0.5, -10.0,  0.0, 1.0, 0.0,  10.0, 10.0
        ];
        // plane VAO
        let (mut planeVAO, mut planeVBO) = (0, 0);
        gl::GenVertexArrays(1, &mut planeVAO);
        gl::GenBuffers(1, &mut planeVBO);
        gl::BindVertexArray(planeVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, planeVBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (planeVertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &planeVertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::BindVertexArray(0);

        // load textures
        // -------------
        let floorTexture = loadTexture("resources/textures/wood.png");

        // shader configuration
        // --------------------
        shader.useProgram();
        shader.setInt(c_str!("texture1"), 0);

        (shader, planeVBO, planeVAO, floorTexture)
    };

    // lighting info
    // -------------
    let lightPos = vec3(0.0, 0.0, 0.0);

    // render loop
    // -----------
    while !window.should_close() {
        // per-frame time logic
        // --------------------
        let currentFrame = glfw.get_time() as f32;
        deltaTime = currentFrame - lastFrame;
        lastFrame = currentFrame;

        // events
        // -----
        process_events(&events, &mut firstMouse, &mut lastX, &mut lastY, &mut camera);

        // input
        // -----
        processInput(&mut window, deltaTime, &mut camera, &mut blinn, &mut blinnKeyPressed);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // draw objects
            shader.useProgram();
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32 , 0.1, 100.0);
            let view = camera.GetViewMatrix();
            shader.setMat4(c_str!("projection"), &projection);
            shader.setMat4(c_str!("view"), &view);
            // set light uniforms
            shader.setVector3(c_str!("viewPos"), &camera.Position.to_vec());
            shader.setVector3(c_str!("lightPos"), &lightPos);
            shader.setInt(c_str!("blinn"), blinn as i32);
            // floor
            gl::BindVertexArray(planeVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, floorTexture);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(1, &planeVAO);
        gl::DeleteBuffers(1, &planeVBO);
    }
}

// NOTE: not the same version as in common.rs
pub fn processInput(window: &mut glfw::Window, deltaTime: f32, camera: &mut Camera, blinn: &mut bool, blinnKeyPressed: &mut bool) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }

    if window.get_key(Key::W) == Action::Press {
        camera.ProcessKeyboard(FORWARD, deltaTime);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.ProcessKeyboard(BACKWARD, deltaTime);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.ProcessKeyboard(LEFT, deltaTime);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.ProcessKeyboard(RIGHT, deltaTime);
    }

    if window.get_key(Key::B) == Action::Press && !(*blinnKeyPressed) {
        *blinn = !(*blinn);
        *blinnKeyPressed = true;
        println!("{}", if *blinn { "Blinn-Phong" } else { "Phong" })
    }
    if window.get_key(Key::B) == Action::Release {
        *blinnKeyPressed = false;
    }
}

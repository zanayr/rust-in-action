use graphics::math::{Vec2d, add, mul_scalar};
use piston_window::*;
use rand::prelude::*;
use std::alloc::{GlobalAlloc, System, Layout};
use std::time::Instant;

#[global_allocator]                                                                     // `#[global_allocator]` marks the
static ALLOCATOR: ReportingAllocator = ReportingAllocator;                              // following value (ALLOCATOR) as
                                                                                        // satisfying the GlobalAlloc trait
struct ReportingAllocator;                                                              // prints the time taked for each
                                                                                        // allocations to STDOUT as the
unsafe impl GlobalAlloc for ReportingAllocator {                                        // program runs [...]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);                                                 // defers the actuall memory
        let end = Instant::now();                                                       // allocation to the system's
        let time_taken = end - start;                                                   // default memory allocator
        let bytes_requested = layout.size();

        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

struct World {                                                                          // contains the data
    current_turn: u64,                                                                  // that is useful for
    particles: Vec<Box<Particle>>,                                                      // the lifetime of the
    height: f64,                                                                        // program
    width: f64,
    rng: ThreadRng,
}

struct Particle {                                                                       // defines an object in
    height: f64,                                                                        // 2D space
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Particle {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..=world.width);                                       // starts a random position
        let y = world.height;                                                           // along the bottom of the window
        let x_velocity = 0.0;                                                           // rises vertically over time
        let y_velocity = rng.gen_range(-2.0..0.0);
        let x_acceleration = 0.0;                                                       // increases the speed
        let y_acceleration = rng.gen_range(0.0..0.15);                                  // of the rise over time

        Particle {
            height: 4.0,
            width: 4.0,
            position: [x, y].into(),                                                    // `.into()` converts the
            velocity: [x_velocity, y_velocity].into(),                                  // arrays of type [f64; 2]
            acceleration: [x_acceleration, y_acceleration].into(),                      // into `Vec2d`
            color: [1.0, 1.0, 1.0, 0.99],
        }
    }

    fn update(&mut self) {
        self.velocity = add(self.velocity, self.acceleration);                          // move the particle
        self.position = add(self.position, self.velocity);                              // to its next position
        self.acceleration = mul_scalar(                                                 // slow down the particle's
            self.acceleration,                                                          // rate or increase as it travels
            0.7                                                                         // across the screen
        );
        self.color[3] *= 0.995;                                                         // makes the particle more
    }                                                                                   // transparent over time
}

impl World {
    fn new(width: f64, height: f64) -> World {
        World {
            current_turn: 0,
            particles: Vec::<Box<Particle>>::new(),                                     // uses `Box<Particle>` rather
            height: height,                                                             // than `Particle` to incur an extra
            width: width,                                                               // memory allocations when
            rng: thread_rng(),                                                          // every particle is created
        }
    }
    
    fn add_particles(&mut self, n: i32) {                                               // creates a new `Particle` as a
        for _ in 0..n.abs() {                                                           // local variable on the stack
            let particle = Particle::new(&self);                                        // takes ownership of `particle`,
            let boxed_particle = Box::new(particle);                                    // moving its data to the heap,
            self.particles.push(boxed_particle);                                        // and creates a reference to
        }                                                                               // that data on the stack
    }                                                                                   // pushes the reference
                                                                                        // into `self.particles`
    fn remove_particles(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut to_delete = None;

            let particle_iter = self.particles                                          // `particle_iter` is split into
                .iter()                                                                 // its own variable to more
                .enumerate();                                                           // easily fit on the (book) page
            
            for (i, particle) in particle_iter {                                        // for n iterations, removes
                if particle.color[3] < 0.2 {                                            // the first `particle` that's
                    to_delete = Some(i);                                                // invisible
                }
                break;
            }

            if let Some(i) = to_delete {                                                // if there are no invisible
                self.particles.remove(i);                                               // particles, then removes
            } else {                                                                    // the oldest
                self.particles.remove(0);
            }
        }
    }

    fn update(&mut self) {                                                              // return a random
        let n = self.rng.gen_range(-3..=3);                                             // integer between -3
                                                                                        // and 3, inclusive
        if n > 0 {
            self.add_particles(n);
        } else {
            self.remove_particles(n);
        }

        self.particles.shrink_to_fit();
        for particle in &mut self.particles {
            particle.update();
        }
        self.current_turn += 1;
    }
}

fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new(
        "particles", [width, height]
    )
    .exit_on_esc(true)
    .build()
    .expect("Could not create a window.");

    let mut world = World::new(width, height);
    world.add_particles(1000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for p in &mut world.particles {
                let size = [p.position[0], p.position[1], p.width, p.height];
                rectangle(p.color, size, ctx.transform, renderer);
            }
        });
    }
}

use std::env;

use photon::arena;
use photon::scene;

#[cfg(feature = "progress")]
use photon::progress;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<_>>();
    let arena = arena::Arena::default();
    let scene = scene::Scene::load(&arena, &args[1])?;

    #[cfg(feature = "progress")]
    let progress = std::thread::spawn({
        let pixels = scene.width() * scene.height();
        move || progress::run(pixels)
    });

    scene.render();

    #[cfg(feature = "stats")] {
        println!("{}", photon::stats::ARENA_MEMORY);
        println!("{}", photon::stats::INTERSECTION_TESTS);
        println!("{}", photon::stats::BOUNDING_BOX_INTERSECTION_TESTS);
        println!("{}", photon::stats::BVH_HITS);
        println!("{}", photon::stats::BVH_MISSES);
        println!("{}", photon::stats::SPHERE_INTERSECTION_TESTS);
        println!("{}", photon::stats::TRI_INTERSECTION_TESTS);
        println!("{}", photon::stats::LIST_INTERSECTION_TESTS);
    }

    #[cfg(feature = "progress")]
    progress.join().unwrap().ok();
    Ok(())
}

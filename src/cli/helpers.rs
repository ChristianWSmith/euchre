use std::{mem, thread};

use crate::organism::evolution::{evolve, Organism};

pub fn evolve_cli(population_size: usize, generations: usize, out_dir: String) {
    // max supported population size + 31, don't ask why
    let stack_size: usize = mem::size_of::<Organism>() * (2048 + 31);

    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(move || -> std::io::Result<()> {
            match population_size {
                2048 => evolve::<2048, 1024>(generations, out_dir.clone()).unwrap(),
                1024 => evolve::<1024, 512>(generations, out_dir.clone()).unwrap(),
                512 => evolve::<512, 256>(generations, out_dir.clone()).unwrap(),
                256 => evolve::<256, 128>(generations, out_dir.clone()).unwrap(),
                128 => evolve::<128, 64>(generations, out_dir.clone()).unwrap(),
                64 => evolve::<64, 32>(generations, out_dir.clone()).unwrap(),
                32 => evolve::<32, 16>(generations, out_dir.clone()).unwrap(),
                16 => evolve::<16, 8>(generations, out_dir.clone()).unwrap(),
                8 => evolve::<8, 4>(generations, out_dir.clone()).unwrap(),
                4 => evolve::<4, 2>(generations, out_dir.clone()).unwrap(),
                _ => panic!("Invalid population size.  Valid populations sizes: [2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4]")
            };
            Ok(())
        })
        .unwrap();

    handle.join().unwrap().ok();
}

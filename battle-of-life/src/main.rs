use battle_of_life::{Cell, Universe};

fn main() {
    let mut universe = Universe::new(5, 5, |_| Cell::Dead);
    universe.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    println!("{}", universe);
    universe.tick();
    println!("Changed len: {}", universe.changed_len());
    println!("Changed idx: {:?}", universe.get_changed());
    println!("{}", universe);
    universe.tick();
    println!("Changed len: {}", universe.changed_len());
    println!("Changed idx: {:?}", universe.get_changed());
    println!("{}", universe);
    //
    // let mut universe = Universe::new(64, 64, |i| {
    //     if i % 2 == 0 || i % 7 == 0 {
    //         Cell::Alive
    //     } else {
    //         Cell::Dead
    //     }
    // });
    // println!("{}", universe);
    // universe.tick();
    // println!("Changed len: {}", universe.changed_len());
    // println!("Changed idx: {:?}", universe.get_changed());
    // println!("{}", universe);
    // universe.tick();
    // println!("Changed len: {}", universe.changed_len());
    // println!("Changed idx: {:?}", universe.get_changed());
    // println!("{}", universe);
}

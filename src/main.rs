#![windows_subsystem = "windows"]
use tray_item::{TrayItem, IconSource};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use enigo::{Enigo, Mouse, Settings, Coordinate};

fn jiggle_mouse(is_running: Arc<Mutex<bool>>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    loop {
        if !*is_running.lock().unwrap() {
            break;
        }
        // Get the current mouse position
        let pos = enigo.location().unwrap();
        enigo.move_mouse(pos.0 - 5, pos.1 -5, Coordinate::Abs).unwrap();
        thread::sleep(Duration::from_millis(300));

        let new_pos = enigo.location().unwrap();
        // Check the see if the user is moving the cursor by checking to see if the position after being moved is +/- 6 pixels from the original position
        if ((new_pos.0 - 6)..=(new_pos.0 + 6)).contains(&pos.0) && ((new_pos.1 - 6)..=(new_pos.1 + 6)).contains(&pos.1) {
            enigo.move_mouse(pos.0+1,pos.1+1, Coordinate::Abs).unwrap();
        }
        
        // Wait for 2 seconds before repeating
        thread::sleep(Duration::from_secs(2));
    }
}
fn main() {
    // Create a new TrayItem with a title and an icon
    let mut tray = TrayItem::new("Riggle", IconSource::Resource("tray-default")).unwrap();

    let is_jiggling = Arc::new(Mutex::new(false));

    // Add "Jiggle" menu Item
    let is_jiggling_clone = Arc::clone(&is_jiggling);
    tray.add_menu_item("Jiggle", move || {
        let mut running = is_jiggling_clone.lock().unwrap();
        if !*running {
            *running = true;
            let is_jiggling_clone = Arc::clone(&is_jiggling_clone);
            thread::spawn(move || jiggle_mouse(is_jiggling_clone));
        }
    }).unwrap();

    // Add "No Jiggle" menu Item
    let is_jiggling_clone = Arc::clone(&is_jiggling);
    tray.add_menu_item("No Jiggle", move || {
        let mut running = is_jiggling_clone.lock().unwrap();
        *running = false;
    }).unwrap();

    // Add "Quit" menu item
    tray.add_menu_item("Quit", move || {
        std::process::exit(0);
    }).unwrap();

    // Keep the main thread alive
    loop {
        std::thread::park();
    }
}
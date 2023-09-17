// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use closest_pair::Point;
use rand::Rng;
use lazy_static::lazy_static;
use std::sync::Mutex;
extern crate closest_pair;

// Define global storage for dots
lazy_static! {
    static ref DOTS: Mutex<Vec<(f64, f64)>> = Mutex::new(Vec::new());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_dots, get_saved_dots, get_closest_pair, read_csv_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Function to generate 100 dots and save them in the global variable
#[tauri::command]
async fn get_dots(n :i32) -> Result<Vec<(f64, f64)>, String> {
    let mut rng = rand::thread_rng();
    let new_dots: Vec<_> = (0..n).map(|_| {
        (rng.gen::<f64>() * 100.0, rng.gen::<f64>() * 100.0)
    }).collect();

    let mut global_dots = DOTS.lock().unwrap();
    *global_dots = new_dots.clone();
    
    Ok(new_dots)
}

// Helper function to retrieve the saved dots from the global variable
#[tauri::command]
async fn get_saved_dots() -> Result<Vec<(f64, f64)>, String> {
    let global_dots = DOTS.lock().unwrap();
    Ok(global_dots.clone())
}


// Utility function to map vec<f64, f64> to vec<point>
fn map_to_points(dots: Vec<(f64, f64)>) -> Vec<closest_pair::Point> {
    let mut points = Vec::new();
    for dot in dots {
        points.push(Point{x: dot.0 ,y: dot.1});
    }
    points
}

// Tauri function to return the closest pair of dots
#[tauri::command]
async fn get_closest_pair() -> Result<(f64, f64, f64, f64, f64), String> {
    let global_dots = DOTS.lock().unwrap();
    let (p1, p2, dist) = closest_pair::closest_pair(&map_to_points(global_dots.clone()));
    Ok((p1.x, p1.y, p2.x, p2.y, dist))
}

// Utility function that read a CSV file from a given path and returns a Vec<f64, f64>
fn read_csv(path: &str) -> Vec<(f64, f64)> {
    let mut reader = csv::Reader::from_path(path).unwrap();
    let mut dots = Vec::new();
    for result in reader.records() {
        let record = result.unwrap();
        dots.push((record[0].parse::<f64>().unwrap(), record[1].parse::<f64>().unwrap()));
    }
    dots
}

// Tauri function to read a CSV file from a given path and save it in the global variable and return the dots
#[tauri::command]
async fn read_csv_file(path: String) -> Result<Vec<(f64, f64)>, String> {
    let dots = read_csv(&path);
    let mut global_dots = DOTS.lock().unwrap();
    *global_dots = dots.clone();
    Ok(dots)
}
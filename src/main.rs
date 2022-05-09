#![forbid(unsafe_code)]
#[macro_use]
extern crate rocket;

pub mod database;
pub mod equipment;
mod exercises;
pub mod handlers;
pub mod muscles;
#[cfg(test)]
mod tests;
pub mod util;

use database::build_tables;
use equipment::WeightType;
use exercises::{ExerciseEntry, SetEntry, WorkoutEntry, EXERCISE_BENCH_PRESS};
use rocket_db_pools::Database;
use sqlx::{ConnectOptions, SqliteConnection};
use uuid::Uuid;

#[allow(unused_imports)]
use crate::{database::create_connection, equipment::Weight};

// Global Preference for weight, implement configuration later
const GLOBAL_WEIGHT_UNIT: WeightType = equipment::POUNDS;

#[rocket::main]
async fn main() {
    // connect to DB
    let conn =
        <sqlx::sqlite::SqliteConnectOptions as std::str::FromStr>::from_str("sqlite://data.db")
            .unwrap()
            .create_if_missing(true)
            .connect()
            .await
            .unwrap();

    //database::build_tables(conn).await;
    dev(conn).await;
    let rocket = launch_web().await;
}

async fn launch_web() -> Result<(), rocket::Error> {
    // launch web server
    let shield = rocket::shield::Shield::default()
        .enable(rocket::shield::Referrer::NoReferrer)
        .enable(rocket::shield::XssFilter::EnableBlock);
    let rocket = rocket::build()
        .attach(shield)
        .attach(database::Db::init())
        .attach(rocket_dyn_templates::Template::fairing())
        .mount(
            "/",
            routes![
                crate::handlers::workout_json,
                crate::handlers::view,
                crate::handlers::static_file
            ],
        );
    rocket.launch().await
}

// Stuff not needed for prod, but useful for testing
async fn dev(conn: SqliteConnection) {
    let bench_press = SetEntry {
        exercise: *EXERCISE_BENCH_PRESS,
        reps: 8,
        weight: Weight {
            weight: 135.0,
            weight_unit: "lbs".to_string(),
        },
        reps_in_reserve: 1.5,
    };
    let mut bench_set = ExerciseEntry {
        exercise: *EXERCISE_BENCH_PRESS,
        comments: String::from(""),
        sets: vec![bench_press.clone()],
    };
    for i in 0..2 {
        bench_set.sets.push(bench_press.clone());
    }
    let uuid = Uuid::new_v4();
    let bench_workout = WorkoutEntry {
        uuid: uuid.to_string(),
        start_time: std::time::UNIX_EPOCH.elapsed().unwrap().as_secs(),
        end_time: std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() + 3600,
        exercises: vec![bench_set.clone()],
        comments: "".to_string(),
        user: "John Doe".to_string(),
    };
    //build_tables(conn).await;
    //database::insert_workout(uuid, bench_workout, conn).await;
    //http://localhost:8000/workout/f6af9f72-f10c-427d-b814-eab720b84cd9/json
}

#[macro_use]
extern crate rocket;
use std::{fs::create_dir, path::PathBuf};

use rocket::{fs::NamedFile, log, State};

#[get("/")]
fn index() -> &'static str {
    "welcome to rit!"
}

#[get("/<owner>/<repo>/<path..>")]
async fn git(config: &State<Config>, owner: &str, repo: &str, path: PathBuf) -> Option<NamedFile> {
    let requested_path = config
        .repository
        .join(owner)
        .join(repo)
        .join(".git")
        .join(path);

    log::debug_!("requesting repo: {}", requested_path.to_string_lossy());

    NamedFile::open(requested_path).await.ok()
}

struct Config {
    pub repository: PathBuf,
}

#[launch]
fn rocket() -> _ {
    let repository_path: PathBuf = std::env::var("RIT_REPO_PATH")
        .unwrap_or("/tmp/repos".to_string())
        .into();

    if !repository_path.exists() {
        create_dir(&repository_path).expect("Unable to create repository directory");
    }

    rocket::build()
        .manage(Config {
            repository: repository_path.clone(),
        })
        .mount("/", routes![index, git])
}

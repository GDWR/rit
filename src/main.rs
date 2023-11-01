#[macro_use]
extern crate rocket;
use std::{fs::create_dir, path::PathBuf, vec};

use rit::Repositories;
use rocket::{fs::NamedFile, log, State};
use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Namespace {
    name: String,
    repositories: Vec<String>,
}

#[derive(Serialize)]
struct IndexContext {
    namespaces: Vec<Namespace>,
}

#[get("/")]
fn index(repositories: &State<Repositories>) -> Result<Template, std::io::Error> {
    Ok(Template::render(
        "index",
        IndexContext {
            namespaces: repositories
                .all_namespaces()?
                .iter()
                .map(|name| Namespace {
                    name: name.to_string(),
                    repositories: repositories.all_projects(name).unwrap_or(Vec::new()),
                })
                .collect(),
        },
    ))
}

#[get("/<namespace>/<repo>/<path..>")]
async fn git(
    config: &State<Config>,
    namespace: &str,
    repo: &str,
    path: PathBuf,
) -> Option<NamedFile> {
    let requested_path = config
        .repository
        .join(namespace)
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
        .manage(Repositories::new(repository_path.clone()))
        .mount("/", routes![index, git])
        .attach(Template::fairing())
}

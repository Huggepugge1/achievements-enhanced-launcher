use reqwest;

async fn get_update(current: Option<String>) -> Result<Option<String>, reqwest::Error> {
    let res = reqwest::get("https://github.com/Huggepugge1/achievements-enhanced/releases/latest")
        .await?;

    let newest_version = res.url().path().split("/").last().unwrap();

    match current {
        Some(current) => {
            if newest_version != current {
                return Ok(Some(newest_version.to_string()));
            }
        }
        None => {
            return Ok(Some(newest_version.to_string()));
        }
    }

    Ok(None)
}

async fn update_achievements(old: Option<String>, new: String) {
    match old.clone() {
        Some(old) => {
            println!("New version available: {:?} -> {}", old, new);
            println!("Do you want to update? (Y/n)");
        }
        None => {
            println!("Version {} available", new);
            println!("Do you want to install it? (Y/n)");
        }
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "n" {
        return;
    }
    println!("Downloading new version...");
    let res = if cfg!(target_os = "linux") {
        reqwest::get("https://github.com/Huggepugge1/achievements-enhanced/releases/latest/download/achievements-enhanced")
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap()
    } else {
        reqwest::get("https://github.com/Huggepugge1/achievements-enhanced/releases/latest/download/achievements-enhanced.exe")
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap()
    };

    println!("Writing new version...");
    let new = format!("./achievements-enhanced-{}", new);
    let _ = std::fs::create_dir(new.clone());

    if cfg!(target_os = "linux") {
        std::fs::write(new.clone() + "/achievements-enhanced", res).unwrap();
        println!("Marking new version as executable...");
        let _ = std::process::Command::new("chmod")
            .args(&["+x", &(new.clone() + "/achievements-enhanced")])
            .spawn()
            .expect("Failed to start the program")
            .wait();
    } else {
        std::fs::write(new.clone() + "/achievements-enhanced.exe", res).unwrap();
    }
    match old {
        Some(old) => {
            println!("Updated from {} to {}", old, new);
            let _ = std::fs::remove_dir(format!("./achievements-enhanced-{}", old));
        }
        None => {
            println!("Installed version {}", new);
        }
    }
}

fn get_current() -> Option<String> {
    match std::fs::read_dir(".")
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .filter(|entry| {
            entry
                .path()
                .to_str()
                .unwrap()
                .to_string()
                .starts_with("./achievements-enhanced")
        })
        .max_by_key(|entry| entry.path())
    {
        Some(entry) => Some(
            entry
                .path()
                .to_str()
                .unwrap()
                .to_string()
                .split("-")
                .last()
                .unwrap()
                .to_string(),
        ),
        None => None,
    }
}

#[tokio::main]
async fn main() {
    let current = get_current();
    let update = get_update(current.clone()).await.unwrap();
    match current.clone() {
        Some(current) => {
            println!("Current version: {}", current);
        }
        None => {
            println!("No version found");
        }
    }
    if let Some(update) = update {
        update_achievements(current, update).await;
    }
    let mut current = match get_current() {
        Some(current) => current,
        None => {
            panic!("No version found");
        }
    };
    println!("Starting achievements enhanced...");
    current = format!("./achievements-enhanced-{}", current);
    if cfg!(target_os = "linux") {
        current = current + "/achievements-enhanced";
    } else {
        current = current + "/achievements-enhanced.exe";
    }
    let _ = std::process::Command::new(current)
        .spawn()
        .expect("Failed to start the program")
        .wait();
}

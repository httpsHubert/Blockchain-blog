use std::cell::RefCell;
use crate::blog::Blog;
use crate::config::Config;

mod blog;
mod config;

thread_local! {
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
    static CONFIG: RefCell<Config> = RefCell::new(Config::new());
}

#[ic_cdk::update]
fn add_config (new_config: Config) {
    CONFIG.with(|config| *config.borrow_mut() = new_config);
}

#[ic_cdk::update]
fn add_blog(title: String, content: String, tags: Vec<String>) -> Result<String, String> {
    let config = CONFIG.with(|config| config.borrow().clone());

    if title.len() > config.max_title_length as usize {
        return Err("Title is too long!".to_string())
    }
    if content.len() > config.max_content_length as usize {
        return Err("Content is too long!".to_string())
    }
    if tags.len() > config.max_tags_count as usize {
        return Err("You can add a maximum of 3 tags.".to_string());
    }

    let are_tags_in_config = tags.iter().any(|tag| !config.tags.contains(tag));

    if are_tags_in_config {
        return Err("You can only use tags from the config.".to_string());
    }

    let blog = Blog::new(title, content, tags);
    BLOGS.with(|blogs| blogs.borrow_mut().push(blog));

    Ok("Added new blog".to_string())
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog> {
    BLOGS.with(|blogs| blogs.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

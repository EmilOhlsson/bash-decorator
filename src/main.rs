extern crate git2;

use git2::Repository;

fn git_decoration() -> Result<String, git2::Error> {
    let repo = Repository::open(".")?;
    let head = repo.head()?;

    Ok(String::from(head.name().unwrap_or("unknown")))
}

fn main() {
    println!("{}", git_decoration().unwrap());
}

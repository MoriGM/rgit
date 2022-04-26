pub mod repo;

fn main() {
    let repo = match repo::GitRepo::new("/home/max/Code/rust/book/hello_world/") {
        Ok(repo) => repo,
        Err(err) => panic!("{}", err)
    };
    
    repo.logs().for_each(|rev| {
        println!("{}", rev);
    });
}

use git2::Repository;

fn main() {
    let repo = match Repository::init("/home/max/Code/rust/book/hello_world/") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
    
    let mut revs = match repo.revwalk() {
        Ok(rev) => rev,
        Err(e) => panic!("failed to init: {}", e)
    };
    
    revs.push_head().unwrap();
    
    revs.for_each(|rev| {
        print!("{:?}", rev.ok());
    });
}

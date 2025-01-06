fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = std::env::args_os()
        .nth(1)
        .ok_or("need repo path argument")?;
    let repo = gix::discover(&repo_path)?;
    println!("Found git dir: {:?}", repo.git_dir());
    let work_dir = repo.work_dir().ok_or("need non-bare repo")?;
    println!("Using index path: {:?}", repo.index_path());
    println!("Found work dir: {work_dir:?}");

    let mut index = repo.open_index()?;

    let outcome = gix::worktree::state::checkout(
        &mut index,
        work_dir,
        repo.objects.clone(),
        &gix::progress::Discard,
        &gix::progress::Discard,
        &gix::interrupt::IS_INTERRUPTED,
        Default::default(), // FIXME: Specify options.
    )?;

    println!("{:?}", outcome);

    Ok(())
}

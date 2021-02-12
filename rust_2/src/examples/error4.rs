fn write_info_old(info: &Info) -> io::Result<()> {
    // Early return on error
    let mut file = match File::create("file.txt") {
           Err(e) => return Err(e),
           Ok(f) => f,
    };

    // Further work with the valid file
}

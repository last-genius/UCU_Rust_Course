fn write_info_new(info: &Info) -> io::Result<()> {
    // Early return on error
    let mut file = File::create("file.txt")?;
    
    // Further work with the valid file
}

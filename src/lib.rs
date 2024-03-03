use std::{fs, io, path::PathBuf};

pub async fn protokoll_i_månad(år: u32, månad: u32) -> io::Result<Vec<(String, String)>> {
    let path = PathBuf::from(format!("assets/protokoll/{:04}/{:02}/", år, månad));

    let read_dir = fs::read_dir(path)?;

    let mut filnamn = Vec::new();

    for dir_entry in read_dir {
        let dir_entry = dir_entry?;
        let nuvarande_filnamn = dir_entry.file_name();
        let nuvarande_filnamn = match nuvarande_filnamn.to_str() {
            Some(value) => value,
            None => {
                log::error!("Kunde inte få en &str från OsString");
                continue;
            }
        };
        let utan_filtyp = match nuvarande_filnamn.split('.').next() {
            Some(value) => value,
            None => {
                log::warn!("Ingen filtyp på {nuvarande_filnamn}");
                continue;
            }
        };
        let mut delar_i_filnamn = utan_filtyp.split(' ');
        let år = match delar_i_filnamn.next() {
            Some(value) => value,
            None => continue,
        };
        let månad = match delar_i_filnamn.next() {
            Some(value) => value,
            None => continue,
        };
        let dag = match delar_i_filnamn.next() {
            Some(value) => value,
            None => continue,
        };

        filnamn.push((
            format!("{år} {månad} {dag}"),
            format!("{år}/{månad}/{år} {månad} {dag}"),
        ))
    }

    Ok(filnamn)
}

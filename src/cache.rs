
use std::{fs::File, io::{BufReader, BufWriter}, collections::{HashMap, HashSet}, error::Error};

lazy_static! {
    static ref CACHE_DIR: String = {
        let mut path = dirs::cache_dir().unwrap();
        path.push("sub-solver");
        path.to_str().unwrap().to_string()
    };
}

fn get_filename(content: &str) -> String {
    format!("{}/{:x}.bin", *CACHE_DIR, md5::compute(content))
}

pub fn load_cached_dictionary(content: &str) -> Option<HashMap<String, HashSet<String>>> {
    if let Ok(file) = File::open(get_filename(content)) {
        let reader = BufReader::new(file);
        bincode::deserialize_from(reader).ok()
    } else {
        None
    }
}

pub fn save_cached_dictionary(content: &str, dictionary: &HashMap<String, HashSet<String>>) -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(&*CACHE_DIR)?;  // Create folder if doesn't exist
    let mut file = BufWriter::new(File::create(get_filename(content))?);
    bincode::serialize_into(&mut file, dictionary)?;
    Ok(())
}

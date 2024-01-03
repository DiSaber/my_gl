use std::{ffi::CString, io::BufReader};

use tobj::{LoadError, Model};

pub fn cstring_with_length(length: usize) -> CString {
    unsafe { CString::from_vec_unchecked(Vec::from_iter(std::iter::repeat(b' ').take(length))) }
}

pub fn tobj_from_slice_no_mtl(obj_slice: &[u8]) -> Result<Vec<Model>, LoadError> {
    Ok(tobj::load_obj_buf(
        &mut BufReader::new(obj_slice),
        &tobj::GPU_LOAD_OPTIONS,
        |_| tobj::load_mtl_buf(&mut BufReader::new(&[] as &[u8])),
    )?
    .0)
}

// With mtl from resources
/*|path| {
    tobj::load_mtl_buf(&mut if let Some(mtl) =
        resources.get(path.file_name().unwrap().to_str().unwrap())
    {
        BufReader::new(mtl.as_slice())
    } else {
        BufReader::new(&[] as &[u8])
    })
}*/

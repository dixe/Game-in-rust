use freetype::Library;


pub struct FreeTypeWrapper {
    pub lib: freetype::Library,
    pub face: freetype::face::Face
}



pub fn load_free_type() -> FreeTypeWrapper {

    let lib = Library::init().unwrap();

    let path = "C:/Windows/Fonts/arial.ttf";
    let face = lib.new_face(path, 0).unwrap();

    face.set_char_size(40*64, 0, 50, 0).unwrap();


    FreeTypeWrapper {
        lib,
        face
    }
}

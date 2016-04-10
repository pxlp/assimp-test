extern crate assimp;
extern crate libz_sys;

#[test]
fn it_works() {
    let mut importer = assimp::Importer::new();
    let scene = importer.read_file("Barrel1.x").unwrap();
    assert_eq!(scene.num_meshes(), 1);
}

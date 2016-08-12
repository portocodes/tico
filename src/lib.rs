fn tico(tico: &str) -> String {
    let mut v: Vec<&str> = tico.split('/').collect();
    v.reverse();
    let lastDir = v.first();

    v.reverse();
    v.iter().map(|&a| a.chars().take(1)).collect();
}

#[test]
fn it_works() {
    assert_eq!(tico("~"), "~");
    assert_eq!(tico("/"), "/");
    assert_eq!(tico("/home/hugopeixoto/work/personal/tico"), "/h/h/w/p/tico");
    assert_eq!(tico("~/work/personal/tico"), "~/w/p/tico");
    assert_eq!(tico("~/work/personal/tico/"), "~/w/p/t/");
    assert_eq!(tico("~/work/ééé/tico"), "~/w/é/tico");
}

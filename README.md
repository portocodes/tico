The best README is probably the [test cases](https://github.com/portocodes/tico/blob/master/src/lib.rs#L30-L38):

```rust
#[test]
fn it_works() {
    assert_eq!(tico("~"), "~");
    assert_eq!(tico("/"), "/");
    assert_eq!(tico("/home/hugopeixoto/work/personal/tico"), "/h/h/w/p/tico");
    assert_eq!(tico("~/work/personal/tico"), "~/w/p/tico");
    assert_eq!(tico("~/work/personal/tico/"), "~/w/p/t/");
    assert_eq!(tico("~/work/ééé/tico"), "~/w/é/tico");
}
```

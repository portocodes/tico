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

## Install

`$ cargo install --git git@github.com:portocodes/tico.git`

## Use

Here's how I'm using it in `fish_prompt.fish` to replace `prompt_pwd`:

`set -l cwd $cyan(tico (echo $PWD | sed -e "s|^$HOME|~|"))`

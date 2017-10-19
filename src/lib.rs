/// Shorten a path.
///
/// This function keeps only the first letter of each
/// level (`/` separated) of the path, except the current one.
///
/// # Examples
///
/// ```
/// let shortened = tico::tico("/home/.secret/path");
/// println!("{}", shortened);
/// // => "/h/.s/path"
/// ```
pub fn tico(tico: &str) -> String {
    let mut shortened = String::from("");
    let mut skip_char = false;
    let mut count = 0;
    let sections = tico.chars().filter(|&x| x == '/').count();

    for c in tico.chars() {
        match c {
            '~' => {
                if !skip_char {
                    shortened.push(c)
                }
            }
            '.' => {
                skip_char = false;
                shortened.push(c);
            }
            '/' => {
                skip_char = false;
                count += 1;
                shortened.push(c)
            }
            _ => {
                if skip_char && count < sections {
                    continue;
                } else {
                    skip_char = true;
                    shortened.push(c);
                }
            }
        }
    }

    shortened
}

#[test]
fn it_works() {
    assert_eq!(tico("~"), "~");
    assert_eq!(tico("/"), "/");
    assert_eq!(
        tico("/home/hugopeixoto/work/personal/tico"),
        "/h/h/w/p/tico"
    );
    assert_eq!(tico("~/work/personal/tico"), "~/w/p/tico");
    assert_eq!(tico("~/work/personal/tico/"), "~/w/p/t/");
    assert_eq!(tico("~/work/ééé/tico"), "~/w/é/tico");
    assert_eq!(tico("~/.config/htop"), "~/.c/htop");
}

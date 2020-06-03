extern crate dirs;

/// Shorten a path.
///
/// This function keeps only the first letter of each
/// level (`/` separated) of the path, except the current one.
///
/// # Examples
///
/// ```
/// let shortened = tico::tico("/home/.secret/path", Option::None);
/// println!("{}", shortened);
/// // => "/h/.s/path"
/// ```
pub fn tico(path: &str, home_dir: Option<&str>) -> String {
    let tico = match home_dir {
        Some(dir) => path.replacen(&dir, "~", 1),
        None => path.to_owned(),
    };

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
    assert_eq!(tico("~", Option::None), "~");
    assert_eq!(tico("/", Option::None), "/");
    assert_eq!(
        tico("/home/hugopeixoto/work/personal/tico", Option::None),
        "/h/h/w/p/tico"
    );
    assert_eq!(tico("~/work/personal/tico", Option::None), "~/w/p/tico");
    assert_eq!(tico("~/work/personal/tico/", Option::None), "~/w/p/t/");
    assert_eq!(tico("~/work/ééé/tico", Option::None), "~/w/é/tico");
    assert_eq!(tico("~/.config/htop", Option::None), "~/.c/htop");
}

#[test]
fn home_dir_shorthand() {
    let home_dir = Option::Some("/home/hugopeixoto");

    assert_eq!(tico("~", home_dir), "~");
    assert_eq!(tico("/", home_dir), "/");
    assert_eq!(
        tico("/home/hugopeixoto/work/personal/tico", home_dir),
        "~/w/p/tico"
    );
    assert_eq!(tico("~/work/personal/tico", home_dir), "~/w/p/tico");
    assert_eq!(tico("~/work/personal/tico/", home_dir), "~/w/p/t/");
    assert_eq!(tico("~/work/ééé/tico", home_dir), "~/w/é/tico");
    assert_eq!(tico("~/.config/htop", home_dir), "~/.c/htop");
}

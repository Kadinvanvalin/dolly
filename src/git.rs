use regex::Regex;
use std::env;
use std::path::PathBuf;

#[derive(PartialEq, Debug)]
pub struct GitRepo {
    pub host: String,
    pub slug: String,
    pub repo_name: String,
}
pub fn valid_ssh_url(url: &str) -> bool {
    let matches = Regex::new(r"(git)@([^/:]+):([^/:]+)/(.+)(.git)");

    return match matches {
        Ok(content) => content.is_match(url),
        Err(_) => false,
    };
}

pub fn write_location(url: &str) -> PathBuf {
    return parse_write_location(parse_url(url));
}
pub fn make_url(url: &str) -> String {
    return make_url_private(parse_url(url));
}

pub fn parse_url(url: &str) -> GitRepo {
    let re = Regex::new(r"(git)@([^/:]+):([^/:]+)/(.+)(.git)").expect("failed to parse regex");

    let caps = re.captures(&url).unwrap();
    let host = caps.get(2).map_or("", |m| m.as_str());
    let slug = caps.get(3).map_or("", |m| m.as_str());
    let repo_name = caps.get(4).map_or("", |m| m.as_str());
    return GitRepo {
        host: host.parse().unwrap(),
        slug: slug.parse().unwrap(),
        repo_name: repo_name.parse().unwrap(),
    };
}

fn make_url_private(git_repo: GitRepo) -> String {
    String::from(&format!(
        "https://{}/{}/{}",
        git_repo.host, git_repo.slug, git_repo.repo_name
    ))
}

fn parse_write_location(git_repo: GitRepo) -> PathBuf {
    return PathBuf::from(&format!(
        "{}/{}/{}/{}",
        env::var("HOME").unwrap(),
        git_repo.host,
        git_repo.slug,
        git_repo.repo_name
    ));
}


#[test]
fn check_valid_ssh_url() {
    let url = "git@github.com:Kadinvanvalin/dolly.git";
    assert_eq!(valid_ssh_url(url), true);
}

#[test]
fn check_invalid_ssh_url() {
    let url = "some-other-string";
    assert_eq!(valid_ssh_url(url), false);
}

#[test]
fn check_string_is_converted_to_struct() {
    let expected = GitRepo {
        host: "github.com".parse().unwrap(),
        slug: "Kadinvanvalin".parse().unwrap(),
        repo_name: "dolly".parse().unwrap(),
    };
    let url = "git@github.com:Kadinvanvalin/dolly.git";
    assert_eq!(parse_url(url), expected);
}
// todo:: this looks like it should work
// #[test]
// fn check_write_location() {
//     let git_repo = GitRepo {
//         host: "github.com".parse().unwrap(),
//         slug: "kadinvanvalin".parse().unwrap(),
//         repo_name: "dolly".parse().unwrap(),
//     };
//     let expected = PathBuf::from(&format!(
//         "{}/github.com/kadinvanalin/dolly",
//         env::var("HOME").unwrap()
//     ));
//     let actual = parse_write_location(git_repo);
//     assert_eq!(actual, expected);
// }

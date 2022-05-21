git_remote=$(git remote get-url origin)
url=$(cargo run -- $git_remote o)
open $url
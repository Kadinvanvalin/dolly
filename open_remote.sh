git_remote=$(git remote get-url origin)
url=$(dolly $git_remote o) || $git_remote

open $url
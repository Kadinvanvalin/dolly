into=$(dolly $1)

git clone "$1" "$into" && cd "$into"


# alias clone_and_go=". dolly_INNER_clone_and_go.sh"

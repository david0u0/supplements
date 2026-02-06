#compdef qit

_qit() {
    local -a candidates
    local cur=${words[-1]}

    if [[ -z "$cur" ]]; then
        candidates=("${(@f)$( echo zsh ${words[1,CURRENT]} "''" | xargs PLACEHOLDER_FOR_BIN_PATH )}")
    else
        candidates=("${(@f)$( echo zsh ${words[1,CURRENT]} | xargs PLACEHOLDER_FOR_BIN_PATH )}")
    fi

    local -a values descs
    for line in $candidates; do
        parts=(${(@ps:\t:)line})
        values+=("${parts[1]}")
        descs+=("${parts[1]} ${parts[2]}")
    done

    compadd -d descs -a values
}

_qit "$@"

# Black: \u001b[30m
# Red: \u001b[31m
# Green: \u001b[32m
# Yellow: \u001b[33m
# Blue: \u001b[34m
# Magenta: \u001b[35m
# Cyan: \u001b[36m
# White: \u001b[37m
# Reset: \u001b[0m

function reset-style () {
    print -n $(styled reset)
}

function set-style () {
    print -n $(style-open $@)
}

function styled () {
    if [[ $1 == "-n" ]]; then
        print -n $(style-open $@ reset)
    else
        print $(style-open $@ reset)
    fi
}

function style-open () {
    local STRING=""
    local NO_NEWLINE=false
    if [[ -t 0 ]]; then
        local BOLD="\u001b[1m"
        local bold="\u001b[1m"
        local UNDERLINE="\u001b[4m"
        local underline="\u001b[4m"
        local REVERSED="\u001b[7m"
        local reversed="\u001b[7m"
        local BLACK="\u001b[30m"
        local black="\u001b[30m"
        local RED="\u001b[31m"
        local red="\u001b[31m"
        local GREEN="\u001b[32m"
        local green="\u001b[32m"
        local YELLOW="\u001b[33m"
        local yellow="\u001b[33m"
        local BLUE="\u001b[34m"
        local blue="\u001b[34m"
        local MAGENTA="\u001b[35m"
        local magenta="\u001b[35m"
        local CYAN="\u001b[36m"
        local cyan="\u001b[36m"
        local WHITE="\u001b[37m"
        local white="\u001b[37m"
        local RESET="\u001b[0m"
        local reset="\u001b[0m"
    fi
    for ARG in "$@"
    do
        if [[ $ARG == "-n" && $ARG == $1 ]]; then
            NO_NEWLINE=true
            continue
        fi
        case "$ARG" in
            bold|underline|black|red|green|yellow|blue|magenta|cyan|white|reversed|BOLD|UNDERLINE|BLACK|RED|GREEN|YELLOW|BLUE|MAGENTA|CYAN|WHITE|REVERSED)
                    STRING="${(P)ARG}$STRING"
                ;;
            "reset")
                STRING="$STRING$RESET"
                ;;
            *)
                STRING="$STRING$ARG"
                ;;
        esac
    done
    if [[ NO_NEWLINE == true ]]; then
        print -n "$STRING"
    else
        print "$STRING"
    fi
}
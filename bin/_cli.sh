#!/bin/bash

# CLI to call entry-point functions.
# Any command defined without prefix '_' could be invoked via CLI.

set -euo pipefail
IFS=$'\n\t'

_cli() {
  if [ $# -eq 0 ]; then
    readme
    exit 0
  fi
  local _cmd=$1
  shift

  # call entry-point function
  for fn in $(_cli_functions); do
    if [ "$_cmd" == "${fn}" ]; then
      $fn $@
      exit 0
    fi
  done

  # show help if command is not supported
  echo "Invalid invocation."
  readme
  exit 1
}

# gives list of defined functions which does not start with _
_cli_functions(){
  local _functions="$(declare -F)"
  _functions=${_functions//declare -f /}
  echo "${_functions}" | sed -ne "/^[^_].*/p"
}

_cli_commands(){
  local _functions=$(_cli_functions)
  printf "%s\n" $_functions
}

readme(){
  echo "Available commands:"
  for cmd in $(_cli_commands); do echo "  ${cmd}"; done
}

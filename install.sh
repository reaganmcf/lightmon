#!/usr/bin/env sh

### NOTE: This script is almost line for line from the starship rs project, I do not take credit for this!

set -eu

BOLD="$(tput bold 2>/dev/null || printf '')"
GREY="$(tput setaf 0 2>/dev/null || printf '')"
UNDERLINE="$(tput smul 2>/dev/null || printf '')"
RED="$(tput setaf 1 2>/dev/null || printf '')"
GREEN="$(tput setaf 2 2>/dev/null || printf '')"
YELLOW="$(tput setaf 3 2>/dev/null || printf '')"
BLUE="$(tput setaf 4 2>/dev/null || printf '')"
MAGENTA="$(tput setaf 5 2>/dev/null || printf '')"
NO_COLOR="$(tput sgr0 2>/dev/null || printf '')"

SUPPORTED_TARGETS="x86_64-unknown-linux-gnu i686-unknown-linux-musl \
		   x86_64-apple-darwin x86_64-pc-windows-msvc \
		   i686-pc-wndows-msvc"

info() {
  printf '%s\n' "${BOLD}${GREY}>${NO_COLOR} $*"
}

warn() {
  printf '%s\n' "${YELLOW}! $*${NO_COLOR}"
}

error() {
  printf '%s\n' "${RED}x $*${NO_COLOR}" >&2
}

completed() {
  printf '%s\n' "${GREEN}✓${NO_COLOR} $*"
}

has() {
	command -v "$1" 1>/dev/null 2>&1
}

# Currently supporting:
#		- win (Git Bash)
# 	- darwin
# 	- linux
detect_platform() {
	local platform;
	platform="$(uname -s | tr '[:upper:]' '[:lower:]')"

	case "${platform}" in
		mysys_nt*) platform="pc-windows-msvc" ;;
		cygwin_nt*) platform="pc-windows-msvc" ;;
		# mingw is Git-Bash
		mingw*) platform="pc-windows-msvc" ;;
		linux) platform="unknown-linux-musl" ;;
		darwin) platform="apple-darwin" ;;
	esac

	printf '%s' "${platform}"
}

# Currently supporting:
#		- x86_64
#		- i386
detect_arch() {
	local arch
	arch="$(uname -m | tr '[:upper:]' '[:lower:]')"

	case "${arch}" in
		amd64) arch="x86_64" ;;
	esac

	# `uname -m` in some cases mis-reports 32bit OS as 64bit, so double check
	if [ "${arch}" = "x86_64" ] && [ "$(getconf LONG_BIT)" -eq 32 ]; then
		arch=i686
	fi

	printf '%s' "${arch}"
}

detect_target() {
	local arch="$1"
	local platform="$2"
	local target="$arch-$platform"

	printf '%s' "${target}"
}

is_build_available() {
	local arch="$1"
	local platform="$2"
	local target="$3"

	local good

	good=$(
		IFS=" "
		for t in $SUPPORTED_TARGETS; do
			if [ "${t}" = "${target}" ]; then
				printf 1
				break
			fi
		done
	)

	if [ "${good}" != "1" ]; then
		error "${arch} builds for ${platform} are not yet availble for Lightmon"
		printf "\n" >&2
		info "If you would like to see a build for your configuration,"
		info "please create an issue requesting a build for ${MAGENTA}${target}$-{NO_COLOR}:"
		info "${BOLD}${UNDERLINE}https://github.com/reaganmcf/lightmon/issues/new/${NO_COLOR}"
		printf "\n"
		exit 1
	fi
}

# defaults
if [ -z "${PLATFORM-}" ]; then
	PLATFORM="$(detect_platform)"
fi

if [ -z "${BIN_DIR-}" ]; then
	BIN_DIR=/usr/local/bin
fi

if [ -z "${ARCH-}" ]; then
	ARCH="$(detect_arch)"
fi

if [ -z "${BASE_URL-}" ]; then
	BASE_URL="https://github.com/reaganmcf/lightmon/releases"
fi

TARGET="$(detect_target "${ARCH}" "${PLATFORM}")"

is_build_available "${ARCH}" "${PLATFORM}" "${TARGET}"

printf "  %s\n" "${UNDERLINE}Configuration${NO_COLOR}"
info "${BOLD}Bin directory${NO_COLOR}: ${GREEN}${BIN_DIR}${NO_COLOR}"
info "${BOLD}Platform${NO_COLOR}:      ${GREEN}${PLATFORM}${NO_COLOR}"
info "${BOLD}Arch${NO_COLOR}:          ${GREEN}${ARCH}${NO_COLOR}"

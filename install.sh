#!/usr/bin/env sh

### NOTE: This script is almost line for line from the starship.rs project, I do not take credit for this!

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
		   aarch64-apple-darwin x86_64-apple-darwin \
		   x86_64-pc-windows-msvc i686-pc-wndows-msvc"

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

confirm() {
  if [ -z "${FORCE-}" ]; then
    printf "%s " "${MAGENTA}?${NO_COLOR} $* ${BOLD}[y/N]${NO_COLOR}"
    set +e
    read -r yn </dev/tty
    rc=$?
    set -e
    if [ $rc -ne 0 ]; then
      error "Error reading from prompt (please re-run with the '--yes' option)"
      exit 1
    fi
    if [ "$yn" != "y" ] && [ "$yn" != "yes" ]; then
      error 'Aborting (please answer "yes" to continue)'
      exit 1
    fi
  fi
}

check_bin_dir() {
  local bin_dir="$1"

  if [ ! -d "$BIN_DIR" ]; then
    error "Installation location $BIN_DIR does not appear to be a directory"
    info "Make sure the location exists and is a directory, then try again."
    exit 1
  fi

  # https://stackoverflow.com/a/11655875
  local good
  good=$(
    IFS=:
    for path in $PATH; do
      if [ "${path}" = "${bin_dir}" ]; then
        printf 1
        break
      fi
    done
  )

  if [ "${good}" != "1" ]; then
    warn "Bin directory ${bin_dir} is not in your \$PATH"
  fi
}

# Currently supporting:
#	- win (Git Bash)
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
		linux) platform="unknown-linux-gnu" ;;
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
		arm64) arch="aarch64" ;;
	esac

	# `uname -m` in some cases mis-reports 32bit OS as 64bit, so double check
	if [ "${arch}" = "x86_64" ] && [ "$(getconf LONG_BIT)" -eq 32 ]; then
		arch=i686
	elif [ "${arch}" = "aarch64" ] && [ "$(getconf LONG_BIT)" -eq 32 ]; then
		arch=arm
	fi

	printf '%s' "${arch}"
}

detect_target() {
	local arch="$1"
	local platform="$2"
	local target="$arch-$platform"

	printf '%s' "${target}"
}

install() {
  local msg
  local sudo
  local archive
  local ext="$1"

  if test_writeable "${BIN_DIR}"; then
    sudo=""
    msg="Installing Lightmon, please wait…"
  else
    warn "Escalated permissions are required to install to ${BIN_DIR}"
    elevate_priv
    sudo="sudo"
    msg="Installing Lightmon as root, please wait…"
  fi
  info "$msg"

  archive=$(get_tmpfile "$ext")

  # download to the temp file
  download "${archive}" "${URL}"

  # unpack the temp file to the bin dir, using sudo if required
  unpack "${archive}" "${BIN_DIR}" "${sudo}"
}

# Gets path to a temporary file, even if
get_tmpfile() {
  local suffix
  suffix="$1"
  if has mktemp; then
    printf "%s.%s" "$(mktemp)" "${suffix}"
  else
    # No really good options here--let's pick a default + hope
    printf "/tmp/lightmon.%s" "${suffix}"
  fi
}

# Test if a location is writeable by trying to write to it. Windows does not let
# you test writeability other than by writing: https://stackoverflow.com/q/1999988
test_writeable() {
  local path
  path="${1:-}/test.txt"
  if touch "${path}" 2>/dev/null; then
    rm "${path}"
    return 0
  else
    return 1
  fi
}

download() {
  file="$1"
  url="$2"

  if has curl; then
    cmd="curl --fail --silent --location --output $file $url"
  elif has wget; then
    cmd="wget --quiet --output-document=$file $url"
  elif has fetch; then
    cmd="fetch --quiet --output=$file $url"
  else
    error "No HTTP download program (curl, wget, fetch) found, exiting…"
    return 1
  fi

  $cmd && return 0 || rc=$?

  error "Command failed (exit code $rc): ${BLUE}${cmd}${NO_COLOR}"
  printf "\n" >&2
  info "This is likely due to Lightmon not yet supporting your configuration."
  info "If you would like to see a build for your configuration,"
  info "please create an issue requesting a build for ${MAGENTA}${TARGET}${NO_COLOR}:"
  info "${BOLD}${UNDERLINE}https://github.com/reaganmcf/lightmon/issues/new/${NO_COLOR}"
  return $rc
}

unpack() {
  local archive=$1
  local bin_dir=$2
  local sudo=${3-}

  case "$archive" in
    *.tar.gz)
      flags=$(test -n "${VERBOSE-}" && echo "-xzvf" || echo "-xzf")
      ${sudo} tar "${flags}" "${archive}" -C "${bin_dir}"
      return 0
      ;;
    *.zip)
      flags=$(test -z "${VERBOSE-}" && echo "-qq" || echo "")
      UNZIP="${flags}" ${sudo} unzip "${archive}" -d "${bin_dir}"
      return 0
      ;;
  esac

  error "Unknown package extension."
  printf "\n"
  info "This almost certainly results from a bug in this script--please file a"
  info "bug report at https://github.com/reaganmcf/lightmon/issues"
  return 1
}

elevate_priv() {
  if ! has sudo; then
    error 'Could not find the command "sudo", needed to get permissions for install.'
    info "If you are on Windows, please run your shell as an administrator, then"
    info "rerun this script. Otherwise, please run this script as root, or install"
    info "sudo."
    exit 1
  fi
  if ! sudo -v; then
    error "Superuser not granted, aborting installation"
    exit 1
  fi
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
		error "${arch} builds for ${platform} are not yet available for Lightmon"
		printf "\n" >&2
		info "If you would like to see a build for your configuration,"
		info "please create an issue requesting a build for ${MAGENTA}${target}$-${NO_COLOR}:"
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

printf '%s - %s\n' "${ARCH}" "${PLATFORM}"

TARGET="$(detect_target "${ARCH}" "${PLATFORM}")"

is_build_available "${ARCH}" "${PLATFORM}" "${TARGET}"

printf "%s\n" "${UNDERLINE}Configuration${NO_COLOR}"
info "${BOLD}Bin directory${NO_COLOR}: ${GREEN}${BIN_DIR}${NO_COLOR}"
info "${BOLD}Platform${NO_COLOR}:      ${GREEN}${PLATFORM}${NO_COLOR}"
info "${BOLD}Arch${NO_COLOR}:          ${GREEN}${ARCH}${NO_COLOR}"

printf '\n'

EXT=tar.gz
if [ "${PLATFORM}" = "pc-windows-msvc" ]; then
  EXT=zip
fi

URL="${BASE_URL}/latest/download/lightmon-${TARGET}.${EXT}"
info "Tarball URL: ${UNDERLINE}${BLUE}${URL}${NO_COLOR}"
confirm "Install Lightmon ${GREEN}latest${NO_COLOR} to ${BOLD}${GREEN}${BIN_DIR}${NO_COLOR}"
check_bin_dir "${BIN_DIR}"

install "${EXT}"
completed "Lightmon installed"

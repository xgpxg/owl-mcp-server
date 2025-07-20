get_os() {
  case "$(uname -s)" in
    Linux*)
      echo "linux"
      ;;
    Darwin*)
      echo "macos"
      ;;
    CYGWIN* | MINGW32* | MINGW64*)
      echo "windows"
      ;;
    *)
      echo "unknown"
      ;;
  esac
}

OS=$(get_os)
WORK_DIR="/home/wxg/work/project/owl-mcp-server"
APP_NAME="$1"
PACKAGE_DIR="${WORK_DIR}/package/${APP_NAME}"
LOCAL_REPO="/home/wxg/work/project/public-resource/mcp-servers/${APP_NAME}"/"${OS}"
mkdir -p "${LOCAL_REPO}"

cp "${PACKAGE_DIR}"/"${APP_NAME}".tar.gz "${LOCAL_REPO}"/"${APP_NAME}".tar.gz
cd "${LOCAL_REPO}" || exit
git pull --rebase origin master
git add . && git commit -m "update ${APP_NAME}" &&  git push

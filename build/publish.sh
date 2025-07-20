#get_os() {
#  case "$(uname -s)" in
#    Linux*)
#      echo "linux"
#      ;;
#    Darwin*)
#      echo "macos"
#      ;;
#    CYGWIN* | MINGW32* | MINGW64*)
#      echo "windows"
#      ;;
#    *)
#      echo "unknown"
#      ;;
#  esac
#}

OS=$2
APP_NAME="$1"
# 远程存放包的目录
REMOTE="wxg@192.168.1.242"
PACKAGE="/home/wxg/work/project/owl-mcp-server/package/${APP_NAME}/${OS}/${APP_NAME}.tar.gz"
REMOTE_RELEASE_DIR="/home/pub/packages/releases/fs-kb-app/mcp-servers/${APP_NAME}/${OS}"
PACKAGE_NAME="$1"


echo "开始发布 ${PACKAGE_NAME}"

ssh ${REMOTE} "mkdir -p ${REMOTE_RELEASE_DIR}"
# 复制到发布目录
scp  "${PACKAGE}" "${REMOTE}:${REMOTE_RELEASE_DIR}/${APP_NAME}.tar.gz"
echo "发布完成"

WORK_DIR="/home/wxg/work/project/owl-mcp-server"
APP_NAME="$1"
PACKAGE_DIR="${WORK_DIR}/package/${APP_NAME}"
LOCAL_REPO="/home/wxg/work/project/public-resource/mcp-servers/${APP_NAME}"

cp "${PACKAGE_DIR}"/"${APP_NAME}".tar.gz "${LOCAL_REPO}"/"${APP_NAME}".tar.gz
cd "${LOCAL_REPO}" || exit
git add . && git commit -m "update ${APP_NAME}" &&  git push
#!/bin/bash

TARGET=x86_64-pc-windows-gnu
# 应用名
APP_NAME="$2"
## 工作目录
WORK_DIR="/home/wxg/work/project/owl-mcp-server"
# 前端工作目录
FRONTEND_WORK_DIR="/home/wxg/work/project/one-api-client/mcp/${APP_NAME}"
# 可执行文件路径
BIN_PATH="${WORK_DIR}/target/${TARGET}/release/${APP_NAME}.exe"
# 打包目录
PACKAGE_DIR="${WORK_DIR}/package/${APP_NAME}/windows/"


package(){
  echo "开始打包${APP_NAME}"
  cargo build -r -p ${APP_NAME} --target ${TARGET}
  if [ $? -ne 0 ]; then
    echo "编译失败"
    exit 1
  fi

  if [ "$APP_NAME" = "http-api-call" ]; then
    echo "正在编译前端"
    cd ${FRONTEND_WORK_DIR} || exit 1
    npm install
    npm run build
    if [ $? -ne 0 ]; then
      echo "编译前端失败"
      exit 1
    fi
  fi

  echo "正在打包到 ${PACKAGE_DIR}"
  # 清空打包目录
  rm -rf ${PACKAGE_DIR}
  # 打包目录
  mkdir -p ${PACKAGE_DIR}
  # 复制二进制文件
  rsync -av ${BIN_PATH} "${PACKAGE_DIR}/${APP_NAME}.exe"

  resources=""
  if [ "$APP_NAME" = "http-api-call" ]; then
    resources="resources"
    # 资源目录
    mkdir -p ${PACKAGE_DIR}/resources
    # 复制前端文件
    rsync -av ${FRONTEND_WORK_DIR}/dist/ ${PACKAGE_DIR}/resources/web
  fi

  # 生成压缩包
  tar -czvf ${PACKAGE_DIR}/${APP_NAME}.tar.gz -C ${PACKAGE_DIR} ${APP_NAME}.exe ${resources}

  echo "打包完成！"
}

case "$1" in
    package)
        package
        ;;
    *)
        echo "Usage: $0 package"
        exit 1
        ;;
esac
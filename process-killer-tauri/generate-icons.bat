@echo off
echo 正在为 Tauri 应用生成图标...

REM 检查是否提供了图标文件参数
if "%1"=="" (
    echo 用法: generate-icons.bat path/to/your/icon.png
    echo 示例: generate-icons.bat assets/my-icon.png
    exit /b 1
)

REM 检查文件是否存在
if not exist "%1" (
    echo 错误: 图标文件 "%1" 不存在
    exit /b 1
)

echo 使用图标文件: %1

REM 使用 Tauri CLI 生成图标
npx @tauri-apps/cli icon "%1"

if %errorlevel% equ 0 (
    echo 图标生成成功！
    echo 生成的图标文件位于: src-tauri/icons/
) else (
    echo 图标生成失败，请检查：
    echo 1. 图标文件是否为有效的 PNG 格式
    echo 2. 图标文件分辨率是否足够高 (建议 1024x1024)
    echo 3. 是否已安装 Tauri CLI
)

pause
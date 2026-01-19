# Tauri 自定义图标指南

## 需要的图标格式和尺寸

### 基本格式
- `icon.png` - 主图标文件（建议 512x512 或 1024x1024）
- `32x32.png` - 小尺寸图标
- `128x128.png` - 中等尺寸图标
- `128x128@2x.png` - 高分辨率显示器用图标（256x256）

### 平台特定格式
- `icon.ico` - Windows 图标文件
- `icon.icns` - macOS 图标文件

### Windows Store 格式（如果发布到 Microsoft Store）
- `Square30x30Logo.png` (30x30)
- `Square44x44Logo.png` (44x44)
- `Square71x71Logo.png` (71x71)
- `Square89x89Logo.png` (89x89)
- `Square107x107Logo.png` (107x107)
- `Square142x142Logo.png` (142x142)
- `Square150x150Logo.png` (150x150)
- `Square284x284Logo.png` (284x284)
- `Square310x310Logo.png` (310x310)
- `StoreLogo.png` (50x50)

## 生成步骤

### 方法一：使用 Tauri CLI（推荐）
```bash
# 确保有一个高质量的源图标（建议 1024x1024 PNG）
npx @tauri-apps/cli icon your-source-icon.png

# 或者如果全局安装了 Tauri CLI
tauri icon your-source-icon.png
```

### 方法二：在线工具生成
1. 使用在线图标生成器如：
   - https://www.favicon-generator.org/
   - https://realfavicongenerator.net/
   - https://iconifier.net/

2. 上传你的源图标
3. 下载生成的各种格式
4. 替换 `src-tauri/icons/` 目录下的对应文件

### 方法三：使用设计工具
- **Figma/Sketch**: 导出不同尺寸的 PNG
- **GIMP/Photoshop**: 批量导出不同尺寸
- **Inkscape**: 从 SVG 导出不同尺寸的 PNG

## 图标设计建议

1. **使用矢量格式**: 从 SVG 开始设计，便于缩放
2. **简洁设计**: 小尺寸下仍然清晰可辨
3. **高对比度**: 确保在不同背景下都清晰
4. **避免细节过多**: 32x32 像素下细节会丢失
5. **测试不同尺寸**: 确保所有尺寸都看起来不错

## 配置文件

图标在 `tauri.conf.json` 中配置：

```json
{
  "bundle": {
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png", 
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

## 验证图标

构建应用后检查：
```bash
npm run tauri build
```

检查生成的安装包是否使用了正确的图标。
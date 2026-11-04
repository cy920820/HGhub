<div align="center">

# HGhub

**黄果短剧第三方客户端 · Rust + Tauri 2**

Windows / macOS / Android / iOS 四端 · Fluent 设计 · 沉浸式竖屏播放
![alt text](docs/image.png)

</div>

> ⚠️ 本项目仅供 **学习与技术交流**，不提供、不存储、不分发任何音视频内容。
> 所有内容均来自第三方站点，请自行判断合规性，并在 24 小时内删除相关资源。


## 技术栈

| 层 | 选型 |
| --- | --- |
| 壳 | Tauri 2（Rust 1.9x） |
| 前端 | React 19 + TypeScript + Vite 7 + Tailwind CSS 4 |
| 播放 | hls.js（HLS / AES-128） |
| 网络 | reqwest + rustls |
| 图标 | 内联 SVG |

## 目录

```
.
├─ src/                      前端
│  ├─ api.ts                 接口封装 + 本地媒体协议 URL
│  ├─ store.tsx              全局状态（导航栈 / 用户 / 本地收藏 / 主题）
│  ├─ html.ts                吃瓜正文清洗与图片代理
│  ├─ components/            Feed（播放页）/ VideoSurface / PlayerControls / Panels / UI
│  └─ pages/                 Home / Search / Category / Profile / Posts
├─ src-tauri/                Rust 后端
│  └─ src/
│     ├─ net.rs              请求层（边缘信封 + 重试）
│     ├─ api.rs              业务命令
│     ├─ media.rs            封面解密 + HLS 代理 + 缓存
│     └─ session.rs          token 持久化
└─ docs/API.md               接口文档
```

## 开发 / 构建

```bash
pnpm install          # 首次安装
pnpm app:dev          # 桌面端热重载
pnpm app:build        # 打包 Windows 安装程序（NSIS）
```

Android（需 Android SDK + NDK，并配置 `ANDROID_HOME` / `NDK_HOME`）：

```bash
pnpm android:init     # 首次生成 android 工程
pnpm android:dev      # 真机 / 模拟器调试
pnpm android:build    # 产出 APK
```

macOS（需 Xcode + Command Line Tools；`.app` 与 `.dmg` 都要在这上面出，跨平台打不了）：

```bash
pnpm app:dev                   # 热重载
pnpm mac:build                 # .app + .dmg（当前架构）
pnpm tauri build --bundles app,dmg --target universal-apple-darwin   # 通用二进制（需 rustup target add x86_64-apple-darwin）
```

产物在 `src-tauri/target/release/bundle/{macos,dmg}/`；用 `--target` 时在 `target/<triple>/release/bundle/`。
未签名时首次打开要「右键 → 打开」，或 `xattr -dr com.apple.quarantine /Applications/HGhub.app`。

iOS（需 Xcode 与一个 Apple Developer 账号；描述文件要覆盖 bundle id `com.hghub.huangguo`）：

```bash
export APPLE_DEVELOPMENT_TEAM=你的TeamID   # 也可以写进 tauri.conf.json 的 bundle.iOS.developmentTeam
pnpm ios:init                  # 生成 src-tauri/gen/apple（该目录不入库）
pnpm ios:dev                   # 模拟器 / 真机调试
pnpm ios:build                 # = tauri ios build --export-method debugging，产 .ipa
```

`tauri info` 会列出可用的签名 Team。IPA 落在 `src-tauri/gen/apple/build/arm64/`，
用 `debugging`（旧名 `development`）导出的是**需注册设备**的开发包；要上 TestFlight / App Store
换成 `--export-method release-testing` 或 `app-store-connect`，并按 Apple 要求配 API 密钥。

不想折腾证书就打 tag 让 CI 出**未签名 IPA**（`HGhub_<版本>_unsigned.ipa`）：iOS 任务在缺 Apple Secrets 时会自动走
`CODE_SIGNING_ALLOWED=NO` 的 xcodebuild 分支，见 [`.github/workflows/release.yml`](.github/workflows/release.yml) 的「编译未签名 .app 并打成 IPA」。
这种包只能重签后安装（AltStore / Sideloadly 用免费 Apple ID，7 天）或装到越狱设备上。

### 环境注意

- 工程若放在 **exFAT** 盘（不支持符号链接），需保留 `.npmrc` 中的 `node-linker=hoisted`。
- `esbuild` 需在 `package.json` 的 `pnpm.onlyBuiltDependencies` 中放行，否则 `vite build` 缺少二进制。
- iOS 工程生成依赖 CocoaPods（`brew install cocoapods`）与 Xcode；`src-tauri/gen/` 不入库，换机器要重新 `pnpm ios:init`。
- Xcode 26 起 iOS 平台组件是按需下载的（约 10 GB+）：`pnpm ios:build` 会先用 `xcrun simctl list runtimes` 探测，
  探测不到就交互式提示装（`xcodebuild -downloadPlatform iOS`）。没装时 `xcodebuild -destination 'generic/platform=iOS'`
  直接报 `iOS 26.0 is not installed`，改用 `-target … -sdk iphoneos` 能绕开目的地解析（`libapp.a` 与可执行文件都能编出来），
  但 `ibtool` 编译 `LaunchScreen.storyboard` 仍要该组件 —— 所以**本地出包绕不开这次下载，建议交给 CI**。
  GitHub 的 macos runner 自带该组件。
- `tauri ios xcode-script`（Xcode 的 Build Rust Code 阶段）会无条件连回父 CLI 进程的 JSON-RPC
  （`${TMPDIR}/<identifier>-server-addr`），所以不能让 `tauri ios build` 单独用 `xcodebuild` 顶掉；
  未签名构建的做法是先后台跑 `tauri ios build --open`（它只装前端 + 起 RPC，然后挂住不退出），再让 `xcodebuild` 干活，见 workflow 的 iOS 任务。

### CI 出包（`.github/workflows/release.yml`）

打 `v*` tag 自动出 Windows / macOS / Android 产物。iOS 有两种结果：配齐下面 4 个 Apple Secrets 出**签名 IPA**；
一个都不配则退化成**未签名 IPA**（`HGhub_<版本>_unsigned.ipa`），日志里给 warning。未签名包装不到正常的 iPhone 上，
需要先重签：AltStore / Sideloadly（免费 Apple ID，7 天有效）或自己 `codesign`，越狱设备可直接装。

| Secret | 用途 |
| --- | --- |
| `ANDROID_KEY_BASE64` / `ANDROID_KEY_ALIAS` / `ANDROID_KEYSTORE_PASSWORD` / `ANDROID_KEY_PASSWORD` | APK 签名 |
| `APPLE_DEVELOPMENT_TEAM` | Team ID，写进 Xcode 的 `DEVELOPMENT_TEAM` |
| `IOS_CERTIFICATE` | Apple 发布证书 `.p12` 的 base64 |
| `IOS_CERTIFICATE_PASSWORD` | `.p12` 口令 |
| `IOS_MOBILE_PROVISION` | 覆盖 `com.hghub.huangguo` 的描述文件 `.mobileprovision` 的 base64 |

macOS 默认出未签名的 `.app` / `.dmg`；要签名+公证再补 `APPLE_SIGNING_IDENTITY` 与 Tauri 的 notarization 相关 Secrets。

## 实现要点

1. **统一边缘信封**：站点把 `/api/**` 收敛为 `POST /api/edge/request`（`method/path/body(base64)/content_type` + `Authorization: Bearer`），本项目只对接这一个入口，无需处理 Cookie。
2. **封面解密**：封面为 AES-128-CBC 密文，Rust 侧解密后经自定义协议输出，前端 `<img>` 直接使用。
3. **HLS 代理**：m3u8 与分片经自定义协议转发并改写 `#EXT-X-KEY` / 分片地址，绕开 WebView 的 CORS 与防盗链。
4. **自定义协议**：同一份 `hg` 协议在各宿主外面套的壳不同 —— Windows(WebView2) 与 Android 是
   `http://hg.localhost/*`，macOS / iOS(WKWebView) 是真自定义协议 `hg://localhost/*`。
   前端 `src/api.ts` 的 `LOCAL` 和 Rust `src-tauri/src/lib.rs` 的 `origin` 用同一套平台判断，
   两边必须一致，否则 m3u8 改写出来的分片地址 WebView 解析不了。
   注意**不要**从 `request.uri()` 反推：wry 在 Windows/Android 上会先把 `http(s)://hg.` 还原成
   `hg://` 再回调，反推会拿到一个页面里根本打不开的地址。
5. **iOS 播放**：iPhone 的 WKWebView 没有 MSE，hls.js 只能靠 iOS 17.1+ 的 ManagedMediaSource，
   而原生 `<video>` 又啃不动 `hg://` 自定义协议 —— 所以 `bundle.iOS.minimumSystemVersion` 定在 **17.1**，
   让 iOS 与其它端共用同一套代理链路。
6. **不做签名逆向**：只复用浏览器同源行为，接口字段均有文档与实测样本。
7. **iOS 触感与键盘**：iOS 的 WebKit 不实现 Vibration API，长按倍速等反馈走 `tauri-plugin-haptics`
   （Android 仍用 `navigator.vibrate`，见 `src/haptics.ts`）；弹层用 `visualViewport` 量键盘高度后上移
   （`src/keyboard.ts` + `Modal`），因为 iOS 的 layout viewport 不随键盘缩小。
   加了 Tauri 插件后 `gen/` 要重新 `pnpm ios:init` / `pnpm android:init` 才会带上原生侧。

接口细节见 [docs/API.md](docs/API.md)。

## 免责声明

- 本项目与内容提供方无任何关联，作者不参与内容的制作、上传与分发。
- 使用者应自行遵守所在地法律法规；因使用本项目产生的任何后果由使用者自行承担。
- 若相关权利方认为本项目存在侵权，请通过 Issue 联系，我们会及时处理。

## License

[MIT](LICENSE)
## 常见问题

**1. `pnpm install` 报 `ERR_PNPM_EISDIR ... symlink`**

工程放在 exFAT / FAT32 盘时无法创建符号链接，保留 `.npmrc` 里的 `node-linker=hoisted` 即可。

**2. `tauri android build` 报 `Failed to create a symbolic link ... jniLibs`**

同样是 exFAT 不支持符号链接。可手动把 Rust 产物拷进 jniLibs，再直接跑 Gradle：

```bash
cp src-tauri/target/aarch64-linux-android/debug/libhuangguo_client_lib.so \
   src-tauri/gen/android/app/src/main/jniLibs/arm64-v8a/

cd src-tauri/gen/android
./gradlew assembleDebug        # Windows: gradlew.bat assembleDebug
```

产物在 `src-tauri/gen/android/app/build/outputs/apk/`。

**3. Android 构建提示找不到 SDK / NDK**

需要设置 `ANDROID_HOME`（Android SDK 根目录）、`NDK_HOME`（`$ANDROID_HOME/ndk/<版本>`）与 `JAVA_HOME`（JDK 17）。

## LinuxDO
https://linux.do/
<div align="center">

# HGhub

**黄果短剧第三方客户端 · Rust + Tauri 2**

Windows / macOS / Android / iOS 四端 · Fluent 设计 · 沉浸式竖屏播放
![alt text](docs/image.png)

</div>

> ⚠️ 本项目仅供 **学习与技术交流**，不提供、不存储、不分发任何音视频内容。
> 所有内容均来自第三方站点，版权归原作者与上游站点所有。

**使用须知**

- **以访客身份正常使用**：像普通访客一样浏览就好，请勿滥用本项目。
- **不要给上游增加压力**：本项目本身不提供下载功能，请勿改动代码去做批量下载 / 批量抓取，也不要高频轮询、并发轰炸接口或挂机刷播放量；内置的封面与播放列表缓存请不要改小或关掉。
- **尊重上游版权**：不要去除水印与站点标识，不要把剧集搬运、转码、修改后二次分发到别处（网盘、资源站、其他 App 等），更不能改完拿出去出售。
- **不要公开传播接口与直链**：接口地址、播放直链、解密方式请勿公开张贴或打包成「资源站」——上游一旦封禁，所有人都没法再用。
- **不要商业化**：禁止打包出售、接入广告 / 会员 / 付费点播，或作为收费服务的一部分分发（详见 [LICENSE](LICENSE)）。
- **合理使用**：仅供个人学习、研究与自用；请在 24 小时内删除相关资源，并支持正版。
- **遵守当地法律**：使用者应自行判断并承担合规责任。

> ⚖️ 许可证自 v1.1.0 版本起改为 **PolyForm Noncommercial 1.0.0**：**禁止任何商业用途**——
> 包括打包出售、接入付费/会员/广告变现、作为收费产品或服务的一部分分发。
> 个人学习、研究、自用与非营利使用不受限制，详见 [LICENSE](LICENSE)。


## 官方渠道与仿冒提醒

- **唯一官方仓库**：[github.com/Minireel/HGhub](https://github.com/Minireel/HGhub)
- **唯一下载渠道**：[GitHub Releases](https://github.com/Minireel/HGhub/releases)。仓库源码页不放任何可执行文件，
  其余网站、网盘、QQ 群、公众号、第三方下载站分发的「HGhub」安装包**一律非官方**，作者不保证其安全性与完整性。

> 🚫 **已发现的恶意修改版**：[`hichuv/HGhub`](https://github.com/hichuv/HGhub)
>
> 该仓库与本项目无关，是在旧版本源码基础上擅自加入**卡密验证、VIP 付费解锁**等变现功能的修改版，
> 代码改动不透明、安全性无法验证。**请勿下载、勿注册、勿付费**；如已安装，建议立即卸载，改用官方 Release 包安装。
>
> 本项目**从未、也不会**加入任何卡密 / VIP / 付费解锁 / 广告功能。如果你遇到的「HGhub」要求输入卡密或付费开通，那一定不是官方版本。


## 技术栈

| 层 | 选型 |
| --- | --- |
| 壳 | Tauri 2（Rust 1.90+） |
| 前端 | React 19 + TypeScript + Vite 7 + Tailwind CSS 4 |
| 播放 | hls.js（HLS / AES-128） |
| 网络 | reqwest + rustls |

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

> 下面的本地命令用于调试与自测。正式出包（Windows / macOS / Android / iOS）统一交给 CI，见「CI 出包」——
> 本地产物落在 `src-tauri/target/`，清目录或换机器就没了，别把它当成安装。

```bash
pnpm install          # 首次安装
pnpm app:dev          # 桌面端热重载
pnpm app:build        # 打包：Windows → NSIS 安装程序；macOS → .app + .dmg
```

macOS 先装 Xcode Command Line Tools（`xcode-select --install`），打包目标写在
`src-tauri/tauri.macos.conf.json`（`app` + `dmg`，窗口用系统标题栏 Overlay）。

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
- pnpm 10 靠 `package.json` 的 `pnpm.onlyBuiltDependencies` 放行 `esbuild`，否则 `vite build` 缺少二进制；
  pnpm 11 起该字段失效，白名单改在 `pnpm-workspace.yaml` 的 `allowBuilds`（两处都保留）。
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

四个平台的包都在 CI 出：打 `v*` tag 除了产物还会发一个带全平台安装包的 GitHub Release；
手动 Run workflow（不分平台）只留 artifact，适合验证。iOS 有两种结果：配齐下面 4 个 Apple Secrets 出**签名 IPA**；
一个都不配则退化成**未签名 IPA**（`HGhub_<版本>_unsigned.ipa`），日志里给 warning。未签名包装不到正常的 iPhone 上，
需要先重签：AltStore / Sideloadly（免费 Apple ID，7 天有效）或自己 `codesign`，越狱设备可直接装。

取产物（artifact 名就是任务名缩写：`windows` / `macos` / `ios` / `android`）：

```bash
gh run download <run-id> -n macos -D ./out     # 或 -a 一次拉全部
```

| Secret | 用途 |
| --- | --- |
| `ANDROID_KEY_BASE64` / `ANDROID_KEY_ALIAS` / `ANDROID_KEYSTORE_PASSWORD` / `ANDROID_KEY_PASSWORD` | APK 签名。缺 `ANDROID_KEY_BASE64` 时退化成**未签名 APK**（`HGhub_<版本>_<abi>_unsigned.apk`），日志给 warning；装到设备前需自己 `apksigner` 签一次 |
| `APPLE_DEVELOPMENT_TEAM` | Team ID，写进 Xcode 的 `DEVELOPMENT_TEAM`。tauri-cli 让这个环境变量覆盖配置值，所以**空字符串**会让 `tauri ios init --ci` 报 `apple.development-team is empty`；未签名路径缺省用占位 `AAAAAAAAAA` |
| `IOS_CERTIFICATE` | Apple 发布证书 `.p12` 的 base64 |
| `IOS_CERTIFICATE_PASSWORD` | `.p12` 口令 |
| `IOS_MOBILE_PROVISION` | 覆盖 `com.hghub.huangguo` 的描述文件 `.mobileprovision` 的 base64 |

macOS 默认出未签名的 `.app` / `.dmg`；要签名+公证再补 `APPLE_SIGNING_IDENTITY` 与 Tauri 的 notarization 相关 Secrets。

### 同步上游（`.github/workflows/sync-upstream.yml`）

fork 的 `main` 用这个任务跟 `Minireel/HGhub` 的 `main` 对齐：能 fast-forward 就 ff，
fork 有自己的提交（例如为注册 workflow 补的空提交）就退化成 merge commit，真冲突则失败并提示手动 `git merge upstream/main`。

- 手动：Actions → Sync upstream → Run workflow（定时任务在 fork 上默认关闭，要在 Settings → Actions → General 勾上才自动跑）。
- 本地等价操作：
  ```bash
  git remote add upstream https://github.com/Minireel/HGhub.git   # 已存在就跳过
  git fetch upstream main
  git checkout main && git merge --ff-only upstream/main && git push fork main
  git fetch upstream && git rebase upstream/main <你的功能分支>    # 功能分支要自己跟上
  ```

## 实现要点

安全加固后的正文处理、FakeIP/内网限制、账号线路隔离及本地回归方法，见 [客户端内容与网络安全](docs/security.md)。镜像线路仅支持游客浏览，跨站点切换和首次升级后需要重新登录。

1. **统一边缘信封**：站点把 `/api/**` 收敛为 `POST /api/edge/request`（`method/path/body(base64)/content_type` + `Authorization: Bearer`），本项目只对接这一个入口，无需处理 Cookie。
2. **封面解密**：封面为 AES-128-CBC 密文，Rust 侧解密后经自定义协议输出，前端 `<img>` 直接使用。
3. **HLS 代理**：m3u8 与分片经自定义协议转发并改写 `#EXT-X-KEY` / 分片地址，绕开 WebView 的 CORS 与防盗链。
4. **自定义协议**：同一份 `hg` 协议在各宿主外面套的壳不同 —— Windows(WebView2) 与 Android 是
   `http://hg.localhost/*`，macOS / iOS(WKWebView) 是真自定义协议 `hg://localhost/*`。
   Rust 侧基址定义在 `src-tauri/src/media.rs` 的 `LOCAL_ORIGIN`（`lib.rs` 用它改写 m3u8），
   前端 `src/api.ts` 的 `LOCAL` 用同一套平台判断，两边必须一致，否则封面与视频全部加载失败。
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


## 免责声明

- 本项目与内容提供方无任何关联，作者不参与内容的制作、上传与分发。
- 本项目仅在上述官方渠道发布；任何第三方修改版（尤其是加卡密、VIP 变现的版本）均与作者无关，请自行甄别。
- 使用者应自行遵守所在地法律法规；因使用本项目产生的任何后果由使用者自行承担。
- 若相关权利方认为本项目存在侵权，请通过 Issue 联系，我们会及时处理。

## License

[PolyForm Noncommercial License 1.0.0](LICENSE) —— **仅限非商业用途**。

- **允许**：个人学习、研究、自用、爱好项目，以及在非营利组织内部使用；可以自由修改、二次开发。
- **禁止**：任何商业用途。包括但不限于打包出售、接入付费/会员/广告变现、作为收费服务或商业产品的一部分分发。
- **保留声明**：分发本项目或其修改版时，必须附带本许可证全文（或上面的链接）以及 `Required Notice` 声明行。
- 商业授权请通过 Issue 联系作者单独洽谈。

> 历史说明：v1.0.1 及更早版本以 MIT 发布，已发出的副本继续按 MIT 授权；自 **v1.1.0** 起的新版本一律适用 PolyForm Noncommercial 1.0.0。

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

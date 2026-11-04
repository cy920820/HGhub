mod api;
mod media;
mod net;
mod session;

use api::Api;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // iOS 的 WebKit 没有 Vibration API，触感只能走 UIImpactFeedbackGenerator；
        // 桌面端该插件是空实现，不影响 Windows/macOS。
        .plugin(tauri_plugin_haptics::init())
        .manage(Api::new())
        .setup(|app| {
            if let Ok(dir) = app.path().app_data_dir() {
                let _ = std::fs::create_dir_all(&dir);
                let state: tauri::State<Api> = app.state();
                state.session.bind(dir);
            }
            // 无边框窗口在 macOS 上仍然需要菜单栏：WKWebView 的复制/粘贴/全选走
            //  responder chain，没有 Edit 菜单时登录框连密码都粘不进去。
            #[cfg(target_os = "macos")]
            {
                let menu = tauri::menu::Menu::default(app.handle())?;
                app.set_menu(menu)?;
            }
            Ok(())
        })
        .register_asynchronous_uri_scheme_protocol("hg", |_app, request, responder| {
            let uri = request.uri().clone();
            let path = uri.path().to_string();
            let query = uri.query().unwrap_or("").to_string();
            // m3u8 里的分片 / key 要改写成 WebView 真能解析的绝对地址，而各宿主给自定义协议
            // 套的壳不一样：WKWebView（macOS / iOS）是真自定义协议 hg://localhost，
            // WebView2 与 Android 是 http://hg.localhost。
            // 不能从 request.uri() 反推：wry 在 Windows/Android 上会先把 http(s)://hg. 还原成
            // hg:// 再回调，反推会得到一个 WebView 解析不了的地址。必须和前端 api.ts 的
            // LOCAL 用同一套判断。
            let origin = if cfg!(any(target_os = "macos", target_os = "ios")) {
                "hg://localhost"
            } else {
                "http://hg.localhost"
            }
            .to_string();
            tauri::async_runtime::spawn(async move {
                let resp = media::handle(&path, &query, &origin).await;
                responder.respond(resp);
            });
        })
        .invoke_handler(tauri::generate_handler![
            api::app_boot,
            api::debug_log,
            api::net_diag,
            api::api_login,
            api::api_register,
            api::api_logout,
            api::api_me,
            api::api_feed,
            api::api_category,
            api::api_rank,
            api::api_search,
            api::cache_clear,
            api::api_posts,
            api::api_post,
            api::api_post_like,
            api::api_post_collect,
            api::api_post_comments,
            api::api_video,
            api::api_play,
            api::api_comments,
            api::api_comment_send,
            api::api_like,
            api::api_collect,
            api::api_follow,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

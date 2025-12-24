#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 初始化日志 (完全由环境变量 RUST_LOG 控制)
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("开盘啦应用启动");

    // 运行 Tauri 应用
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

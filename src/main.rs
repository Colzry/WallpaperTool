use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    time::Duration,
    thread,
    fs,
    ptr,
};
use std::os::windows::ffi::OsStringExt;
use winapi::{
    ctypes::c_void,
    um::{
        winuser::{SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFILE, SPIF_SENDWININICHANGE},
        shlobj::{SHGetKnownFolderPath},
        combaseapi::CoTaskMemFree,
        knownfolders::FOLDERID_Desktop,
    },
};
use rand::seq::SliceRandom;
use clap::{Parser, ValueEnum};
use widestring::U16CString;
use winapi::um::winuser::SystemParametersInfoW;

// 定义轮换模式的枚举
#[derive(Debug, Clone, ValueEnum)]
enum RotationMode {
    Random,     // 随机模式
    Sequential, // 顺序模式
}

// 定义命令行参数结构
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 图片文件路径或包含图片的文件夹路径
    path: String,

    /// 启用壁纸轮换功能（仅对文件夹路径有效）
    #[arg(short = 'r', long)]
    rotate: bool,

    /// 轮换间隔时间（分钟）
    #[arg(short = 'i', long, default_value_t = 15, requires = "rotate")]
    interval: u64,

    /// 轮换模式（随机或顺序）
    #[arg(short = 'm', long, value_enum, default_value_t = RotationMode::Sequential, requires = "rotate")]
    mode: RotationMode,
}

// 设置壁纸的函数
fn set_wallpaper(image_path: &str) -> Result<(), String> {
    let path = Path::new(image_path);
    if !path.exists() {
        return Err(format!("路径不存在: {}", image_path));
    }

    // 转换为绝对路径
    let absolute_path = fs::canonicalize(path).map_err(|e| e.to_string())?;
    let path_str = absolute_path.to_str().ok_or("无效路径")?;

    // 使用宽字符版本
    let wide_path = U16CString::from_str(path_str).map_err(|_| "无法转换路径为宽字符")?;

    // 调用 Windows API 设置壁纸 (使用W版本)
    unsafe {
        let result = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            wide_path.as_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE | SPIF_SENDWININICHANGE,
        );

        if result == 0 {
            return Err(format!("设置壁纸失败，错误代码: {}", std::io::Error::last_os_error()));
        }
    }

    Ok(())
}

#[allow(dead_code)]
// 获取壁纸目录的函数
fn get_wallpaper_directory() -> PathBuf {
    unsafe {
        let mut path_ptr: *mut u16 = ptr::null_mut();
        if SHGetKnownFolderPath(&FOLDERID_Desktop, 0, ptr::null_mut(), &mut path_ptr) == 0 {
            let wide_str = U16CString::from_ptr_str(path_ptr);
            CoTaskMemFree(path_ptr as *mut _);
            PathBuf::from(OsString::from_wide(wide_str.as_slice()))
        } else {
            PathBuf::from(".")
        }
    }
}

// 获取目录中所有图片文件的函数
fn get_image_files(dir_path: &str) -> Result<Vec<PathBuf>, String> {
    let dir = Path::new(dir_path);
    if !dir.is_dir() {
        return Err("提供的路径不是目录".to_string());
    }

    let mut image_files = Vec::new();
    let entries = fs::read_dir(dir).map_err(|e| e.to_string())?;

    // 遍历目录中的文件
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                // 检查文件扩展名是否为支持的图片格式
                if ["jpg", "jpeg", "png", "bmp"].contains(&ext.as_str()) {
                    image_files.push(path);
                }
            }
        }
    }

    if image_files.is_empty() {
        return Err("目录中没有找到图片文件".to_string());
    }

    Ok(image_files)
}

// 壁纸轮换函数
fn rotate_wallpapers(dir_path: &str, interval: u64, mode: RotationMode) -> Result<(), String> {
    let mut image_files = get_image_files(dir_path)?;
    let mut index = 0;

    // 如果是随机模式，先打乱图片顺序
    match mode {
        RotationMode::Random => {
            image_files.shuffle(&mut rand::thread_rng());
        }
        RotationMode::Sequential => {} // 顺序模式不需要特殊处理
    }

    // 无限循环轮换壁纸
    loop {
        let current_image = &image_files[index % image_files.len()];
        if let Err(e) = set_wallpaper(current_image.to_str().unwrap()) {
            eprintln!("设置壁纸时出错: {}", e);
        } else {
            println!("壁纸已设置为: {}", current_image.display());
        }

        // 等待指定的时间间隔
        thread::sleep(Duration::from_secs(interval * 60));

        // 根据模式更新索引
        match mode {
            RotationMode::Sequential => index += 1,
            RotationMode::Random => {
                // 随机模式下每次轮换都重新打乱顺序
                image_files.shuffle(&mut rand::thread_rng());
                index = 0;
            }
        }
    }
}

// 主函数
fn main() {
    // 解析命令行参数
    let args = Args::parse();

    if args.rotate {
        // 启用轮换模式
        if let Err(e) = rotate_wallpapers(&args.path, args.interval, args.mode) {
            eprintln!("错误: {}", e);
        }
    } else {
        // 单张壁纸模式
        if let Err(e) = set_wallpaper(&args.path) {
            eprintln!("错误: {}", e);
        } else {
            println!("壁纸已设置为: {}", args.path);
        }
    }
}

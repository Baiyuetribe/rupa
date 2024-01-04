// 定义统一的utils

use rand::Rng; // 随机字符
use regex::Regex;
use sha1::{Digest, Sha1};
use std::collections::HashSet;

pub fn now_unix() -> i64 {
	chrono::Utc::now().timestamp()
}

// 这种更高效
pub fn rand_str(length: usize) -> String {
	const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz)(*&^%$#@!~";
	let mut rng = rand::thread_rng();

	let rand_string: String = (0..length)
		.map(|_| {
			let idx = rng.gen_range(0..CHARSET.len());
			CHARSET[idx] as char
		})
		.collect();
	rand_string
}

pub fn sha1(input: &str) -> String {
	let mut hasher = Sha1::default();

	hasher.update(input.as_bytes());
	let result = hasher.finalize();
	format!("{:x}", result)
}

pub fn is_email(word: &str) -> bool {
	let re = Regex::new(r"^[A-Za-z\d]+([-_.][A-Za-z\d]+)*@([A-Za-z\d]+[-.])+[A-Za-z\d]{2,4}$").unwrap();
	re.is_match(word)
}

pub fn is_phone(word: &str) -> bool {
	let re = Regex::new(r"^1[356789]\d{9}$").unwrap();
	re.is_match(word)
}

// 字符串列表去重、去空
pub fn remove_duplicates_and_empty(a: Vec<String>) -> Vec<String> {
	let mut set = HashSet::new();
	let mut ret = Vec::new();

	for item in a {
		if item.is_empty() || !set.insert(item.clone()) {
			continue;
		}
		ret.push(item);
	}

	ret
}

use encoding_rs::GBK;
use std::fs::File;
use std::io::Read;

// 仅限utf8和gbk
pub fn file2utf8(path: &str) -> String {
	let file = match File::open(path) {
		Ok(file) => Some(file),
		Err(_) => {
			return String::new();
		}
	};
	let mut contents = String::new();
	match file {
		Some(mut file) => match file.read_to_string(&mut contents) {
			Ok(_) => contents,
			Err(_) => {
				let mut file = File::open(path).unwrap();
				let bytes = unsafe {
					let mut v = Vec::new();
					file.read_to_end(&mut v).unwrap();
					v
				};
				let (cow, _, _) = GBK.decode(&bytes);
				cow.into_owned()
			}
		},
		None => String::new(),
	}
}
use std::fs;
// 判断是否为目录
pub fn is_dir(path: &str) -> bool {
	match fs::metadata(path) {
		Ok(metadata) => metadata.is_dir(),
		Err(_) => false,
	}
}

// 判断文件是否存在
pub fn is_file_exist(path: &str) -> bool {
	match fs::metadata(path) {
		Ok(_) => true,
		Err(_) => false,
	}
}

use bcrypt::{hash, verify, DEFAULT_COST};
// bycpt加密
pub fn hash_password(password: &str) -> String {
	bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap_or(String::new())
}
// bycpt验证
pub fn check_password(plain_password: &str, hashed_password: &str) -> bool {
	bcrypt::verify(plain_password, hashed_password).unwrap_or(false)
}

pub fn get_uuid() -> String {
	ulid::Ulid::new().to_string() // 26位
}

// 执行命令
pub fn exec_spawn(cmd: &str) {
	// 新建进程执行
	std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
}

pub fn exec_cmd(cmd: &str) {
	// 当前进程阻塞执行
	// let _ = std::process::Command::new("sh").arg("-c").arg(cmd).status(); // 有输出
	// let _ = std::process::Command::new("sh")
	// 	.arg("-c")
	// 	.arg(cmd)
	// 	.stdout(std::process::Stdio::null())
	// 	.stderr(std::process::Stdio::null())
	// 	.status(); // 静默执行
	let (shell, argument): (&str, &str) = if cfg!(target_os = "windows") { ("cmd", "/C") } else { ("sh", "-c") }; // 跨平台
	let _ = std::process::Command::new(shell)
		.arg(argument)
		.arg(cmd)
		.stdout(std::process::Stdio::null())
		.stderr(std::process::Stdio::null())
		.status(); // 静默执行
}

pub fn exec_cmd_with_output(cmd: &str) -> String {
	let (shell, argument): (&str, &str) = if cfg!(target_os = "windows") { ("cmd", "/C") } else { ("sh", "-c") };
	match std::process::Command::new(shell).arg(argument).arg(cmd).output() {
		Ok(output) => String::from_utf8_lossy(&output.stdout).into_owned(),
		Err(_) => String::new(),
	}
}

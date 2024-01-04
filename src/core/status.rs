use std::any::Any;

use serde::{Deserialize, Serialize};
use sysinfo::{Disks, Networks, System};

use crate::utils; // 系统信息

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemData {
	uptime: u64, // 运行时间
	procs: u32,  // 进程数
	// 负载
	load1: f64,
	load5: f64,
	load15: f64,
	cpu_usage: f32,
	// 内存
	memory_total: u64,
	memory_available: u64,
	memory_used: u64,
	// 缓存
	swap_memory_total: u64,
	swap_memory_available: u64,
	swap_memory_used: u64,
	// 磁盘 - 待实现
	total_space: u64,
	available_space: u64,
	disks: Vec<DiskInfo>,
	// io_read_bytes: u64,
	// io_write_bytes: u64,
	// io_count: u64,
	// io_read_time: u32,
	// io_write_time: u32,
	// disk_data: Vec<DiskData>,
	// 网络
	net_bytes_sent: u64,
	net_bytes_recv: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiskInfo {
	name: String,
	available_space: u64,
	total_space: u64,
	file_system: String,
	file_type: String,
}
pub fn get_system_data() -> SystemData {
	let mut sys = sysinfo::System::new_all();
	sys.refresh_all();
	let uptime = sysinfo::System::uptime();
	let procs = sys.processes().len() as u32;
	let load = sysinfo::System::load_average();
	let load1 = load.one; // 1分钟负载,单位百分比
	let load5 = load.five;
	let load15 = load.fifteen;
	let cpu_usage = sys.global_cpu_info().cpu_usage(); // cpu使用率
												   // 内存信息
	let memory_total = sys.total_memory();
	let memory_available = sys.free_memory();
	let memory_used = sys.used_memory();
	// 缓存
	let swap_memory_total = sys.total_swap();
	let swap_memory_available = sys.free_swap();
	let swap_memory_used = sys.used_swap();
	// 磁盘
	let mut total_space = 0;
	let mut available_space = 0;
	let disks_all = Disks::new_with_refreshed_list();
	// 总信息
	disks_all.iter().for_each(|(v)| {
		// 其他统计
		total_space += v.total_space();
		available_space += v.available_space();
	});
	// 磁盘列表
	let disks = disks_all
		.iter()
		.map(|x| DiskInfo {
			name: x.name().to_string_lossy().into_owned(),
			available_space: x.available_space(),
			total_space: x.total_space(),
			file_system: x.file_system().to_string_lossy().into_owned(),
			file_type: format!("{:?}", x.type_id()),
		})
		.collect::<Vec<_>>();

	// let mut io_read_bytes = 0;
	// let mut io_write_bytes = 0;
	// let mut io_count = 0;
	// let mut io_read_time = 0;
	// let mut io_write_time = 0;
	// // let disks = Disks::new_with_refreshed_list(); // 此处仅给出所有磁盘的挂载信息
	// let process = sys.processes();
	// for p in process {
	// 	p.1.disk_usage();
	// 	io_read_bytes += disk.read_bytes();
	// }
	// 网络
	let networks = Networks::new_with_refreshed_list();
	let net_bytes_sent = networks.iter().map(|(_, v)| v.transmitted()).sum();
	let net_bytes_recv = networks.iter().map(|(_, v)| v.received()).sum();
	SystemData {
		uptime,
		procs,
		load1,
		load5,
		load15,
		cpu_usage,
		memory_total,
		memory_available,
		memory_used,
		swap_memory_total,
		swap_memory_available,
		swap_memory_used,
		total_space,
		available_space,
		disks,
		// io_read_bytes,
		// io_write_bytes,
		// io_count,
		// io_read_time,
		// io_write_time,
		// disk_data,
		net_bytes_sent,
		net_bytes_recv,
	}
}

#[test]
fn test_get_sys() {
	log!("{:?}", get_system_data());
}

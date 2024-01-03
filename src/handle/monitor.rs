use crate::config;
use crate::jwt;
use crate::model;
use crate::utils;
use axum::http::HeaderMap;
use axum::{
	extract::{Path, Query, State},
	response::IntoResponse,
	Json,
};
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use serde_default::serde_default;
use serde_json::json;
use std::sync::Arc;
use sysinfo::{Components, Disks, Networks, Pid, Signal, System, Users};
// 系统信息
// 实时获得系统cup使用率
pub async fn get_cpu() -> impl IntoResponse {
	let sys = sysinfo::System::new_all();
	println!("total memory: {} bytes", sys.total_memory());
	println!("used memory : {} bytes", sys.used_memory());
	println!("total swap  : {} bytes", sys.total_swap());
	println!("used swap   : {} bytes", sys.used_swap());
	// let cpu = cpu.get_processors();
	// let mut cpu_usage = 0.0;
	// for cpu in cpu {
	// 	cpu_usage += cpu.get_cpu_usage();
	// }
	Json(json!({"status":200,"data":0,}))
}

#[test]
fn test_get_cpu() {
	let mut sys = sysinfo::System::new_all();
	sys.refresh_all();
	println!("total memory: {} bytes", sys.total_memory());
	println!("used memory : {} bytes", sys.used_memory());
	println!("total swap  : {} bytes", sys.total_swap());
	println!("used swap   : {} bytes", sys.used_swap());
	// Display system information:
	println!("System name:             {:?}", System::name());
	println!("System kernel version:   {:?}", System::kernel_version());
	println!("System OS version:       {:?}", System::os_version());
	println!("System host name:        {:?}", System::host_name());
	// Number of CPUs:
	println!("NB CPUs: {}", sys.cpus().len());

	// Display processes ID, name na disk usage:
	// for (pid, process) in sys.processes() {
	// 	println!("[{pid}] {} {:?}", process.name(), process.disk_usage());
	//     // [293] sysextd DiskUsage { total_written_bytes: 0, written_bytes: 0, total_read_bytes: 0, read_bytes: 0 }
	// }

	// We display all disks' information:
	// println!("=> disks:");
	// let disks = Disks::new_with_refreshed_list();
	// for disk in &disks {
	// 	log!("{disk:?}");
	//     // Disk("1T")[FS: "apfs"][Type: SSD][removable: yes] mounted on "/": 751377987234/1000203816960 B
	// }

	// Network interfaces name, data received and data transmitted:
	let networks = Networks::new_with_refreshed_list();
	log!("{:?}", networks);
	println!("=> networks:");
	for (interface_name, data) in &networks {
		println!("{interface_name}: {}/{} B", data.received(), data.transmitted());
	}
}

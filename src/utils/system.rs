use std::process::Command;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Usage {
    pub cpu: f32,
    pub ram: f32,
    pub disk: f32,
    pub gpu: f32,
}

pub fn system_usage() -> Usage {
    Usage {
        cpu: cpu_percent(),
        ram: ram_percent(),
        disk: disk_percent(),
        gpu: gpu_percent(),
    }
}

pub fn bash_float(cmd: &str) -> f32 {
    Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse::<f32>().ok())
        .unwrap_or(0.0)
}


pub fn cpu_percent() -> f32 {
    bash_float(r#"awk '/^cpu / { usage=($2+$4)*100/($2+$4+$5) } END { printf "%.2f\n", usage }' /proc/stat"#)
}

pub fn ram_percent() -> f32 {
    bash_float(r#"free | awk '/Mem:/ { printf "%.2f\n", $3*100/$2 }'"#)
}

pub fn disk_percent() -> f32 {
    bash_float(r#"df -P -x tmpfs -x devtmpfs | awk 'NR>1 {used+=$3; total+=$2} END {printf "%.2f\n", used*100/total}'"#)
}

pub fn gpu_percent() -> f32 {
    bash_float("command -v nvidia-smi >/dev/null && nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader,nounits || command -v intel_gpu_top >/dev/null && intel_gpu_top -J -s 1000 | jq '.engines.Render.busy' | head -n1 || command -v radeontop >/dev/null && radeontop -d - -l 1 | awk -F'[, ]+' '/gpu/ {print $2}'")
}

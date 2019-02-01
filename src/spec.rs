// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_json;
use std::collections::HashMap;

pub type ConfigLinux = Option<serde_json::Value>;
pub type ConfigSolaris = Option<serde_json::Value>;
pub type ConfigVm = Option<serde_json::Value>;
pub type ConfigWindows = Option<serde_json::Value>;
pub type DefsLinux = Option<serde_json::Value>;
pub type DefsVm = Option<serde_json::Value>;
pub type DefsWindows = Option<serde_json::Value>;
pub type Defs = Option<serde_json::Value>;

/// Open Container Initiative Runtime Specification Container Configuration Schema
#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct ConfigSchema {
    #[builder(default)]
    pub annotations: Option<HashMap<String, Option<serde_json::Value>>>,
    #[builder(default)]
    pub hooks: Option<Hooks>,
    #[builder(default)]
    pub hostname: Option<String>,
    #[builder(default)]
    pub linux: Option<Linux>,
    #[builder(default)]
    pub mounts: Option<Vec<Mount>>,
    #[serde(rename = "ociVersion")]
    pub oci_version: String,
    #[builder(default)]
    pub process: Option<Process>,
    /// Configures the container's root filesystem.
    #[builder(default)]
    pub root: Option<Root>,
    #[builder(default)]
    pub solaris: Option<Solaris>,
    #[builder(default)]
    pub vm: Option<Vm>,
    #[builder(default)]
    pub windows: Option<Windows>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Hooks {
    #[builder(default)]
    pub poststart: Option<Vec<Hook>>,
    #[builder(default)]
    pub poststop: Option<Vec<Hook>>,
    #[builder(default)]
    pub prestart: Option<Vec<Hook>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Hook {
    #[builder(default)]
    pub args: Option<Vec<String>>,
    #[builder(default)]
    pub env: Option<Vec<String>>,
    pub path: String,
    #[builder(default)]
    pub timeout: Option<i64>,
}

/// Linux platform-specific configurations
#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Linux {
    #[serde(rename = "cgroupsPath")]
    #[builder(default)]
    pub cgroups_path: Option<String>,
    #[builder(default)]
    pub devices: Option<Vec<LinuxDevice>>,
    #[serde(rename = "gidMappings")]
    #[builder(default)]
    pub gid_mappings: Option<Vec<IdMapping>>,
    #[serde(rename = "intelRdt")]
    #[builder(default)]
    pub intel_rdt: Option<IntelRdt>,
    #[serde(rename = "maskedPaths")]
    #[builder(default)]
    pub masked_paths: Option<Vec<String>>,
    #[serde(rename = "mountLabel")]
    #[builder(default)]
    pub mount_label: Option<String>,
    #[builder(default)]
    pub namespaces: Option<Vec<NamespaceReference>>,
    #[serde(rename = "readonlyPaths")]
    #[builder(default)]
    pub readonly_paths: Option<Vec<String>>,
    #[builder(default)]
    pub resources: Option<LinuxResources>,
    #[serde(rename = "rootfsPropagation")]
    #[builder(default)]
    pub rootfs_propagation: Option<RootfsPropagation>,
    #[builder(default)]
    pub seccomp: Option<Seccomp>,
    #[builder(default)]
    pub sysctl: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "uidMappings")]
    #[builder(default)]
    pub uid_mappings: Option<Vec<IdMapping>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct LinuxDevice {
    #[serde(rename = "fileMode")]
    #[builder(default)]
    pub file_mode: Option<i64>,
    #[builder(default)]
    pub gid: Option<i64>,
    #[builder(default)]
    pub major: Option<i64>,
    #[builder(default)]
    pub minor: Option<i64>,
    pub path: String,
    #[serde(rename = "type")]
    pub device_type: String,
    #[builder(default)]
    pub uid: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct IdMapping {
    #[serde(rename = "containerID")]
    pub container_id: i64,
    #[serde(rename = "hostID")]
    pub host_id: i64,
    pub size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct IntelRdt {
    #[serde(rename = "closID")]
    #[builder(default)]
    pub clos_id: Option<String>,
    #[serde(rename = "l3CacheSchema")]
    #[builder(default)]
    pub l3_cache_schema: Option<String>,
    #[serde(rename = "memBwSchema")]
    #[builder(default)]
    pub mem_bw_schema: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder, PartialEq)]
#[builder(setter(into))]
pub struct NamespaceReference {
    #[builder(default)]
    pub path: Option<String>,
    #[serde(rename = "type")]
    pub namespace_reference_type: NamespaceType,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct LinuxResources {
    #[serde(rename = "blockIO")]
    #[builder(default)]
    pub block_io: Option<BlockIo>,
    #[builder(default)]
    pub cpu: Option<PurpleCpu>,
    #[builder(default)]
    pub devices: Option<Vec<DeviceCgroup>>,
    #[serde(rename = "hugepageLimits")]
    #[builder(default)]
    pub hugepage_limits: Option<Vec<HugepageLimit>>,
    #[builder(default)]
    pub memory: Option<PurpleMemory>,
    #[builder(default)]
    pub network: Option<ResourcesNetwork>,
    #[builder(default)]
    pub pids: Option<Pids>,
    #[builder(default)]
    pub rdma: Option<HashMap<String, Rdma>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct BlockIo {
    #[serde(rename = "leafWeight")]
    #[builder(default)]
    pub leaf_weight: Option<i64>,
    #[serde(rename = "throttleReadBpsDevice")]
    #[builder(default)]
    pub throttle_read_bps_device: Option<Vec<BlockIoDeviceThrottle>>,
    #[serde(rename = "throttleReadIOPSDevice")]
    #[builder(default)]
    pub throttle_read_iops_device: Option<Vec<BlockIoDeviceThrottle>>,
    #[serde(rename = "throttleWriteBpsDevice")]
    #[builder(default)]
    pub throttle_write_bps_device: Option<Vec<BlockIoDeviceThrottle>>,
    #[serde(rename = "throttleWriteIOPSDevice")]
    #[builder(default)]
    pub throttle_write_iops_device: Option<Vec<BlockIoDeviceThrottle>>,
    #[builder(default)]
    pub weight: Option<i64>,
    #[serde(rename = "weightDevice")]
    #[builder(default)]
    pub weight_device: Option<Vec<BlockIoDeviceWeight>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct BlockIoDeviceThrottle {
    pub major: i64,
    pub minor: i64,
    #[builder(default)]
    pub rate: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct BlockIoDeviceWeight {
    pub major: i64,
    pub minor: i64,
    #[serde(rename = "leafWeight")]
    #[builder(default)]
    pub leaf_weight: Option<i64>,
    #[builder(default)]
    pub weight: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct PurpleCpu {
    #[builder(default)]
    pub cpus: Option<String>,
    #[builder(default)]
    pub mems: Option<String>,
    #[builder(default)]
    pub period: Option<i64>,
    #[builder(default)]
    pub quota: Option<i64>,
    #[serde(rename = "realtimePeriod")]
    #[builder(default)]
    pub realtime_period: Option<i64>,
    #[serde(rename = "realtimeRuntime")]
    #[builder(default)]
    pub realtime_runtime: Option<i64>,
    #[builder(default)]
    pub shares: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct DeviceCgroup {
    #[builder(default)]
    pub access: Option<String>,
    pub allow: bool,
    #[builder(default)]
    pub major: Option<i64>,
    #[builder(default)]
    pub minor: Option<i64>,
    #[serde(rename = "type")]
    #[builder(default)]
    pub device_cgroup_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct HugepageLimit {
    pub limit: i64,
    #[serde(rename = "pageSize")]
    pub page_size: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct PurpleMemory {
    #[serde(rename = "disableOOMKiller")]
    #[builder(default)]
    pub disable_oom_killer: Option<bool>,
    #[builder(default)]
    pub kernel: Option<i64>,
    #[serde(rename = "kernelTCP")]
    #[builder(default)]
    pub kernel_tcp: Option<i64>,
    #[builder(default)]
    pub limit: Option<i64>,
    #[builder(default)]
    pub reservation: Option<i64>,
    #[builder(default)]
    pub swap: Option<i64>,
    #[builder(default)]
    pub swappiness: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct ResourcesNetwork {
    #[serde(rename = "classID")]
    #[builder(default)]
    pub class_id: Option<i64>,
    #[builder(default)]
    pub priorities: Option<Vec<NetworkInterfacePriority>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct NetworkInterfacePriority {
    pub name: String,
    pub priority: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Pids {
    pub limit: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Rdma {
    #[serde(rename = "hcaHandles")]
    #[builder(default)]
    pub hca_handles: Option<i64>,
    #[serde(rename = "hcaObjects")]
    #[builder(default)]
    pub hca_objects: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Seccomp {
    #[builder(default)]
    pub architectures: Option<Vec<SeccompArch>>,
    #[serde(rename = "defaultAction")]
    pub default_action: SeccompAction,
    #[builder(default)]
    pub syscalls: Option<Vec<Syscall>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Syscall {
    pub action: SeccompAction,
    #[builder(default)]
    pub args: Option<Vec<SyscallArg>>,
    pub names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder, PartialEq)]
#[builder(setter(into))]
pub struct SyscallArg {
    pub index: i64,
    pub op: SeccompOperators,
    pub value: i64,
    #[serde(rename = "valueTwo")]
    #[builder(default)]
    pub value_two: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Mount {
    pub destination: String,
    #[builder(default)]
    pub options: Option<Vec<String>>,
    #[builder(default)]
    pub source: Option<String>,
    #[serde(rename = "type")]
    #[builder(default)]
    pub mount_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Process {
    #[serde(rename = "apparmorProfile")]
    #[builder(default)]
    pub apparmor_profile: Option<String>,
    pub args: Vec<String>,
    #[builder(default)]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "consoleSize")]
    #[builder(default)]
    pub console_size: Option<ConsoleSize>,
    pub cwd: String,
    #[builder(default)]
    pub env: Option<Vec<String>>,
    #[serde(rename = "noNewPrivileges")]
    #[builder(default)]
    pub no_new_privileges: Option<bool>,
    #[serde(rename = "oomScoreAdj")]
    #[builder(default)]
    pub oom_score_adj: Option<i64>,
    #[builder(default)]
    pub rlimits: Option<Vec<Rlimit>>,
    #[serde(rename = "selinuxLabel")]
    #[builder(default)]
    pub selinux_label: Option<String>,
    #[builder(default)]
    pub terminal: Option<bool>,
    #[builder(default)]
    pub user: Option<User>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Capabilities {
    #[builder(default)]
    pub ambient: Option<Vec<String>>,
    #[builder(default)]
    pub bounding: Option<Vec<String>>,
    #[builder(default)]
    pub effective: Option<Vec<String>>,
    #[builder(default)]
    pub inheritable: Option<Vec<String>>,
    #[builder(default)]
    pub permitted: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct ConsoleSize {
    pub height: i64,
    pub width: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Rlimit {
    pub hard: i64,
    pub soft: i64,
    #[serde(rename = "type")]
    pub rlimit_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct User {
    #[serde(rename = "additionalGids")]
    #[builder(default)]
    pub additional_gids: Option<Vec<i64>>,
    #[builder(default)]
    pub gid: Option<i64>,
    #[builder(default)]
    pub uid: Option<i64>,
    #[builder(default)]
    pub username: Option<String>,
}

/// Configures the container's root filesystem.
#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Root {
    pub path: String,
    #[builder(default)]
    pub readonly: Option<bool>,
}

/// Solaris platform-specific configurations
#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Solaris {
    #[builder(default)]
    pub anet: Option<Vec<Anet>>,
    #[serde(rename = "cappedCPU")]
    #[builder(default)]
    pub capped_cpu: Option<CappedCpu>,
    #[serde(rename = "cappedMemory")]
    #[builder(default)]
    pub capped_memory: Option<CappedMemory>,
    #[builder(default)]
    pub limitpriv: Option<String>,
    #[serde(rename = "maxShmMemory")]
    #[builder(default)]
    pub max_shm_memory: Option<String>,
    #[builder(default)]
    pub milestone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Anet {
    #[serde(rename = "allowedAddress")]
    #[builder(default)]
    pub allowed_address: Option<String>,
    #[serde(rename = "configureAllowedAddress")]
    #[builder(default)]
    pub configure_allowed_address: Option<String>,
    #[builder(default)]
    pub defrouter: Option<String>,
    #[builder(default)]
    pub linkname: Option<String>,
    #[serde(rename = "linkProtection")]
    #[builder(default)]
    pub link_protection: Option<String>,
    #[serde(rename = "lowerLink")]
    #[builder(default)]
    pub lower_link: Option<String>,
    #[serde(rename = "macAddress")]
    #[builder(default)]
    pub mac_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct CappedCpu {
    #[builder(default)]
    pub ncpus: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct CappedMemory {
    #[builder(default)]
    pub physical: Option<String>,
    #[builder(default)]
    pub swap: Option<String>,
}

/// configuration for virtual-machine-based containers
#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Vm {
    /// hypervisor config used by VM-based containers
    #[builder(default)]
    pub hypervisor: Option<Hypervisor>,
    /// root image config used by VM-based containers
    #[builder(default)]
    pub image: Option<Image>,
    /// kernel config used by VM-based containers
    pub kernel: Kernel,
}

/// hypervisor config used by VM-based containers
#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Hypervisor {
    #[builder(default)]
    pub parameters: Option<Vec<String>>,
    pub path: String,
}

/// root image config used by VM-based containers
#[derive(Debug, Serialize, Deserialize, Clone, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Image {
    pub format: RootImageFormat,
    pub path: String,
}

/// kernel config used by VM-based containers
#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Kernel {
    #[builder(default)]
    pub initrd: Option<String>,
    #[builder(default)]
    pub parameters: Option<Vec<String>>,
    pub path: String,
}

/// Windows platform-specific configurations
#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Windows {
    #[serde(rename = "credentialSpec")]
    #[builder(default)]
    pub credential_spec: Option<HashMap<String, Option<serde_json::Value>>>,
    #[builder(default)]
    pub devices: Option<Vec<WindowsDevice>>,
    #[builder(default)]
    pub hyperv: Option<Hyperv>,
    #[serde(rename = "ignoreFlushesDuringBoot")]
    #[builder(default)]
    pub ignore_flushes_during_boot: Option<bool>,
    #[serde(rename = "layerFolders")]
    pub layer_folders: Vec<String>,
    #[builder(default)]
    pub network: Option<WindowsNetwork>,
    #[builder(default)]
    pub resources: Option<WindowsResources>,
    #[builder(default)]
    pub servicing: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder, PartialEq)]
#[builder(setter(into))]
pub struct WindowsDevice {
    pub id: String,
    #[serde(rename = "idType")]
    pub id_type: IdType,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Hyperv {
    #[serde(rename = "utilityVMPath")]
    #[builder(default)]
    pub utility_vm_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct WindowsNetwork {
    #[serde(rename = "allowUnqualifiedDNSQuery")]
    #[builder(default)]
    pub allow_unqualified_dns_query: Option<bool>,
    #[serde(rename = "DNSSearchList")]
    #[builder(default)]
    pub dns_search_list: Option<Vec<String>>,
    #[serde(rename = "endpointList")]
    #[builder(default)]
    pub endpoint_list: Option<Vec<String>>,
    #[serde(rename = "networkNamespace")]
    #[builder(default)]
    pub network_namespace: Option<String>,
    #[serde(rename = "networkSharedContainerName")]
    #[builder(default)]
    pub network_shared_container_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct WindowsResources {
    #[builder(default)]
    pub cpu: Option<FluffyCpu>,
    #[builder(default)]
    pub memory: Option<FluffyMemory>,
    #[builder(default)]
    pub storage: Option<Storage>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct FluffyCpu {
    #[builder(default)]
    pub count: Option<i64>,
    #[builder(default)]
    pub maximum: Option<i64>,
    #[builder(default)]
    pub shares: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct FluffyMemory {
    #[builder(default)]
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(setter(into))]
pub struct Storage {
    #[builder(default)]
    pub bps: Option<i64>,
    #[builder(default)]
    pub iops: Option<i64>,
    #[serde(rename = "sandboxSize")]
    #[builder(default)]
    pub sandbox_size: Option<i64>,
}

/// Open Container Runtime State Schema
#[derive(Debug, Serialize, Deserialize, Clone, Builder, PartialEq)]
#[builder(setter(into))]
pub struct StateSchema {
    #[builder(default)]
    pub annotations: Option<HashMap<String, Option<serde_json::Value>>>,
    pub bundle: String,
    /// the container's ID
    pub id: String,
    #[serde(rename = "ociVersion")]
    pub oci_version: String,
    #[builder(default)]
    pub pid: Option<i64>,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum NamespaceType {
    #[serde(rename = "cgroup")]
    Cgroup,
    #[serde(rename = "ipc")]
    Ipc,
    #[serde(rename = "mount")]
    Mount,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "pid")]
    Pid,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "uts")]
    Uts,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RootfsPropagation {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "slave")]
    Slave,
    #[serde(rename = "unbindable")]
    Unbindable,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SeccompArch {
    #[serde(rename = "SCMP_ARCH_AARCH64")]
    ScmpArchAarch64,
    #[serde(rename = "SCMP_ARCH_ARM")]
    ScmpArchArm,
    #[serde(rename = "SCMP_ARCH_MIPS")]
    ScmpArchMips,
    #[serde(rename = "SCMP_ARCH_MIPS64")]
    ScmpArchMips64,
    #[serde(rename = "SCMP_ARCH_MIPS64N32")]
    ScmpArchMips64N32,
    #[serde(rename = "SCMP_ARCH_MIPSEL")]
    ScmpArchMipsel,
    #[serde(rename = "SCMP_ARCH_MIPSEL64")]
    ScmpArchMipsel64,
    #[serde(rename = "SCMP_ARCH_MIPSEL64N32")]
    ScmpArchMipsel64N32,
    #[serde(rename = "SCMP_ARCH_PARISC")]
    ScmpArchParisc,
    #[serde(rename = "SCMP_ARCH_PARISC64")]
    ScmpArchParisc64,
    #[serde(rename = "SCMP_ARCH_PPC")]
    ScmpArchPpc,
    #[serde(rename = "SCMP_ARCH_PPC64")]
    ScmpArchPpc64,
    #[serde(rename = "SCMP_ARCH_PPC64LE")]
    ScmpArchPpc64Le,
    #[serde(rename = "SCMP_ARCH_S390")]
    ScmpArchS390,
    #[serde(rename = "SCMP_ARCH_S390X")]
    ScmpArchS390X,
    #[serde(rename = "SCMP_ARCH_X32")]
    ScmpArchX32,
    #[serde(rename = "SCMP_ARCH_X86")]
    ScmpArchX86,
    #[serde(rename = "SCMP_ARCH_X86_64")]
    ScmpArchX8664,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SeccompAction {
    #[serde(rename = "SCMP_ACT_ALLOW")]
    ScmpActAllow,
    #[serde(rename = "SCMP_ACT_ERRNO")]
    ScmpActErrno,
    #[serde(rename = "SCMP_ACT_KILL")]
    ScmpActKill,
    #[serde(rename = "SCMP_ACT_TRACE")]
    ScmpActTrace,
    #[serde(rename = "SCMP_ACT_TRAP")]
    ScmpActTrap,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SeccompOperators {
    #[serde(rename = "SCMP_CMP_EQ")]
    ScmpCmpEq,
    #[serde(rename = "SCMP_CMP_GE")]
    ScmpCmpGe,
    #[serde(rename = "SCMP_CMP_GT")]
    ScmpCmpGt,
    #[serde(rename = "SCMP_CMP_LE")]
    ScmpCmpLe,
    #[serde(rename = "SCMP_CMP_LT")]
    ScmpCmpLt,
    #[serde(rename = "SCMP_CMP_MASKED_EQ")]
    ScmpCmpMaskedEq,
    #[serde(rename = "SCMP_CMP_NE")]
    ScmpCmpNe,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RootImageFormat {
    #[serde(rename = "qcow2")]
    Qcow2,
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "vdi")]
    Vdi,
    #[serde(rename = "vhd")]
    Vhd,
    #[serde(rename = "vmdk")]
    Vmdk,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum IdType {
    #[serde(rename = "class")]
    Class,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Status {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopped")]
    Stopped,
}

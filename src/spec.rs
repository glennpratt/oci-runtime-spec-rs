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
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConfigSchema {
    pub annotations: Option<HashMap<String, Option<serde_json::Value>>>,
    pub hooks: Option<Hooks>,
    pub hostname: Option<String>,
    pub linux: Option<Linux>,
    pub mounts: Option<Vec<Mount>>,
    #[serde(rename = "ociVersion")]
    pub oci_version: String,
    pub process: Option<Process>,
    /// Configures the container's root filesystem.
    pub root: Option<Root>,
    pub solaris: Option<Solaris>,
    pub vm: Option<Vm>,
    pub windows: Option<Windows>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Hooks {
    pub poststart: Option<Vec<Hook>>,
    pub poststop: Option<Vec<Hook>>,
    pub prestart: Option<Vec<Hook>>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Hook {
    pub args: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub path: String,
    pub timeout: Option<i64>,
}

/// Linux platform-specific configurations
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Linux {
    #[serde(rename = "cgroupsPath")]
    pub cgroups_path: Option<String>,
    pub devices: Option<Vec<LinuxDevice>>,
    #[serde(rename = "gidMappings")]
    pub gid_mappings: Option<Vec<IdMapping>>,
    #[serde(rename = "intelRdt")]
    pub intel_rdt: Option<IntelRdt>,
    #[serde(rename = "maskedPaths")]
    pub masked_paths: Option<Vec<String>>,
    #[serde(rename = "mountLabel")]
    pub mount_label: Option<String>,
    pub namespaces: Option<Vec<NamespaceReference>>,
    #[serde(rename = "readonlyPaths")]
    pub readonly_paths: Option<Vec<String>>,
    pub resources: Option<LinuxResources>,
    #[serde(rename = "rootfsPropagation")]
    pub rootfs_propagation: Option<RootfsPropagation>,
    pub seccomp: Option<Seccomp>,
    pub sysctl: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "uidMappings")]
    pub uid_mappings: Option<Vec<IdMapping>>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct LinuxDevice {
    #[serde(rename = "fileMode")]
    pub file_mode: Option<i64>,
    pub gid: Option<i64>,
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub path: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub uid: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct IdMapping {
    #[serde(rename = "containerID")]
    pub container_id: i64,
    #[serde(rename = "hostID")]
    pub host_id: i64,
    pub size: i64,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct IntelRdt {
    #[serde(rename = "closID")]
    pub clos_id: Option<String>,
    #[serde(rename = "l3CacheSchema")]
    pub l3_cache_schema: Option<String>,
    #[serde(rename = "memBwSchema")]
    pub mem_bw_schema: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NamespaceReference {
    pub path: Option<String>,
    #[serde(rename = "type")]
    pub namespace_reference_type: NamespaceType,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct LinuxResources {
    #[serde(rename = "blockIO")]
    pub block_io: Option<BlockIo>,
    pub cpu: Option<PurpleCpu>,
    pub devices: Option<Vec<DeviceCgroup>>,
    #[serde(rename = "hugepageLimits")]
    pub hugepage_limits: Option<Vec<HugepageLimit>>,
    pub memory: Option<PurpleMemory>,
    pub network: Option<ResourcesNetwork>,
    pub pids: Option<Pids>,
    pub rdma: Option<HashMap<String, Rdma>>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct BlockIo {
    #[serde(rename = "leafWeight")]
    pub leaf_weight: Option<i64>,
    #[serde(rename = "throttleReadBpsDevice")]
    pub throttle_read_bps_device: Option<Vec<BlockIoDeviceThrottle>>,
    #[serde(rename = "throttleReadIOPSDevice")]
    pub throttle_read_iops_device: Option<Vec<BlockIoDeviceThrottle>>,
    #[serde(rename = "throttleWriteBpsDevice")]
    pub throttle_write_bps_device: Option<Vec<BlockIoDeviceThrottle>>,
    #[serde(rename = "throttleWriteIOPSDevice")]
    pub throttle_write_iops_device: Option<Vec<BlockIoDeviceThrottle>>,
    pub weight: Option<i64>,
    #[serde(rename = "weightDevice")]
    pub weight_device: Option<Vec<BlockIoDeviceWeight>>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct BlockIoDeviceThrottle {
    pub major: i64,
    pub minor: i64,
    pub rate: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct BlockIoDeviceWeight {
    pub major: i64,
    pub minor: i64,
    #[serde(rename = "leafWeight")]
    pub leaf_weight: Option<i64>,
    pub weight: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PurpleCpu {
    pub cpus: Option<String>,
    pub mems: Option<String>,
    pub period: Option<i64>,
    pub quota: Option<i64>,
    #[serde(rename = "realtimePeriod")]
    pub realtime_period: Option<i64>,
    #[serde(rename = "realtimeRuntime")]
    pub realtime_runtime: Option<i64>,
    pub shares: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct DeviceCgroup {
    pub access: Option<String>,
    pub allow: bool,
    pub major: Option<i64>,
    pub minor: Option<i64>,
    #[serde(rename = "type")]
    pub device_cgroup_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct HugepageLimit {
    pub limit: i64,
    #[serde(rename = "pageSize")]
    pub page_size: String,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PurpleMemory {
    #[serde(rename = "disableOOMKiller")]
    pub disable_oom_killer: Option<bool>,
    pub kernel: Option<i64>,
    #[serde(rename = "kernelTCP")]
    pub kernel_tcp: Option<i64>,
    pub limit: Option<i64>,
    pub reservation: Option<i64>,
    pub swap: Option<i64>,
    pub swappiness: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ResourcesNetwork {
    #[serde(rename = "classID")]
    pub class_id: Option<i64>,
    pub priorities: Option<Vec<NetworkInterfacePriority>>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct NetworkInterfacePriority {
    pub name: String,
    pub priority: i64,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Pids {
    pub limit: i64,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Rdma {
    #[serde(rename = "hcaHandles")]
    pub hca_handles: Option<i64>,
    #[serde(rename = "hcaObjects")]
    pub hca_objects: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Seccomp {
    pub architectures: Option<Vec<SeccompArch>>,
    #[serde(rename = "defaultAction")]
    pub default_action: SeccompAction,
    pub syscalls: Option<Vec<Syscall>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Syscall {
    pub action: SeccompAction,
    pub args: Option<Vec<SyscallArg>>,
    pub names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SyscallArg {
    pub index: i64,
    pub op: SeccompOperators,
    pub value: i64,
    #[serde(rename = "valueTwo")]
    pub value_two: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Mount {
    pub destination: String,
    pub options: Option<Vec<String>>,
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub mount_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Process {
    #[serde(rename = "apparmorProfile")]
    pub apparmor_profile: Option<String>,
    pub args: Vec<String>,
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "consoleSize")]
    pub console_size: Option<ConsoleSize>,
    pub cwd: String,
    pub env: Option<Vec<String>>,
    #[serde(rename = "noNewPrivileges")]
    pub no_new_privileges: Option<bool>,
    #[serde(rename = "oomScoreAdj")]
    pub oom_score_adj: Option<i64>,
    pub rlimits: Option<Vec<Rlimit>>,
    #[serde(rename = "selinuxLabel")]
    pub selinux_label: Option<String>,
    pub terminal: Option<bool>,
    pub user: Option<User>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Capabilities {
    pub ambient: Option<Vec<String>>,
    pub bounding: Option<Vec<String>>,
    pub effective: Option<Vec<String>>,
    pub inheritable: Option<Vec<String>>,
    pub permitted: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConsoleSize {
    pub height: i64,
    pub width: i64,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Rlimit {
    pub hard: i64,
    pub soft: i64,
    #[serde(rename = "type")]
    pub rlimit_type: String,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct User {
    #[serde(rename = "additionalGids")]
    pub additional_gids: Option<Vec<i64>>,
    pub gid: Option<i64>,
    pub uid: Option<i64>,
    pub username: Option<String>,
}

/// Configures the container's root filesystem.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Root {
    pub path: String,
    pub readonly: Option<bool>,
}

/// Solaris platform-specific configurations
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Solaris {
    pub anet: Option<Vec<Anet>>,
    #[serde(rename = "cappedCPU")]
    pub capped_cpu: Option<CappedCpu>,
    #[serde(rename = "cappedMemory")]
    pub capped_memory: Option<CappedMemory>,
    pub limitpriv: Option<String>,
    #[serde(rename = "maxShmMemory")]
    pub max_shm_memory: Option<String>,
    pub milestone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Anet {
    #[serde(rename = "allowedAddress")]
    pub allowed_address: Option<String>,
    #[serde(rename = "configureAllowedAddress")]
    pub configure_allowed_address: Option<String>,
    pub defrouter: Option<String>,
    pub linkname: Option<String>,
    #[serde(rename = "linkProtection")]
    pub link_protection: Option<String>,
    #[serde(rename = "lowerLink")]
    pub lower_link: Option<String>,
    #[serde(rename = "macAddress")]
    pub mac_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct CappedCpu {
    pub ncpus: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct CappedMemory {
    pub physical: Option<String>,
    pub swap: Option<String>,
}

/// configuration for virtual-machine-based containers
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Vm {
    /// hypervisor config used by VM-based containers
    pub hypervisor: Option<Hypervisor>,
    /// root image config used by VM-based containers
    pub image: Option<Image>,
    /// kernel config used by VM-based containers
    pub kernel: Kernel,
}

/// hypervisor config used by VM-based containers
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Hypervisor {
    pub parameters: Option<Vec<String>>,
    pub path: String,
}

/// root image config used by VM-based containers
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Image {
    pub format: RootImageFormat,
    pub path: String,
}

/// kernel config used by VM-based containers
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Kernel {
    pub initrd: Option<String>,
    pub parameters: Option<Vec<String>>,
    pub path: String,
}

/// Windows platform-specific configurations
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Windows {
    #[serde(rename = "credentialSpec")]
    pub credential_spec: Option<HashMap<String, Option<serde_json::Value>>>,
    pub devices: Option<Vec<WindowsDevice>>,
    pub hyperv: Option<Hyperv>,
    #[serde(rename = "ignoreFlushesDuringBoot")]
    pub ignore_flushes_during_boot: Option<bool>,
    #[serde(rename = "layerFolders")]
    pub layer_folders: Vec<String>,
    pub network: Option<WindowsNetwork>,
    pub resources: Option<WindowsResources>,
    pub servicing: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WindowsDevice {
    pub id: String,
    #[serde(rename = "idType")]
    pub id_type: IdType,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Hyperv {
    #[serde(rename = "utilityVMPath")]
    pub utility_vm_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct WindowsNetwork {
    #[serde(rename = "allowUnqualifiedDNSQuery")]
    pub allow_unqualified_dns_query: Option<bool>,
    #[serde(rename = "DNSSearchList")]
    pub dns_search_list: Option<Vec<String>>,
    #[serde(rename = "endpointList")]
    pub endpoint_list: Option<Vec<String>>,
    #[serde(rename = "networkNamespace")]
    pub network_namespace: Option<String>,
    #[serde(rename = "networkSharedContainerName")]
    pub network_shared_container_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct WindowsResources {
    pub cpu: Option<FluffyCpu>,
    pub memory: Option<FluffyMemory>,
    pub storage: Option<Storage>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FluffyCpu {
    pub count: Option<i64>,
    pub maximum: Option<i64>,
    pub shares: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FluffyMemory {
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Storage {
    pub bps: Option<i64>,
    pub iops: Option<i64>,
    #[serde(rename = "sandboxSize")]
    pub sandbox_size: Option<i64>,
}

/// Open Container Runtime State Schema
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StateSchema {
    pub annotations: Option<HashMap<String, Option<serde_json::Value>>>,
    pub bundle: String,
    /// the container's ID
    pub id: String,
    #[serde(rename = "ociVersion")]
    pub oci_version: String,
    pub pid: Option<i64>,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum IdType {
    #[serde(rename = "class")]
    Class,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

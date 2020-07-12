use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Seccomp {
    // required
    #[serde(alias = "defaultAction")]
    default_action: String,

    // optional
    #[serde(default)]
    architectures: Vec<String>,

    #[serde(default)]
    flag: String,

    #[serde(default)]
    syscalls: Vec<Syscall>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Syscall {
    // required
    names: Vec<String>,
    action: String,

    // optional
    #[serde(default, alias = "errnoRet")]
    errno_ret: u32,

    #[serde(default)]
    args: Vec<Args>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Args {
    // required
    index: u64,
    value: u64,
    op: String,

    // optional
    #[serde(default, alias = "valueTwo")]
    value_two: u64,
}

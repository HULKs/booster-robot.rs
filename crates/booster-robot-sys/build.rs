fn main() {
    let mut bridge = cxx_build::bridges([
        "src/robot.rs",
        "src/robot/b1.rs",
        "src/robot/b1/api_const.rs",
        "src/robot/common.rs",
    ]);

    bridge
        .cpp(true)
        .std("c++17")
        .flag("-Wno-reorder") // Suppress warnings about field ordering we can't fix
        .include("include")
        .include("booster_robotics_sdk/include")
        .include("booster_robotics_sdk/third_party/include")
        .compile("wrapper");

    println!("cargo:rerun-if-changed=include/wrapper.hpp");

    println!("cargo::rustc-link-search=native=booster_robotics_sdk/lib/x86_64");
    println!("cargo::rustc-link-search=native=booster_robotics_sdk/third_party/lib/x86_64");
    println!("cargo::rustc-link-lib=static=booster_robotics_sdk");
    println!("cargo::rustc-link-lib=static=foonathan_memory-0.7.3");
    println!("cargo::rustc-link-lib=fastcdr");
    println!("cargo::rustc-link-lib=fastrtps");
}

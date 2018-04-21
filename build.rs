extern crate gcc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::fs::File;
use std::fs::copy;

struct BlueantService {
    service_name: &'static str,
    wsdl_url: &'static str,
}

fn main() {
    let destination_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let services_to_build = [
        BlueantService {
            service_name: "baseService",
            wsdl_url: "https://blueant.sinnerschrader.com/blueant/services/BaseService?wsdl",
        },
        BlueantService {
            service_name: "worktimeAccountingService",
            wsdl_url: "https://blueant.sinnerschrader.com/blueant/services/WorktimeAccountingService?wsdl",
        },
    ];

    generate_gsoap_code(&services_to_build, &destination_path);
    compile_soap_lib(&services_to_build, &destination_path);
    generate_rust_bindings(&services_to_build, &destination_path);
}

fn generate_gsoap_code(services_to_build: &[BlueantService], destination_path: &PathBuf) {
    for service in services_to_build {
        if !destination_path
        .join(service.service_name.to_owned() + ".h")
        .as_path()
        .exists()
        {
            println!("running wsdl2h: {}", service.service_name.to_owned() + ".h");
            Command::new("wsdl2h")
                .args(&[
                    "-n",
                    service.service_name,
                    "-c",
                    "-o",
                    &destination_path
                        .join(service.service_name.to_owned() + ".h")
                        .to_str()
                        .unwrap(),
                    service.wsdl_url,
                ])
                .status()
                .expect("Could not generate header for WSDL");
        }

        if !destination_path
        .join(service.service_name.to_owned() + "ClientLib.c")
        .as_path()
        .exists()
        {
            println!("running soapcpp2: {}", service.service_name.to_owned() + "ClientLib.c");
            Command::new("soapcpp2")
                .args(&[
                    "-p",
                    service.service_name,
                    "-nwxC",
                    "-d",
                    &destination_path.to_str().unwrap(),
                    &destination_path
                        .join(service.service_name.to_owned() + ".h")
                        .to_str()
                        .unwrap(),
                ])
                .status()
                .expect("Could not generate code for header");
        }
    }

    //Create empty header file for gsoap general code (serialize etc.)
    File::create(&destination_path.join("env.h")).expect("Could nor create env.h file");

    Command::new("soapcpp2")
        .args(&[
            "-penv",
            "-c",
            "-d",
            &destination_path.to_str().unwrap(),
            &destination_path.join("env.h").to_str().unwrap(),
        ])
        .status()
        .expect("Could not generate code for env header");
}

fn compile_soap_lib(services_to_build: &[BlueantService], destination_path: &PathBuf) {
    println!("Compile stuff"); 
    copy(
        PathBuf::from("blueant-soap-c/main.c"),
        &destination_path.join("main.c"),
    ).expect("Could not copy main.c to build dir");

    println!("cargo:rustc-link-lib=gsoapssl");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=z");

    let mut services_to_compile = vec![destination_path.join("main.c"), destination_path.join("envC.c")];

    for service in services_to_build {
        services_to_compile.push(destination_path.join(service.service_name.to_owned() + "ClientLib.c"));
    }

    gcc::Build::new()
        .files(services_to_compile.as_slice())
        .flag("-Wno-unused-function")
        .shared_flag(true)
        .compile("blueant");
}

fn generate_rust_bindings(services_to_build: &[BlueantService], destination_path: &PathBuf) {
    for service in services_to_build {
        if !destination_path
            .join(&destination_path.join(service.service_name.to_owned() + "binding.rs"))
            .as_path()
            .exists()
            {
            let bindings = bindgen::Builder::default()
                .header(
                    destination_path.join(service.service_name.to_owned() + "H.h")
                        .to_str()
                        .unwrap()
                        .clone(),
                )
                .layout_tests(false)
                .blacklist_type("_bindgen_ty_2")
                .blacklist_type("_bindgen_ty_8")
                .generate()
                .expect("Unable to generate bindings");

            bindings
                .write_to_file(&destination_path.join(service.service_name.to_owned() + "binding.rs"))
                .expect("Couldn't write bindings!");
        }
    }

    //Build one binding file with every binding in it

    // let bindings = bindgen::Builder::default();

    // let generated_bindings = bindings.blacklist_type("_bindgen_ty_2")
    //     .header(destination_path.join("baseServiceH.h").to_str().unwrap().clone())
    //     .header(destination_path.join("worktimeAccountingServiceH.h").to_str().unwrap().clone())
    //     .blacklist_type("_bindgen_ty_8")
    //     .layout_tests(false)
    //     .generate()
    //     .expect("Unable to generate bindings");

    // generated_bindings
    //     .write_to_file(&destination_path.join("binding.rs"))
    //     .expect("Couldn't write bindings!");
}
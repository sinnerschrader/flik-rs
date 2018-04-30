## Build SOAP Client with gsoap

Create header file for client:

`wsdl2h -c -o baseService.h https://blueant.sinnerschrader.com/blueant/services/BaseService\?wsdl`

Build C code from header:

`soapcpp2 -cC baseService.h`

Compile library files:

`cc -c  soapC.c soapClientLib.c`

Create create libary with the name libsoap.a:

`ar ruv libsoap.a *.o`

## Create C Bindings with bindgen

Generate bindings: 
* cd src/gsoap
* bindgen --blacklist-type _bindgen_ty_2 --blacklist-type_bindgen_ty_8 soapH.h > ../binding.rs

## Dependecies for building

* Use rust nightly -> `rustup toolchain install nightly && rustup default nightly`
* Install rustfmt-preview (for bindgen) -> `rustup component add rustfmt-preview`
* Install gsoap (Works on gsoap-2.8-28-1) -> Manjaro only has 2.8.61-1 this seems to remove a dummy attribute in the logout response -.-
* Under `target/debug/build/flik-*/out` you can find the c files and bindings.

Tested under Windows Linux Subsystem with Ubuntu. Under Manjaro the linkink seems to be not correct and the build fails .....

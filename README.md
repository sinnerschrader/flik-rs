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

How to install: 

* cargo install rusfmt-nightly
* cargo install bindgen

Generate bindings: 
* cd src/gsoap
* bindgen --blacklist-type _bindgen_ty_2 --blacklist-type_bindgen_ty_8 soapH.h > ../binding.rs
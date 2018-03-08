## Build SOAP Client with gsoap

Create header file for client:

`wsdl2h -c -o baseService.h https://blueant.sinnerschrader.com/blueant/services/BaseService\?wsdl`

Build C code from header:

`soapcpp2 -cC baseService.h`

Compile library files:

`cc -c  soapC.c soapClientLib.c`

Create create libary with the name libsoap.a:

`ar ruv libsoap.a *.o`
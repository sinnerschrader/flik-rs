blueant:	main.o soapBaseBindingProxy.o soapC.o
	cc -o blueant main.o soapBaseBindingProxy.o soapC.o -lstdc++ -lgsoapssl++ -lssl -lcrypto -lz

%.o: %.cpp %.h blueant.h
	cc -o $@ -c $*.cpp

blueant.h: 
	wsdl2h -o blueant.h https://blueant.sinnerschrader.com/blueant/services/BaseService?wsdl
	
soapBaseBindingProxy.cpp: blueant.h
	soapcpp2 -j -CL blueant.h
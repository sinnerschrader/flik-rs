ARCH=$(shell uname -s)

$(ARCH)/libblueant.dylib: $(ARCH)/blueant
	g++ -dynamiclib -o $(ARCH)/libblueant.dylib $(ARCH)/soapBaseBindingProxy.o $(ARCH)/soapC.o $(ARCH)/main.o -lstdc++ -lgsoapssl++ -lssl -lcrypto -lz

$(ARCH)/blueant: $(ARCH) $(ARCH)/soapBaseBindingProxy.o $(ARCH)/soapC.o $(ARCH)/main.o 
	cc -o $(ARCH)/blueant $(ARCH)/main.o $(ARCH)/soapBaseBindingProxy.o $(ARCH)/soapC.o -lstdc++ -lgsoapssl++ -lssl -lcrypto -lz

$(ARCH)/main.o: main.cpp
	cc -I$(ARCH) -o $@ -c main.cpp

%.o: %.cpp
	cc -o $@ -c $*.cpp
	
$(ARCH)/soapBaseBindingProxy.o: $(ARCH)/soapBaseBindingProxy.cpp
	cc -o $@ -c $*.cpp
	
$(ARCH)/soapBaseBindingProxy.cpp: $(ARCH)/blueant.h
	cd $(ARCH) && soapcpp2 -j -CL blueant.h

$(ARCH)/blueant.h: 
	cd $(ARCH) && wsdl2h -o blueant.h https://blueant.sinnerschrader.com/blueant/services/BaseService?wsdl

$(ARCH):
	mkdir -p $(ARCH)

clean:
	rm -rf $(ARCH)

ARCH=$(shell uname -s)
CFLAGS=-fpic

$(ARCH)/libblueant.dylib: $(ARCH)/blueant
	cc $(CFLAGS) -shared -o $(ARCH)/libblueant.so $(ARCH)/soapClient.o $(ARCH)/soapC.o $(ARCH)/main.o -lgsoapssl -lssl -lcrypto -lz
	#g++ -dynamiclib -o $(ARCH)/libblueant.dylib $(ARCH)/soapClient.o $(ARCH)/soapC.o $(ARCH)/main.o -lgsoapssl -lssl -lcrypto -lz

$(ARCH)/blueant: $(ARCH) $(ARCH)/soapClient.o $(ARCH)/soapC.o $(ARCH)/main.o 
	cc $(CFLAGS) -o $(ARCH)/blueant $(ARCH)/main.o $(ARCH)/soapClient.o $(ARCH)/soapC.o -lgsoapssl -lssl -lcrypto -lz

$(ARCH)/main.o: main.c
	cc $(CFLAGS) -I$(ARCH) -o $@ -c main.c

%.o: %.c
	cc $(CFLAGS) -o $@ -c $*.c
	
$(ARCH)/soapClient.o: $(ARCH)/soapClient.c
	cc $(CFLAGS) -o $@ -c $*.c
	
$(ARCH)/soapClient.c: $(ARCH)/blueant.h
	cd $(ARCH) && soapcpp2 -CL blueant.h

$(ARCH)/blueant.h: 
	cd $(ARCH) && wsdl2h -c -o blueant.h https://blueant.sinnerschrader.com/blueant/services/BaseService?wsdl

$(ARCH):
	mkdir -p $(ARCH)

clean:
	rm -rf $(ARCH)

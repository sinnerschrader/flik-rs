ARCH=$(shell uname -s)
CFLAGS=-fpic

$(ARCH)/libblueant.dylib: $(ARCH)/blueant
	cc $(CFLAGS) -shared -o $(ARCH)/libblueant.so $(ARCH)/workTimeAccountingServiceClient.o $(ARCH)/baseServiceClient.o $(ARCH)/baseServiceC.o $(ARCH)/workTimeAccountingServiceC.o $(ARCH)/main.o -lgsoapssl -lssl -lcrypto -lz
	#g++ -dynamiclib -o $(ARCH)/libblueant.dylib $(ARCH)/soapClient.o $(ARCH)/soapC.o $(ARCH)/main.o -lgsoapssl -lssl -lcrypto -lz

$(ARCH)/blueant: $(ARCH) $(ARCH)/workTimeAccountingServiceClient.o $(ARCH)/baseServiceClient.o $(ARCH)/main.o $(ARCH)/baseServiceC.o $(ARCH)/workTimeAccountingServiceC.o
	cc $(CFLAGS) -o $(ARCH)/blueant $(ARCH)/main.o $(ARCH)/workTimeAccountingServiceClient.o $(ARCH)/baseServiceClient.o $(ARCH)/baseServiceC.o $(ARCH)/workTimeAccountingServiceC.o -lgsoapssl -lssl -lcrypto -lz

$(ARCH)/main.o: main.c
	cc $(CFLAGS) -I$(ARCH) -o $@ -c main.c

%.o: %.c
	cc $(CFLAGS) -o $@ -c $*.c
	
$(ARCH)/workTimeAccountingServiceClient.o: $(ARCH)/workTimeAccountingServiceClient.c
	cc $(CFLAGS) -o $@ -c $*.c
	
$(ARCH)/baseServiceClient.o: $(ARCH)/baseServiceClient.c
	cc $(CFLAGS) -o $@ -c $*.c

$(ARCH)/baseServiceClient.c: $(ARCH)/blueantBaseService.h
	cd $(ARCH) && soapcpp2 -pbaseService -CL blueantBaseService.h

$(ARCH)/workTimeAccountingServiceClient.c: $(ARCH)/blueantWorkTimeAccountingService.h
	cd $(ARCH) && soapcpp2 -pworkTimeAccountingService -CL blueantWorkTimeAccountingService.h

$(ARCH)/blueantBaseService.h: 
	cd $(ARCH) && wsdl2h -c -o blueantBaseService.h https://blueant.sinnerschrader.com/blueant/services/BaseService?wsdl

$(ARCH)/blueantWorkTimeAccountingService.h: 
	cd $(ARCH) && wsdl2h -c -o blueantWorkTimeAccountingService.h https://blueant.sinnerschrader.com/blueant/services/WorktimeAccountingService?wsdl

$(ARCH):
	mkdir -p $(ARCH)

clean:
	rm -rf $(ARCH)

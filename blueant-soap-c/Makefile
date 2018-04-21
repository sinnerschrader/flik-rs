ARCH=$(shell uname -s)
CFLAGS=-fpic

$(ARCH)/libblueant.dylib: $(ARCH)/blueant
	cc $(CFLAGS) -shared -o $(ARCH)/libblueant.so $(ARCH)/workTimeAccountingServiceClientLib.o $(ARCH)/baseServiceClientLib.o $(ARCH)/envC.o $(ARCH)/main.o -lgsoapssl -lssl -lcrypto -lz
	#g++ -dynamiclib -o $(ARCH)/libblueant.dylib $(ARCH)/soapClient.o $(ARCH)/soapC.o $(ARCH)/main.o -lgsoapssl -lssl -lcrypto -lz

$(ARCH)/blueant: $(ARCH) $(ARCH)/envC.o $(ARCH)/workTimeAccountingServiceClientLib.o $(ARCH)/baseServiceClientLib.o $(ARCH)/main.o 
	cc $(CFLAGS) -o $(ARCH)/blueant $(ARCH)/main.o $(ARCH)/workTimeAccountingServiceClientLib.o $(ARCH)/baseServiceClientLib.o $(ARCH)/envC.o -lgsoapssl -lssl -lcrypto -lz

$(ARCH)/main.o: main.c
	echo "Do main"
	cc $(CFLAGS) -I$(ARCH) -o $@ -c main.c

%.o: %.c
	echo "Do all"
	cc $(CFLAGS) -o $@ -c $*.c

$(ARCH)/envC.o: $(ARCH)/envC.c
	echo "Do env"
	cc $(CFLAGS) -o $@ -c $*.c

$(ARCH)/workTimeAccountingServiceClientLib.o: $(ARCH)/workTimeAccountingServiceClientLib.c
	echo "Do worktimeservice soap"
	cc $(CFLAGS) -o $@ -c $*.c
	
$(ARCH)/baseServiceClientLib.o: $(ARCH)/baseServiceClientLib.c
	echo "Do base"
	cc $(CFLAGS) -o $@ -c $*.c

$(ARCH)/envC.c: $(ARCH)/env.h
	cd $(ARCH) && soapcpp2 -c -penv env.h

$(ARCH)/baseServiceClientLib.c: $(ARCH)/blueantBaseService.h
	cd $(ARCH) && soapcpp2 -pbaseService -nwxC blueantBaseService.h

$(ARCH)/workTimeAccountingServiceClientLib.c: $(ARCH)/blueantWorkTimeAccountingService.h
	cd $(ARCH) && soapcpp2 -pworkTimeAccountingService -nwxC blueantWorkTimeAccountingService.h

$(ARCH)/env.h: 
	cd $(ARCH) && touch env.h

$(ARCH)/blueantBaseService.h: 
	cd $(ARCH) && wsdl2h -nbaseService -c -o blueantBaseService.h https://blueant.sinnerschrader.com/blueant/services/BaseService?wsdl

$(ARCH)/blueantWorkTimeAccountingService.h: 
	cd $(ARCH) && wsdl2h -nworkTimeAccountingService -c -o blueantWorkTimeAccountingService.h https://blueant.sinnerschrader.com/blueant/services/WorktimeAccountingService?wsdl

$(ARCH):
	mkdir -p $(ARCH)

clean:
	rm -rf $(ARCH)

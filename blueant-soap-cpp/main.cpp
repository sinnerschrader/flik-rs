#include "BaseBinding.nsmap"      // XML namespace mapping table (only needed once at the global level)
#include "soapBaseBindingProxy.h" // the proxy class, also #includes "soapH.h" and "soapStub.h"

int main(int argc, char** argv)
{
  BaseBindingProxy blueantBase;

  struct _ns3__LoginRequestParameter loginParams;
  loginParams.username = argv[1];
  loginParams.password = "uGH~mvVnLw(~bHV@eb~4A{P3-i34wkYHhjk;f3U,mq";

  struct _ns3__session session;

  if (blueantBase.Login(&loginParams, session) == SOAP_OK)
    std::cout << "Sum = " << session.sessionID << std::endl;
  else
    blueantBase.soap_stream_fault(std::cerr);
   blueantBase.destroy(); // same as: soap_destroy(calc.soap); soap_end(calc.soap);
}
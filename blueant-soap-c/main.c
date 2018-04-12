#include "BaseBinding.nsmap" // XML namespace mapping table (only needed once at the global level)
#include "soapH.h"    // client stubs, serializers, etc.


struct soap *newBlueantBase() {
  return soap_new(); // allocate and initalize a context
}

void deleteBlueantBase(struct soap *soap) {
  soap_destroy(soap); // delete deserialized objects
  soap_end(soap);     // delete allocated data
  soap_free(soap);    // free the soap struct context data
}

int blueantLogin(struct soap *soap, char *username, char *password, struct _ns3__session *session) {
    struct _ns3__LoginRequestParameter loginParams;
    loginParams.username = username;
    loginParams.password = password;

    return soap_call___ns1__Login(soap, NULL, NULL, &loginParams, session);
}

int main(int argc, char** argv)
{
  struct soap *soap = soap_new(); // allocate and initalize a context

  struct _ns3__LoginRequestParameter loginParams;
  loginParams.username = argv[1];
  loginParams.password = "uGH~mvVnLw(~bHV@eb~4A{P3-i34wkYHhjk;f3U,mq";

   struct _ns3__session session;

  if (soap_call___ns1__Login(soap, NULL, NULL, &loginParams, &session) == SOAP_OK)
    printf("Session = %s\n", session.sessionID);
  else
    soap_print_fault(soap, stderr);
  soap_destroy(soap); // delete deserialized objects
  soap_end(soap);     // delete allocated data
  soap_free(soap);    // free the soap struct context data
}
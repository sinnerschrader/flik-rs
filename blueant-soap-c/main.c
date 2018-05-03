#include "baseServiceStub.h" // get quote Service stub 
/*#include "worktimeAccountingServiceStub.h" // get rate Service stub */
#include "envH.h"

#include "soap.nsmap" // get quote namespace bindings 
#include "baseService.nsmap" // get quote namespace bindings 
/*#include "worktimeAccountingService.nsmap" // get rate namespace bindings */

struct soap *newBlueantBase() {
  struct soap *soap = soap_new(); // allocate and initalize a context
  soap_set_namespaces(soap, baseService_namespaces);
  return soap;
}

void deleteBlueantBase(struct soap *soap) {
  soap_print_fault(soap, stderr);
  soap_destroy(soap); // delete deserialized objects
  soap_end(soap);     // delete allocated data
  soap_free(soap);    // free the soap struct context data
}

// int blueantLogin(struct soap *soap, char *username, char *password, struct _baseService3__session *session) {
//     struct _baseService3__LoginRequestParameter loginParams;
//     loginParams.username = username;
//     loginParams.password = password;

//     return soap_call___baseService1__Login(soap, NULL, NULL, &loginParams, session);
// }

// int main(int argc, char** argv)
// {
//   struct soap *soap = soap_new(); // allocate and initalize a context
//   soap_set_namespaces(soap, baseService_namespaces);

//   struct _baseService3__LoginRequestParameter loginParams;
//   loginParams.username = argv[1];
//   loginParams.password = "uGH~mvVnLw(~bHV@eb~4A{P3-i34wkYHhjk;f3U,mq";

//    struct _baseService3__session session;

//   if (soap_call___baseService1__Login(soap, NULL, NULL, &loginParams, &session) == SOAP_OK)
//     printf("Session = %s\n", session.sessionID);
//   else
//     soap_print_fault(soap, stderr);
//   soap_destroy(soap); // delete deserialized objects
//   soap_end(soap);     // delete allocated data
//   soap_free(soap);    // free the soap struct context data
// }


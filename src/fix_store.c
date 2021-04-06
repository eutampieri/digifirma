#include <openssl/x509.h>
#include <openssl/x509_vfy.h>
#include <openssl/x509v3.h>
#include <stdio.h>
void fix_store(X509_STORE *store)
{
    printf("%p", store);
    X509_VERIFY_PARAM *param = X509_STORE_get0_param(store);
    X509_VERIFY_PARAM_set_purpose(param, X509_PURPOSE_ANY);
}
SSL/TLS  Failures
---


Python with static ssl library:

```
<Failure: HTTPSConnectionPool(host='plus.three.ie', port=443): Max retries exceeded with url: /core/offers/top 
(Caused by SSLError(SSLError(1, '[SSL: SSLV3_ALERT_HANDSHAKE_FAILURE] sslv3 alert handshake failure (_ssl.c:1000)')))>

OpenSSL 3.0.15 3 Sep 2024
Traceback (most recent call last):
  File "/home/dz/prj/three/three.py", line 24, in <module>
    print(ssl._ssl.__file__)
          ^^^^^^^^^^^^^^^^^
AttributeError: module '_ssl' has no attribute '__file__'. Did you mean: '__name__'?

```


Pyton with dynamic ssl library:
```
<prints response>

OpenSSL 3.2.2 4 Jun 2024
/usr/lib64/python3.12/lib-dynload/_ssl.cpython-312-x86_64-linux-gnu.so

```
The library:
```
ldd /usr/lib64/python3.12/lib-dynload/_ssl.cpython-312-x86_64-linux-gnu.so
    linux-vdso.so.1 (0x00007f581a81b000)
    libssl.so.3 => /lib64/libssl.so.3 (0x00007f581a6ef000)
    libcrypto.so.3 => /lib64/libcrypto.so.3 (0x00007f581a200000)
    libc.so.6 => /lib64/libc.so.6 (0x00007f581a00d000)
    libz.so.1 => /lib64/libz.so.1 (0x00007f581a6cb000)
    /lib64/ld-linux-x86-64.so.2 (0x00007f581a81d000)
```


The way to check ssl:
```
import ssl
print(ssl.OPENSSL_VERSION)
print(ssl._ssl.__file__)


```

Normal Go fails:
```
Get "https://plus.three.ie/core/offers/top": remote error: tls: handshake failure

```

CGo works.

Check server with 
```
openssl s_client -connect plus.three.ie:443 | grep -i tls
Connecting to 134.213.245.205
depth=2 C=US, O=SSL Corporation, CN=SSL.com TLS RSA Root CA 2022
verify return:1
depth=1 C=US, O=SSL Corporation, CN=Entrust OV TLS Issuing RSA CA 1
verify return:1
depth=0 C=IE, L=Dublin 2, O=Three Ireland (Hutchison) Limited, CN=plus.three.ie
verify return:1
   i:C=US, O=SSL Corporation, CN=Entrust OV TLS Issuing RSA CA 1
 1 s:C=US, O=SSL Corporation, CN=Entrust OV TLS Issuing RSA CA 1
   i:C=US, O=SSL Corporation, CN=SSL.com TLS RSA Root CA 2022
 2 s:C=US, O=SSL Corporation, CN=SSL.com TLS RSA Root CA 2022
issuer=C=US, O=SSL Corporation, CN=Entrust OV TLS Issuing RSA CA 1
New, TLSv1.2, Cipher is AES128-SHA256
    Protocol  : TLSv1.2

```
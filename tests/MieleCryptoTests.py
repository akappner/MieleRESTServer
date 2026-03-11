from MieleCrypto import MieleProvisioningInfo, MieleCryptoProvider
import json
import unittest

class CryptoTestCase(unittest.TestCase):
    def setUp(self):
        self.provisioningInfo=MieleProvisioningInfo.generate_random();
        self.cryptoProvider=MieleCryptoProvider(self.provisioningInfo);
    def tearDown(self):
        pass

    def testRoundTrip(self):
        headers, _ = self.cryptoProvider.get_headers_with_auth(
            host="GARBAGE",
            httpMethod="POST",
            resourcePath="/garbageCan",
            contentTypeHeader="",
            acceptHeader="",
            date="March 99, 2099",
            body="trash",
        )
        iv = self.cryptoProvider.iv_from_auth_header(headers["Authorization"])
        payload = self.cryptoProvider.pad_body_bytes(b"PAYLOAD")
        body_encrypted = self.cryptoProvider.encrypt_payload(payload, iv)
        response_plaintext = MieleCryptoProvider.decrypt_bytes(
            body_encrypted, self.provisioningInfo.get_aes_key(), iv
        )
        self.assertEqual(response_plaintext, payload)

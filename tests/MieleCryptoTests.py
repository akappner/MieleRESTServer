from MieleCrypto import MieleProvisioningInfo, MieleCryptoProvider
import json
import unittest

class CryptoTestCase(unittest.TestCase):
    def setUp(self):
        self.provisioningInfo=MieleProvisioningInfo.generate_random();
        self.cryptoProvider=MieleCryptoProvider(self.provisioningInfo);
    def tearDown(self):
        pass;
    def testRoundTrip (self):
        command=json.dumps({"ProcessAction": 1});
        authHeader=self.cryptoProvider.get_auth_header(host="GARBAGE", httpMethod="POST", date="March 99, 2099", resourcePath="/garbageCan", acceptHeader="", contentTypeHeader="", body="trash")
        iv = self.cryptoProvider.iv_from_auth_header(authHeader);
        payload=self.cryptoProvider.pad_body_bytes(b"PAYLOAD")
        body_encrypted=self.cryptoProvider.encrypt_payload (payload, iv);
        response_plaintext = MieleCryptoProvider.decrypt_bytes(body_encrypted, self.provisioningInfo.get_aes_key(), iv);
        self.assertEqual(response_plaintext, payload);

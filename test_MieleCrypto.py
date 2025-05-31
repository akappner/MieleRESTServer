import unittest
import itertools
from MieleCrypto import MieleProvisioningInfo, MieleCryptoProvider

class TestCrypto (unittest.TestCase):
    def setUp(self):
        p=MieleProvisioningInfo("123456789ABCDEFE","123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE");
        self.signature_key=p.get_signature_key();
        self.iv=bytes(range(0,16));
        self.provider=MieleCryptoProvider(p);
        self.plaintext=b"""HELLO ALICE
		THIS IS BOB

		!! }""";
        self.plaintext=MieleCryptoProvider.pad_body_bytes(self.plaintext);
        self.signature=MieleCryptoProvider.sign_bytes (self.plaintext, self.signature_key);


    def test_keygen (self):
        c=MieleCryptoProvider(MieleProvisioningInfo.generate_random());
#        plaintext=c.pad_body_str(plaintext);
        key=c.provisioningInfo.get_aes_key();
        self.assertEqual(MieleCryptoProvider.decrypt_bytes(MieleCryptoProvider.encrypt_bytes(self.plaintext, key, self.iv), key, self.iv), self.plaintext)
    def test_decrypt_invalid (self):
        c=MieleCryptoProvider(MieleProvisioningInfo.generate_random());
        key=c.provisioningInfo.get_aes_key();
        corrupt_key=bytearray(16);
        corrupt_key[:]=key;
        corrupt_key[0:3]=itertools.repeat(0,3);
        
        self.assertNotEqual(MieleCryptoProvider.decrypt_bytes(MieleCryptoProvider.encrypt_bytes(self.plaintext, key, self.iv), corrupt_key, self.iv), self.plaintext)
    def test_verify_signature_valid(self):
        self.assertTrue (MieleCryptoProvider.verify_signature(self.signature, self.plaintext, self.signature_key));
    def test_verify_signature_invalid(self):
        self.assertFalse (MieleCryptoProvider.verify_signature(self.signature, self.plaintext + b"~", self.signature_key));


if __name__ == '__main__':
    unittest.main()


import io
import json
import tempfile
import unittest
from unittest.mock import MagicMock, patch

import murl
from MieleCrypto import MieleCryptoProvider, MieleProvisioningInfo


class DummyResponse:
    def __init__(self, status_code=204, headers=None):
        self.status_code = status_code
        self.headers = headers if headers is not None else {}


class MurlTests(unittest.TestCase):
    def test_parse_url_http(self):
        host, resource_path = murl._parse_url("http://127.0.0.1/State?x=1")
        self.assertEqual(host, "127.0.0.1")
        self.assertEqual(resource_path, "State?x=1")

    def test_parse_url_https_rejected(self):
        with self.assertRaises(ValueError):
            murl._parse_url("https://127.0.0.1/State")

    def test_send_http_request_uses_generic_method(self):
        provisioning_info = MieleProvisioningInfo.generate_random()
        provider = MieleCryptoProvider(provisioning_info)
        response = DummyResponse(status_code=204, headers={})
        with patch(
            "MieleCrypto.requests.request", return_value=response
        ) as request_mock:
            body, full_response = provider.sendHttpRequest(
                httpMethod="post", host="127.0.0.1", resourcePath="State", payload=""
            )
        self.assertEqual(body, b"")
        self.assertIs(full_response, response)
        request_mock.assert_called_once()
        kwargs = request_mock.call_args.kwargs
        self.assertEqual(kwargs["method"].lower(), "post")
        self.assertEqual(kwargs["url"], "http://127.0.0.1/State")

    def test_main_invokes_provider(self):
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=True) as handle:
            handle.write(
                json.dumps(
                    {
                        "GroupID": "8E156D44B8587D36",
                        "GroupKey": (
                            "8E156D44B8587D365EB01CC111DAE14D2619D6E6BFA419E8"
                            "8E156D44B8587D365EB01CC111DAE14D2619D6E6BFA419E8"
                            "8E156D44B8587D365EB01CC111DAE14D"
                        ),
                    }
                )
            )
            handle.flush()

            provider_mock = MagicMock()
            provider_mock.sendHttpRequest.return_value = [
                b'{"ok":true}',
                DummyResponse(
                    status_code=200, headers={"Content-Type": "application/json"}
                ),
            ]

            with patch("murl.MieleCryptoProvider", return_value=provider_mock):
                with patch("sys.stdout", new=io.StringIO()) as stdout:
                    rc = murl.main(
                        [
                            "-X",
                            "GET",
                            "-k",
                            handle.name,
                            "http://127.0.0.1/State",
                        ]
                    )

        self.assertEqual(rc, 0)
        provider_mock.sendHttpRequest.assert_called_once()
        kwargs = provider_mock.sendHttpRequest.call_args.kwargs
        self.assertEqual(kwargs["httpMethod"], "GET")
        self.assertEqual(kwargs["host"], "127.0.0.1")
        self.assertEqual(kwargs["resourcePath"], "State")
        self.assertEqual(stdout.getvalue().strip(), '{"ok":true}')

    def test_main_with_i_prints_status_headers_and_body(self):
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=True) as handle:
            handle.write(
                json.dumps(
                    {
                        "GroupID": "8E156D44B8587D36",
                        "GroupKey": (
                            "8E156D44B8587D365EB01CC111DAE14D2619D6E6BFA419E8"
                            "8E156D44B8587D365EB01CC111DAE14D2619D6E6BFA419E8"
                            "8E156D44B8587D365EB01CC111DAE14D"
                        ),
                    }
                )
            )
            handle.flush()

            provider_mock = MagicMock()
            provider_mock.sendHttpRequest.return_value = [
                b'{"ok":true}',
                DummyResponse(
                    status_code=200, headers={"Content-Type": "application/json"}
                ),
            ]

            with patch("murl.MieleCryptoProvider", return_value=provider_mock):
                with patch("sys.stdout", new=io.StringIO()) as stdout:
                    rc = murl.main(
                        [
                            "-i",
                            "-X",
                            "GET",
                            "-k",
                            handle.name,
                            "http://127.0.0.1/State",
                        ]
                    )

        self.assertEqual(rc, 0)
        output = stdout.getvalue()
        self.assertIn("HTTP 200 OK", output)
        self.assertIn("Content-Type: application/json", output)
        self.assertIn('{"ok":true}', output)

    def test_main_with_v_alias_prints_status_headers_and_body(self):
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=True) as handle:
            handle.write(
                json.dumps(
                    {
                        "GroupID": "8E156D44B8587D36",
                        "GroupKey": (
                            "8E156D44B8587D365EB01CC111DAE14D2619D6E6BFA419E8"
                            "8E156D44B8587D365EB01CC111DAE14D2619D6E6BFA419E8"
                            "8E156D44B8587D365EB01CC111DAE14D"
                        ),
                    }
                )
            )
            handle.flush()

            provider_mock = MagicMock()
            provider_mock.sendHttpRequest.return_value = [
                b'{"ok":true}',
                DummyResponse(
                    status_code=200, headers={"Content-Type": "application/json"}
                ),
            ]

            with patch("murl.MieleCryptoProvider", return_value=provider_mock):
                with patch("sys.stdout", new=io.StringIO()) as stdout:
                    rc = murl.main(
                        [
                            "-v",
                            "-X",
                            "GET",
                            "-k",
                            handle.name,
                            "http://127.0.0.1/State",
                        ]
                    )

        self.assertEqual(rc, 0)
        output = stdout.getvalue()
        self.assertIn("HTTP 200 OK", output)
        self.assertIn("Content-Type: application/json", output)
        self.assertIn('{"ok":true}', output)


if __name__ == "__main__":
    unittest.main()

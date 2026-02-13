from pathlib import Path

from setuptools import setup


README = Path(__file__).with_name("README.md").read_text(encoding="utf-8")


setup(
    name="MieleRESTServer",
    version="1.1.1",
    description="REST server frontend for local control of Miele@home devices",
    long_description=README,
    long_description_content_type="text/markdown",
    py_modules=[
        "Server",
        "MieleApi",
        "MieleCrypto",
        "MieleDop2",
        "MieleDop2Structures",
        "MieleErrors",
        "_version",
    ],
    install_requires=[
        "cryptography",
        "Flask",
        "flask_restful",
        "numpy",
        "PyYAML",
        "Requests",
    ],
    entry_points={
        "console_scripts": [
            "miele-rest-server=Server:main",
        ]
    },
)

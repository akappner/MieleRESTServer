from pathlib import Path

from setuptools import setup


README = Path(__file__).with_name("README.md").read_text(encoding="utf-8")
REQUIREMENTS = [
    line.strip()
    for line in Path(__file__).with_name("requirements.txt").read_text(encoding="utf-8").splitlines()
    if line.strip() and not line.lstrip().startswith("#")
]


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
    install_requires=REQUIREMENTS,
    entry_points={
        "console_scripts": [
            "miele-rest-server=Server:main",
        ]
    },
)

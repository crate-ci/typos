from __future__ import annotations

from setuptools import setup


TYPOS_VERSION = '1.16.8'


setup(
    name='pre_commit_placeholder_package',
    version='0.0.0',
    install_requires=[f'typos=={TYPOS_VERSION}'],
    package_dir={'': 'crates'},
)

#!/usr/bin/env python
from setuptools import setup

setup(
    name='Blueant soap proxy',
    version='1.0',
    long_description=__doc__,
    packages=['server'],
    include_package_data=True,
    zip_safe=False,
    install_requires=['Flask', 'zeep']
)

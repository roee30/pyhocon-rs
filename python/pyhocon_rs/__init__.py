from pyhocon import ConfigFactory
from pyhocon import *
from ._core import parse_string, parse_file

ConfigFactory.parse = parse_string
ConfigFactory.parse_file = parse_file

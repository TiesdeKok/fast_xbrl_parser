### Pytest notes ###
# pytest tests\integration.py -v -s
#
#
#
#

# Imports

import pytest

import os, json, re, sys, time
from pathlib import Path
import pandas as pd
import numpy as np

# Preamble for testing

## Config

TMP_DIR = r'F:\Temp'
BIN_PATH = r"F:\rust_projects\fast_xbrl_parser\target\x86_64-pc-windows-msvc\release\fast_xbrl_parser.exe"

## Test inputs

test_urls = [
    "https://www.sec.gov/Archives/edgar/data/1326380/000132638021000129/gme-20211030_htm.xml",
    "https://www.sec.gov/Archives/edgar/data/1589526/000158952621000140/blbd-20211002_htm.xml"
]

#TODO# Download the XML files instead of using local files? 

test_files = [
    "F:/rust_projects/fast_xbrl_parser/tests/gme-20211030_htm.xml",
]

## Remove old files

def remove_test_files():
    files_to_remove = []
    for path in test_urls + test_files:
        tmp = re.split(r'/', path)[-1]
        tmp = tmp.replace('.xml', '')
        files_to_remove.append(tmp)
        
    files_to_remove = list(set(files_to_remove))

    for file in files_to_remove:
        for target in list(Path(TMP_DIR).glob("*{}*".format(file))):
            os.remove(target)

remove_test_files()

## Helper functions

def check_file(_input, TMP_DIR, check_type):
    file_blank = "{}.json"
    if check_type == 'facts':
        file_blank = "facts_{}.csv"
    elif check_type == 'dimensions':
        file_blank = "dimensions_{}.csv"
        
    file_stem = _input.split(r'/')[-1].replace('.xml', '')

    path_to_check = Path(TMP_DIR) / file_blank.format(file_stem)

    ### Check if exists
    assert path_to_check.exists(), "File {} does not exist".format(path_to_check)

    ## Check if loadable
    
    if check_type in ['facts', 'dimensions']:
        df = pd.read_csv(path_to_check)
        assert not df.empty, "File {} is empty".format(path_to_check)
    else:
        with open(path_to_check, 'r') as f:
            data = json.load(f)
        assert data, "File {} is empty".format(path_to_check)

# ==============
# Python package
# ==============

import fast_xbrl_parser as fxp

def test_fxp_loaded():
    assert "fast_xbrl_parser" in sys.modules, "fast_xbrl_parser not loaded"

print("\nStarting tests for Python package\n")

# --------------
# Parse from URL
# --------------

## Parse from URL and return all data types

@pytest.mark.parametrize("url", test_urls)
def test_python_url(url):
    xbrl_dict = fxp.parse(
        url, 
        output=['json', 'facts', 'dimensions'], 
        email = "test@fast-xbrl-parser.com"
    )
    assert isinstance(xbrl_dict['info'], dict), "info is not a dictionary"
    assert isinstance(xbrl_dict['json'], list), "json is not a list"
    assert isinstance(xbrl_dict['facts'], list), "facts is not a list"
    assert isinstance(xbrl_dict['dimensions'], list), "dimensions is not a list"

    xbrl_fact_df = pd.DataFrame(xbrl_dict['facts'])
    xbrl_dim_df = pd.DataFrame(xbrl_dict['dimensions'])

    assert isinstance(xbrl_fact_df, pd.DataFrame), "facts is not a pandas DataFrame"
    assert isinstance(xbrl_dim_df, pd.DataFrame), "dimensions is not a pandas DataFrame"

## -------------------
## Parse from XML file
## -------------------

@pytest.mark.parametrize("file", test_files)
def test_python_file(file):
    xbrl_dict = fxp.parse(
        file, 
        output=['json', 'facts', 'dimensions']
    )

    assert isinstance(xbrl_dict['info'], dict)
    assert isinstance(xbrl_dict['json'], list)
    assert isinstance(xbrl_dict['facts'], list)
    assert isinstance(xbrl_dict['dimensions'], list)

    xbrl_fact_df = pd.DataFrame(xbrl_dict['facts'])
    xbrl_dim_df = pd.DataFrame(xbrl_dict['dimensions'])

    assert isinstance(xbrl_fact_df, pd.DataFrame)
    assert isinstance(xbrl_dim_df, pd.DataFrame)

# ========================
# Command line application
# ========================

print("\nStarting tests for command line application\n")

# --------------
# Parse from URL
# --------------

@pytest.mark.parametrize("url", test_urls)
def test_cml_url(url):
    url_cmd = r"""
    "{}"
    --input "{}" 
    --json --facts --dimensions 
    --save-dir "{}" --email "test@fast-xbrl-parser.com"
    """.format(BIN_PATH, url, TMP_DIR).replace("\n", " ").strip()

    return_value = os.popen(url_cmd).read()

    ### Check the created files
    
    for data_type in ['json', 'facts', 'dimensions']:
        check_file(url, TMP_DIR, data_type)

# Clean up

remove_test_files()

## -------------------
## Parse from XML file
## -------------------

@pytest.mark.parametrize("file", test_files)
def test_cml_file(file):
    xml_cmd = r"""
    "{}"
    --input "{}"
    --json --facts --dimensions 
    --save-dir "{}"
    """.format(BIN_PATH, file, TMP_DIR).replace("\n", " ").strip()

    ### Check if the outputs are created 
    
    for data_type in ['json', 'facts', 'dimensions']:
        check_file(file, TMP_DIR, data_type)

# Clean up

remove_test_files()
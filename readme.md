<h1 align="center">
    Fast XBRL Parser<br>
    
   <img src="https://i.imgur.com/2KcunUN.png" alt="Fast XBRL Parser" title="Fast XBRL Parser" />
   
</h1>
<p align="center">  
 <a href="https://mybinder.org/v2/gh/TiesdeKok/fast_xbrl_parser/HEAD?labpath=examples%2Fexample.ipynb"><img src="https://mybinder.org/badge_logo.svg"></a>
 <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
  <img src="https://img.shields.io/badge/last%20updated-January%202022-3d62d1">
 
</p>

<p align="center">
An XBRL parser built in Rust that provides a fast, easy, and lightweight way to convert XBRL XML files into JSON or CSV. Available as a Python library or a standalone command line utility. <br><br>
<strong>Warning - currently in a beta stage - use at your own risk</strong>
<br><br>
  <span style='font-size: 15pt'><strong>Author:</strong> Ties de Kok (<a href="https://www.TiesdeKok.com">Personal Page</a>)</span>
</p>

## Table of contents

  * [Introduction](#introduction)
  	* [Design philosophy](#philosophy)
  	* [Caveats](#caveats)
  * [Installation](#installation)
      * [Python](#python-install)
      * [Command Line Tool](#cli-install) 
  * [How to use](#howtouse)
      * [Python](#python)
      * [Command line](#commandline)
  * [Questions?](#questions)
  * [License](#license)

<h2 id="introduction">Introduction</h2>
  
<h3 id="philosophy">Design philosophy</h3>

The objective of `fast-xbrl-parser` is to provide a fast, easy, and lightweight way to parse XBRL XML files into JSON or CSV. 
It is built with the following objectives in mind:

- An easy to use interface   
- Very fast      
- Lightweight and easy to install    
- Cross-platform support (Windows, Linux, and Mac OS are supported)    

<h3 id="caveats">Caveats</h3>

- Only tested on US XBRL files from SEC EDGAR   
- Opinionated conversion to CSV     
- Not validated against the XBRL specification    

Use at your own discretetion and always verify the results yourself. 

<h2 id="installation">Installation</h2>

<h3 id="python-install">Python package</h3>

**Note:** `fast-xbrl-parser` requires Python 3.6+

```bash
pip install fast-xbrl-parser
```
<h3 id="cli-install">CLI tool</h3>

Download the executable file from:

https://github.com/TiesdeKok/fast_xbrl_parser/releases/

<h2 id="howtouse">Basic use</h2>

For full examples and documentation see: [notebook](https://github.com/TiesdeKok/fast_xbrl_parser/blob/master/examples/example.ipynb)

<h3 id="python">As a Python package</h3>

```python
import fast_xbrl_parser as fxp

input = "https://www.sec.gov/Archives/edgar/data/1326380/000132638021000129/gme-20211030_htm.xml" ## Edgar URL
#input = "gme-20211030_htm.xml" ## Local XML file


xbrl_dict = fxp.parse(
    input, 
    output=['json', 'facts', 'dimensions'],   ### You can adjust this to only return certain outputs. 
    email = "demo@fast-xbrl-parser.com"       ### Adjust this to reflect your email address. This is required by the SEC Edgar system when passing a URL.  
) 

json_valid_dict = xbrl_dict['json']
facts_list = xbrl_dict['facts']
facts_df = pd.DataFrame(facts_list)
```

<h3 id="commandline">Standalone using the command line</h3>

```bash
fast_xbrl_parser.exe
--input "https://www.sec.gov/Archives/edgar/data/1589526/000158952621000140/blbd-20211002_htm.xml" 
--json --facts --dimensions 
--save-dir "D:\xbrl_storage" --email "demo@fast-xbrl-parser.com"

## This will save the JSON / CSV files in the `save-dir`
```

<h2 id="questions">Questions?</h2>

If you have questions or experience problems please use the `issues` tab of this repository.

<h2 id="license">License</h2>

[MIT](LICENSE) - Ties de Kok - 2022

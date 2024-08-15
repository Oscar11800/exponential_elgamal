# Exponential Elgamal

## About this project
This project implements the exponential El Gamal encryption scheme using Rust and Noir. The purpose of this project is to be able to run additively homomorphic encryption and decryption within 
the AMD SEV (Secure Encrypted Virtualization) TEE (Trusted exeution environment) more specifically on an AMD EPYC Ubuntu 20.04. 

## Important note: 
Though this project was made with the intention of running within a SEV VM, it can still be fully used outside of the TEE since AMD SEV does not require the usage of a specific SDK and runs 
based on the AMD hardware architecture capabilities. TLDR: You can use this project without TEE.

To learn more about the tools used in this project, take a look at these excellent resources:
- Expopnential El Gamal: https://ieeexplore.ieee.org/document/9845122
- Noir lang: https://noir-lang.org/
- AMD SEV: https://github.com/AMDESE/AMDSEV
- AMD SEV on Ubuntu 20: https://help.ovhcloud.com/csm/asia-dedicated-servers-amd-sme-sev?id=kb_article_view&sysparm_article=KB0044010
- https://libvirt.org/kbase/launch_security_sev.html
- SEV with EPYC: https://www.amd.com/content/dam/amd/en/documents/epyc-technical-docs/tuning-guides/58207-using-sev-with-amd-epyc-processors.pdf

## Requirements
- Noir >= 0.32.0
- Cargo/Rust >= 0.1.0
- Ubuntu 20.04 (If using AMD SEV TEE following these instructions, follow AMD SEV link above to use a different distro)

## Build and Installation


## Running Tests



## Credit
This code was adapted from jat92392 's https://github.com/jat9292/Private-token project. 

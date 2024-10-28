#!/bin/bash

source .env
hurl --variable QA_API_KEY="$QA_API_KEY" tests/hurl/submit.hurl | jq

#!/usr/bin/env bash
# Stolen from https://www.lpalmieri.com/posts/2020-08-31-zero-to-production-3-5-html-forms-databases-integration-tests/#3-4-database-setup
set -x
set -eo pipefail

bash ./scripts/init_db.sh
bash ./scripts/init_redis.sh

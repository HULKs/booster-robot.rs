#! /bin/bash

find booster_robotics_sdk -name "*.so" \
  | xargs -I{} sh -c 'readelf -d "{}" | grep SONAME' \
  | sort -u

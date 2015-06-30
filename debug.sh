#!/bin/sh
# Copyright 2014 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

# Exit if anything fails
set -e

# Find out where the pretty printer Python module is
RUSTC_SYSROOT=`rustc --print=sysroot`
GDB_PYTHON_MODULE_DIRECTORY="$RUSTC_SYSROOT/lib/rustlib/etc"

# Run GDB with the additional arguments that load the pretty printers
PYTHONPATH="$PYTHONPATH:$GDB_PYTHON_MODULE_DIRECTORY" ./rust-os-gdb/bin/gdb \
  -d "$GDB_PYTHON_MODULE_DIRECTORY" \
  -iex "add-auto-load-safe-path $GDB_PYTHON_MODULE_DIRECTORY" \
  "build/isofiles/boot/rustos.debug" \
  -ex "target remote localhost:1234" \
  "$@"

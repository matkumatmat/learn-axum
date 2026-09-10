#!/bin/bash
# cargo watch -q -c -w testing/tests -w module-04-response/src -x "test -p testing --test module_04 -- --nocapture --test-threads=1"
# cargo watch -q -c -w testing/tests -w module-04-response/src -x "test -p testing --test module_04 -- --nocapture --test-threads=1"

cargo watch -q -c -w testing/tests -w module-05-state/src -x "test -p testing --test module_05 -- --nocapture --test-threads=1"

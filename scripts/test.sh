#!/bin/bash
# cargo watch -q -c -w testing/tests -w module-04-response/src -x "test -p testing --test module_04 -- --nocapture --test-threads=1"
# cargo watch -q -c -w testing/tests -w module-04-response/src -x "test -p testing --test module_04 -- --nocapture --test-threads=1"
# cargo watch -q -c -w testing/tests -w module-04-response/src -x "test -p testing --test module_04 -- --nocapture --test-threads=1"

# cargo watch -q -c -w testing/tests -w module-06-middleware/src -x "test -p testing --test module_06 -- --nocapture --test-threads=1"
# cargo watch -q -c -w testing/tests -w module-07-error/src -x "test -p testing --test module_07 -- --nocapture --test-threads=1"
# cargo watch -q -c -w testing/tests -w module-08-database/src -x "test -p testing --test module_08 -- --nocapture --test-threads=1"

cargo watch -q -c -w testing/tests -w module-09-auth/src -x "test -p testing --test module_09 -- --nocapture --test-threads=1"

# cargo watch -q -c -w module-06-middleware/src -x "test -p module-06-middleware test_06 -- --nocapture --test-threads=1"

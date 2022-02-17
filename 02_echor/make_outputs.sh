#!/bin/bash

# Create folder where sample echo command output will be stored.
OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

# Generate some sample output using the "echo" cmd.
# These will be referenced in tests to make sure "echor" produces the same output.
echo "Hello world" > $OUTDIR/hello1.txt
echo "Hello" "world" > $OUTDIR/hello2.txt
echo -n "Hello world" > $OUTDIR/hello1.n.txt
echo -n "Hello" "world" > $OUTDIR/hello2.n.txt


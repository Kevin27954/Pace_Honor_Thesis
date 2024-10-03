#!/bin/bash


cargo run run main.txt &
RUST_PID=$!
echo "Rust program is running with PID: $RUST_PID"

# Step 2: Use dtrace to profile the process while it is running
DTRACE_OUTPUT="out.stacks"
echo "Profiling process with dtrace... (PID: $RUST_PID)"
# Attach dtrace to the running process, and stop when the process exits
sudo dtrace -x ustackframes=100 -p $RUST_PID -n 'profile-997 /pid == $target/ { @[ustack()] = count(); }' -o $DTRACE_OUTPUT &

wait $RUST_PID

# Step 3: Collapse the stack traces using stackcollapse-dtrace.pl
FOLDED_OUTPUT="out.folded"
echo "Collapsing stack traces..."
stackcollapse.pl $DTRACE_OUTPUT > $FOLDED_OUTPUT

# Step 4: Generate the flamegraph using flamegraph.pl
FLAMEGRAPH_OUTPUT="flamegraph.svg"
echo "Generating flamegraph..."
flamegraph.pl $FOLDED_OUTPUT > $FLAMEGRAPH_OUTPUT
echo "Flamegraph generated: $FLAMEGRAPH_OUTPUT"

# Step 5: Open the flamegraph in the browser
echo "Opening flamegraph in Arc browser..."
open -a "Arc" $FLAMEGRAPH_OUTPUT



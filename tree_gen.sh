#!/bin/sh

# This is a sh script to gen a process tree.
# It will be useful if u test the module in 
# some enviorments which dont have many processes, like busybox. 

create_process_tree() {
  local depth=$1
  local width=$2

  if [ $depth -eq 0 ]; then
    return
  fi

  for i in $(seq 1 $width); do
    create_process_tree $((depth - 1)) $width &
    local child_pid=$!

    echo "Parent PID: $$, Child PID: $child_pid, Depth: $depth"
  done

  sleep 10000000
  echo "PID($$) ends"
}

tree_depth=5
tree_width=3

create_process_tree $tree_depth $tree_width

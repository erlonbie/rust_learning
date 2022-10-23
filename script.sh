#!/bin/sh

for d in */; do
	echo "$d"
	cd "$d"
	for file in .*; do
  #   if [[ $file == ".gitignore" ]]; then
		# cat .gitignore
  #   fi
  echo $file
  rm -rf .git
  rm .gitignore
	done
	cd ..
done

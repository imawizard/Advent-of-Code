#!/bin/bash

readme=README.md

{
  echo
  for yeardir in $(printf '%s\n' 20* | sort -r); do
    year=$(basename "$yeardir")
    echo "## $year"
    echo
    echo "|Day|Solutions|"
    echo "|-|-|"

    for daydir in "$yeardir"/Day-*; do
      files=("$daydir"/main.*)

      found=()
      for file in "${files[@]}"; do
        test -f "$file" || continue
        ext=${file##*.}
        found+=("$ext:$file")
      done
      if [[ -d $daydir/gleam ]]; then
        found+=("gleam:$daydir/gleam")
      fi

      solutions=""
      for found in $(printf '%s\n' "${found[@]}" | sort); do
        ext=${found%%:*}
        file=${found##*:}

        case $ext in
          clj) url="scripts/assets/clojure.svg" ;;
          dart) url="scripts/assets/dart.svg" ;;
          gleam) url="scripts/assets/gleam.svg" ;;
          go) url="scripts/assets/go.svg" ;;
          pl) url="scripts/assets/perl.svg" ;;
          php) url="scripts/assets/php.svg" ;;
          rs) url="scripts/assets/rust.svg" ;;
          zig) url="scripts/assets/zig.svg" ;;
        esac
        solutions="$solutions [<img src=\"$url\">]($file)"
      done

      if [[ $solutions ]]; then
        day=${daydir##*-}
        echo "| $day |$solutions |"
      fi
    done

    echo
  done
} | perl -e '
while (<>) {
  if (/^<!-- BEGIN/../^<!-- END/) {
    print if /^<!--/;
    print <STDIN> if /BEGIN/
  } else {
    print
  }
}
' $readme >$readme.new

if [[ -f $readme.new ]]; then
  rm $readme
  mv $readme.new $readme
fi

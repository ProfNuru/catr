#!/usr/bin/env bash

OUTDIR = "tests/expected"
INDIR = "tests/inputs"

[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

cat $INDIR/empty.txt > $OUTDIR/empty.txt
cat $INDIR/fox.txt > $OUTDIR/fox.txt
cat $INDIR/spiders.txt > $OUTDIR/spiders.txt
cat $INDIR/the-bustle.txt > $OUTDIR/the-bustle.txt
cat -n $INDIR/empty.txt > $OUTDIR/empty-n.txt
cat -n $INDIR/fox.txt > $OUTDIR/fox-n.txt
cat -n $INDIR/spiders.txt > $OUTDIR/spiders-n.txt
cat -n $INDIR/the-bustle.txt > $OUTDIR/the-bustle-n.txt
cat -b $INDIR/empty.txt > $OUTDIR/empty-b.txt
cat -b $INDIR/fox.txt > $OUTDIR/fox-b.txt
cat -b $INDIR/spiders.txt > $OUTDIR/spiders-b.txt
cat -b $INDIR/the-bustle.txt > $OUTDIR/the-bustle-b.txt

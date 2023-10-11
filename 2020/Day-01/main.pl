#!/usr/bin/perl
use strict qw(vars subs);
use warnings;
use v5.16;

sub part1 {
	my (@vals) = @_;
	outer:
	for my $i (@vals[0..$#vals - 1]) {
		for my $j (@vals[1..$#vals]) {
			if ($i + $j == 2020) {
				say $i * $j;
				last outer;
			}
		}
	}
}

sub part2 {
	my (@vals) = @_;
	outer:
	for my $i (@vals[0..$#vals - 2]) {
		for my $j (@vals[1..$#vals - 1]) {
			for my $k (@vals[2..$#vals]) {
				if ($i + $j + $k == 2020) {
					say $i * $j * $k;
					last outer;
				}
			}
		}
	}
}

my @vals = map s/\s+$//r, do {
	local @ARGV = ("../input01.txt");
	<>;
};
part1(@vals);
part2(@vals);

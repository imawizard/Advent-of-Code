#!/usr/bin/perl
use strict qw(vars subs);
use warnings;
use v5.16;

sub part_a {
	my (@vals) = @_;

	for my $i (@vals[0..$#vals - 1]) {
		for my $j (@vals[1..$#vals]) {
			if ($i + $j == 2020) {
				return $i * $j;
			}
		}
	}
	return 0;
}

sub part_b {
	my (@vals) = @_;

	for my $i (@vals[0..$#vals - 2]) {
		for my $j (@vals[1..$#vals - 1]) {
			for my $k (@vals[2..$#vals]) {
				if ($i + $j + $k == 2020) {
					return $i * $j * $k;
				}
			}
		}
	}
	return 0;
}

my @vals = map s/\s+$//r, do {
	local @ARGV = ("input");
	<>;
};
say "a: ", part_a(@vals);
say "b: ", part_b(@vals);

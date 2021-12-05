#include "lib.hpp"
#include <vector>
#include <algorithm>
#include <bitset>
#include <iostream>

template <unsigned int bit_cnt, bool is_sample>
static int do_task(std::FILE* f) {
	unsigned bits[bit_cnt] {};
	unsigned num_in = 0;
	std::vector<unsigned short> p2list;
	char buf[bit_cnt+2];
	while (std::fgets(buf, sizeof buf, f)) {
		if (buf[bit_cnt] != '\n')
			return 3;
		// Part 1
		for (unsigned i = 0; i < bit_cnt; ++i)
			bits[i] += buf[i] == '1';
		// Part 2
		unsigned x = 0;
		for (unsigned i = 0; i < bit_cnt; ++i)
			x = (x << 1) | (buf[i] == '1');
		p2list.push_back(x);
		++num_in;
	}
	if (!std::feof(f))
		return 2;
	// Part 1
	unsigned p1n = 0, p1o;
	for (unsigned i = 0; i < bit_cnt; ++i)
		p1n |= (1 << (bit_cnt - i - 1)) * (bits[i] >= num_in/2);
	p1o = (~p1n) & ((1 << bit_cnt) - 1);
	// Part 2
	unsigned p2n = 0, p2o = 0;
	auto p2nl = p2list, p2ol = p2list;
	for (unsigned i = 0; i < bit_cnt; ++i) {
		unsigned m = 1 << (bit_cnt - i - 1), a = 0, b = 0;
		for (auto x : p2nl)
			a += (x & m) != 0;
		for (auto x : p2ol)
			b += (x & m) != 0;
		if (is_sample)
			std::printf("%u %u;%zu %zu@%u\n", a, b, p2nl.size(), p2ol.size(), m);
		a = (a*2 >= p2nl.size()) ? m : 0;
		b = (b*2 >= p2ol.size()) ? 0 : m;
		p2nl.erase(std::remove_if(p2nl.begin(), p2nl.end(), [&] (auto x) { return (x & m) != a; }), p2nl.end());
		p2ol.erase(std::remove_if(p2ol.begin(), p2ol.end(), [&] (auto x) { return (x & m) != b; }), p2ol.end());
		if (is_sample) {
			for (auto x : p2ol)
				std::cout << std::bitset<bit_cnt>(x) << '\n';
			std::puts("=====");
		}
		if (p2nl.size() == 1)
			p2n = p2nl[0];
		if (p2ol.size() == 1)
			p2o = p2ol[0];
	}
	std::printf("Part 1: %u %u %u\nPart 2: %u %u %u\n", p1n, p1o, p1n * p1o, p2n, p2o, p2n * p2o);
	return 0;
}

AoC_DEF_TASK(3) {
	constexpr unsigned int bc_task = 12, bc_sample = 5;
	if (is_sample)
		return do_task<bc_sample, true>(f);
	else
		return do_task<bc_task, false>(f);
}

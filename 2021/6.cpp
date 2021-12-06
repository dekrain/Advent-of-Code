#include "lib.hpp"

// Arbitrary-size uint, unneded lol
#if 0
struct auint {
	unsigned long long _smi;
	//unsigned long* _big;

	auint(unsigned long long smi = 0)
		: _smi(smi)
	{}

	[[gnu::always_inline]]auint& operator++() {
		/*if (_big) {
			_big_inc(1);
		} else*/ if (_smi != ~0ULL) {
			++_smi;
		} else {
			_conv_to_big();
			_big_inc(1);
		}
		return *this;
	}

	[[gnu::always_inline]]auint& operator+=(auint const& inc) {
		/*if (_big) {
			_big_inc(inc);
		} else if (inc._big) {
			_conv_to_big();
			_big_inc(inc);
		} else*/ if (_smi + inc._smi < _smi) {
			_conv_to_big();
			_big_inc(inc);
		} else {
			_smi += inc._smi;
		}
		return *this;
	}

	[[gnu::always_inline]]void print(std::FILE* f) {
		std::fprintf(f, "%llu", _smi);
	}

	[[gnu::always_inline]]void _conv_to_big() {
		/*_big = new unsigned long;
		_smi = ~0ULL;*/
		__builtin_unreachable();
	}

	[[gnu::always_inline]]void _big_inc(auint const& inc) {
		// TODO
		__builtin_unreachable();
	}
};
#endif
using auint = unsigned long long;

AoC_DEF_TASK(6) {
	unsigned const p1_num_days = 80, p2_num_days = 256;
	unsigned offs = 0;
	auint live[7]{}, born[2]{};
	auint p1 = 0, p2 = 0,* pp = &p1;
	while (true) {
		int ch = std::fgetc(f);
		if (ch == '\n')
			break;
		if (ch == ',')
			continue;
		if (not (ch >= '0' and ch <= '6'))
			return 1;
		++live[ch - '0'];
		++p1;
	}
	if (is_sample) {
		std::printf("Input: %llu %llu %llu %llu %llu %llu %llu\n",
			live[0], live[1], live[2], live[3], live[4], live[5], live[6]);
	}
	for (unsigned sim_count = 0; sim_count < p2_num_days; ++sim_count) {
		auint num_born = live[offs];
		live[offs] += born[1];
		born[1] = born[0];
		if (sim_count == p1_num_days) {
			p2 = p1;
			pp = &p2;
			std::fputc('\n', stdout);
		}
		*pp += born[0] = num_born;
		if (is_sample)
			std::printf("%llu ", *pp);
		offs = (offs + 1) % 7;
	}
	if (is_sample)
		std::fputc('\n', stdout);
	std::printf("Part 1: %llu\nPart 2: %llu\n", p1, p2);
	return 0;
}

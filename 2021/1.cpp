#include "lib.hpp"
#include <cstdlib>

AoC_DEF_TASK(1) {
	char buf[10];
	unsigned int p1=0, p1pos, p1last=-1, p1cur,
		p2=0, p2b1=-1, p2b2=-1;
	while (!std::feof(f)) {
		if (!std::fgets(buf, sizeof buf, f)) {
			if (std::feof(f)) break;
			else return 2;
		}
		// Part one
		p1pos = 0;
		while (buf[p1pos] != '\n')
			if (!buf[p1pos])
				return 3;
			else ++p1pos;
		p1cur = std::strtoul(buf, nullptr, 10);
		if (p1cur > p1last)
			++p1;
		// Part 2
		if (p2b1 != -1 and p2b2 != -1 and p1cur > p2b2)
			++p2;
		p2b2 = p2b1;
		p2b1 = p1last;
		p1last = p1cur;
	}
	std::printf("Part 1: %d\nPart 2: %d\n", p1, p2);
	return 0;
}

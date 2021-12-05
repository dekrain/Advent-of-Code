#include "lib.hpp"
#include <cstdlib>

AoC_DEF_TASK(2) {
	char buf[15];
	size_t p1dep = 0, p1lat = 0, p2aim = 0, p2dep = 0;
	while (std::fgets(buf, sizeof buf, f)) {
		size_t len = 0, sp = -1;
		while (buf[len] != '\n') {
			if (!buf[len])
				return 3;
			else {
				if (buf[len] == ' ')
					sp = len;
				++len;
			}
		}
		if (sp == -1)
			return 4;
		size_t num = std::strtoul(buf + sp + 1, nullptr, 10);
		// Part 1
		switch (buf[0]) {
			case 'f':
				p1lat += num;
				p2dep += num * p2aim;
				break;
			case 'd':
				p1dep += num;
				p2aim += num;
				break;
			case 'u':
				p1dep -= num;
				p2aim -= num;
				break;
			default:
				return 5;
		}
	}
	if (!std::feof(f))
		return 2;
	std::printf("Part 1: %zu\nPart 2: %zu\n", p1dep * p1lat, p2dep * p1lat);
	return 0;
}

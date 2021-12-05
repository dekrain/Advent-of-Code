#include "lib.hpp"
#include <unordered_map>
#include <cstdint>
#include <charconv>
#include <cstring>

struct map_proxy {
	std::unordered_map<uint32_t, uint32_t> _map;
	uint32_t _unit = 0, _pos = 0;
	enum state {
		unvisited = 0,
		visited = 1,
		saturated = 3,
	};

	void wind(uint16_t x, uint16_t y) {
		uint32_t _npos = y | ((x << 12) & 0xFFFF0000U);
		if (_pos != _npos) {
			_map[_pos] = _unit;
			_pos = _npos;
			_unit = _map[_npos];
		}
	}

	state get(uint16_t x, uint16_t y) {
		wind(x, y);
		return static_cast<state>(3 & (_unit >> (2*(x & 0xF))));
	}

	void put(uint16_t x, uint16_t y) {
		wind(x, y);
		_unit |= 1 << (2*(x & 0xF));
	}

	void sat(uint16_t x, uint16_t y) {
		wind(x, y);
		_unit |= 3 << (2*(x & 0xF));
	}
};

AoC_DEF_TASK(5) {
	map_proxy p1_world_map, p2_world_map;
	char buf[25];
	unsigned p1 = 0, p2 = 0;
	while (std::fgets(buf, sizeof buf, f)) {
		char const* end = buf + std::strlen(buf);
		char const* p = buf;
		uint16_t fromx, fromy, tox, toy;
		auto res = std::from_chars(p, end, fromx);
		if (res.ec != std::errc())
			return 2;
		p = res.ptr + 1; // Skip ','
		res = std::from_chars(p, end, fromy);
		if (res.ec != std::errc())
				return 2;
		p = res.ptr + 4; // Skip ' -> '
		res = std::from_chars(p, end, tox);
		if (res.ec != std::errc())
			return 2;
		p = res.ptr + 1; // Skip ','
		res = std::from_chars(p, end, toy);
		if (res.ec != std::errc())
			return 2;

		if (is_sample)
			std::printf("%d,%d -> %d,%d\n", fromx, fromy, tox, toy);
		// Branchless dir function variant that I completely forgot about
		// Adopted from https://cs.stackexchange.com/questions/49987/branchless-function-equivalent
		int16_t dx = (fromx < tox) - (fromx > tox),
		        dy = (fromy < toy) - (fromy > toy);
		bool p1on = fromx == tox or fromy == toy;
		while (true) {
			// Part 1
			if (p1on) {
				switch (p1_world_map.get(fromx, fromy)) {
					case map_proxy::unvisited:
						p1_world_map.put(fromx, fromy);
						break;
					case map_proxy::visited:
						++p1;
						p1_world_map.sat(fromx, fromy);
						break;
					case map_proxy::saturated:
						break;
				}
			}
			// Part 2
			switch (p2_world_map.get(fromx, fromy)) {
				case map_proxy::unvisited:
					p2_world_map.put(fromx, fromy);
					break;
				case map_proxy::visited:
					++p2;
					p2_world_map.sat(fromx, fromy);
					break;
				case map_proxy::saturated:
					break;
			}

			if (fromx == tox and fromy == toy)
				break;
			fromx += dx;
			fromy += dy;
		}
	}
	if (!std::feof(f))
		return 1;
	std::printf("Part 1: %u\nPart 2: %u\n", p1, p2);
	return 0;
}

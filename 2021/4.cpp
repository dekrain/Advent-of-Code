#include "lib.hpp"
#include <cstring>
#include <charconv>
#include <vector>

struct _board_check {
	int start; // First index
	int inc;   // Increment for a win check
	// I thought this was gonna be needed for diagonals, but sadly they didn't get into the task
	// And they don't even need it
	//int acc; // Increment of the increment per index
	// =======
	int cnt;   // Count of win checks
	int delt;  // Increment for starting position between checks
};
_board_check _board_checks_p1[] = {
	{0, 1, 5, 5}, // Rows
	{0, 5, 5, 1}, // Cols
};

#if 0
_board_check _board_checks_diag[] = {
	{0, 6, 1, 0}, // Main diagonal, direct
	{4, 4, 1, 0}, // Main diagonal, inverted
};
#endif

struct board {
	int _fields[5*5];
	int _sum = 0;
	bool _won = false;

	bool parse(int n, char const* p) {
		char const* const end = p + std::strlen(p);
		for (int const e = n+5; n != e; ++n) {
			while (*p == ' ')
				++p;
			auto res = std::from_chars(p, end, _fields[n]);
			if (res.ec != std::errc())
				return true;
			_sum += _fields[n];
			p = res.ptr;
		}
		return false;
	}

	bool adv(int num) {
		int s = _sum;
		for (auto& v : _fields) {
			if (v == num) {
				v = -1;
				_sum -= num;
			}
		}
		if (s != _sum) {
			for (auto& chck : _board_checks_p1) {
				int sidx = chck.start, inc = chck.inc;
				int c = chck.cnt;
				while (c--) {
					int idx = sidx; bool pass = true;
					for (int i = 0; i < 5; ++i) {
						if (_fields[idx] != -1) {
							pass = false;
							break;
						}
						idx += inc;
						//inc += chck.acc;
					}
					if (pass)
						return true;
					sidx += chck.delt;
				}
			}
		}
		return false;
	}

	int score() {
		return _sum;
	}
};

AoC_DEF_TASK(4) {
	char inp[1024], bo[40],* inend;
	char const* p1in;
	if (!std::fgets(inp, sizeof inp, f))
		return 1;
	inend = inp + std::strlen(inp);
	std::vector<board> boards;
	while (!std::feof(f)) {
		board& b = boards.emplace_back();
		// Parse boards
		if (std::fgetc(f) != '\n') {
			if (std::feof(f))
				break;
			else
				return 2;
		}
		for (int i = 0; i < 5; ++i) {
			if (!std::fgets(bo, sizeof bo, f))
				return 3;
			if (b.parse(i*5, bo)) return 6;
		}
	}
	p1in = inp;
	int num = -1, p1num = -1, p2num = -1;
	board* p1win = nullptr,* p2last = nullptr;
	while (p1in < inend) {
		auto res = std::from_chars(p1in, inend, num);
		if (res.ec != std::errc())
			return 4;
		p1in = res.ptr + 1; // Skip comma
		bool board_left = false;
		for (auto& b : boards) {
			if (b._won)
				continue;
			if (b.adv(num)) {
				b._won = true;
				p2last = &b;
				p2num = num;
				if (!p1win) {
					p1win = &b;
					p1num = num;
				}
			} else {
				board_left = true;
			}
		}
		if (!board_left)
			break;
	}
	if (!p1win)
		return 5;

	std::printf("Part 1: %d\nPart 2: %d\n", p1num * p1win->score(), p2num * p2last->score());
	return 0;
}

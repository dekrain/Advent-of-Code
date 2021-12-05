#include "lib.hpp"
#include <vector>

static std::vector<AoCTask*>* _tasks;

void AoCAddTask(AoCTask& t) noexcept {
	if (!_tasks)
		_tasks = new std::vector<AoCTask*>;
	if (!t.day)
		return;
	if (t.day > _tasks->size())
		_tasks->resize(t.day);
	(*_tasks)[t.day-1] = &t;
}

int main() {
	std::puts("AoC 2021!");
	if (!_tasks) {
		std::puts("No tasks!");
		return 1;
	}
	for (auto t : *_tasks) {
		std::printf("--------------\n"
			"Task %d\n", t->day);
		if (!t) {
			std::puts("Task missing!");
		} else {
			char buf[15];
			std::snprintf(buf, sizeof buf, "%d.input.sam", t->day);
			std::FILE* f = std::fopen(buf, "r");
			if (f) {
				std::puts("-=EXAMPLE=-");
				if (int res = t->run(f, true))
					return res;
				std::fclose(f);
				std::puts("-=TASK=-");
			}
			std::snprintf(buf, sizeof buf, "%d.input", t->day);
			f = std::fopen(buf, "r");
			if (!f)
				return 1;
			if (int res = t->run(f, false))
				return res;
			std::fclose(f);
		}
	}
}

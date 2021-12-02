#pragma once

#include <cstdio>

struct AoCTask;
extern void AoCAddTask(AoCTask& t) noexcept;

struct AoCTask {
	unsigned int day;
	int (*run)(std::FILE*);

	AoCTask(unsigned day = 0, int (*run)(std::FILE*) = nullptr)
		noexcept : day(day), run(run)
	{
		if (day)
			AoCAddTask(*this);
	}
};

#define AoC_DEF_TASK(n) \
	static int _task_##n##_run_ (std::FILE*); \
	AoCTask _task_##n##_ (n, &_task_##n##_run_); \
	static int _task_##n##_run_ (std::FILE* f)


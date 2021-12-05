#pragma once

#include <cstdio>

struct AoCTask;
extern void AoCAddTask(AoCTask& t) noexcept;

using AoCTaskProg = int (std::FILE* f, bool is_sample);

struct AoCTask {
	unsigned int day;
	AoCTaskProg* run;

	AoCTask(unsigned day = 0, AoCTaskProg* run = nullptr)
		noexcept : day(day), run(run)
	{
		if (day)
			AoCAddTask(*this);
	}
};

#define AoC_DEF_TASK(n) \
	static AoCTaskProg _task_##n##_run_; \
	AoCTask _task_##n##_ (n, &_task_##n##_run_); \
	static int _task_##n##_run_ (std::FILE* f, bool is_sample)

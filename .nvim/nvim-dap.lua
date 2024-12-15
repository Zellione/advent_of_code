local dap = require("dap")

dap.adapters.lldb = {
	type = "executable",
	command = "/usr/bin/codelldb", -- adjust as needed, must be absolute path
	name = "lldb",
}

dap.configurations.c = {
	{
		name = "Launch Day4 Part1 Debug",
		type = "lldb",
		request = "launch",
		program = vim.fn.getcwd() .. "/aoc2024/day04/part1/build/default/advent_of_code_day4_part1",
		cwd = "${workspaceFolder}/aoc2024/day04/part1/build/default",
		stopOnEntry = false,
		args = {},
	},
	{
		name = "Launch Day4 Part2 Debug",
		type = "lldb",
		request = "launch",
		program = vim.fn.getcwd() .. "/aoc2024/day04/part2/build/default/advent_of_code_day4_part2",
		cwd = "${workspaceFolder}/aoc2024/day04/part2/build/default",
		stopOnEntry = false,
		args = {},
	},
	{
		name = "Launch Day5 Part1 Debug",
		type = "lldb",
		request = "launch",
		program = vim.fn.getcwd() .. "/aoc2024/day05/part1/build/default/advent_of_code_day5_part1",
		cwd = "${workspaceFolder}/aoc2024/day05/part1/build/default",
		stopOnEntry = false,
		args = {},
	},
}

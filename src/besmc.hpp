#pragma once

#include <string>
#include <vector>

struct Options {
    std::string output_file;   // empty = derive from first input
    bool stop_at_object = false;
    bool save_temps = false;
    std::vector<std::string> files;
};

// Parse argv; throws on bad usage. prog is argv[0].
Options parse_args(int argc, char* argv[]);

// Compile/link according to options; throws std::runtime_error on failure.
void compile_files(const Options& opt);

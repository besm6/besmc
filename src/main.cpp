#include "besmc.hpp"

#include <iostream>
#include <stdexcept>

int main(int argc, char* argv[]) {
    try {
        if (argc < 2) {
            // Match clap arg_required_else_help: show help when no args.
            char* help_argv[] = {argv[0], const_cast<char*>("--help"), nullptr};
            parse_args(2, help_argv);
            return 0;
        }
        compile_files(parse_args(argc, argv));
        return 0;
    } catch (const std::exception& e) {
        std::cerr << e.what() << '\n';
        return 1;
    }
}

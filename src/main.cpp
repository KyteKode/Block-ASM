#include <string>
#include <vector>
#include <algorithm>
#include <fstream>

#include <basm.hpp>

int main(int argc, char* argv[]) {
    std::vector<std::string> args(argv, argv + argc);

    std::ifstream basm_file(args[1]);

    auto it = std::find(args.begin(), args.end(), "-o");
    std::string out_name;
    if (it != args.end()) {
        int idx = std::distance(args.begin(), it);
        out_name = args[idx];
    } else {
        out_name = args[1] + ".sb3";
    }

    // TODO: handle compilation flags other than -os

    return 0;
}

/*
-o changes output file name
--reverse converts sb3 back to basm source
-Wop checks for invalid opcodes
-Wparent checks for invalid parent pointers
-Wnext checks for invalid next pointers
-Win checks for invalid inputs
-Wfield checks for invalid fields
-Wmut checks for invalid mutations
-Wall checks for invalid anything
-Werror treats warnings as errors
--stdout returns the sb3 binary in stdout
--verbose tells you the source code path, when tokenization starts, when tokenization ends, how long tokenization took,
how many tokens there are in total, how many tokens of each type there are, when parsing starts, when parsing ends,
how long parsing took, how many AST nodes there were in total, how many of each node type there are, how many blocks there are in total,
how many sprites there are in total and their names, how many blocks are in each sprite, how many threads there are in total,
how many threads are in each sprite, how many global variables there are and their names and values, how many cloud variables there are and their names,
how many sprite variables there are and their names and owners and values, how many global lists there are and their names,
how many sprite lists there are and their names and owners, when sb3 conversion starts, when sb3 conversion ends,
how long sb3 conversion took, and the path to the resulting sb3.
*/
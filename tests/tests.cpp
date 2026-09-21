#include "besmc.hpp"

#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

namespace fs = std::filesystem;

static int g_failed = 0;
static int g_passed = 0;

#define CHECK(cond) do { \
    if (!(cond)) { \
        std::cerr << "FAIL " << __FILE__ << ":" << __LINE__ << " — " #cond << '\n'; \
        ++g_failed; \
    } else { \
        ++g_passed; \
    } \
} while (0)

#define CHECK_EQ(a, b) do { \
    auto _a = (a); auto _b = (b); \
    if (_a != _b) { \
        std::cerr << "FAIL " << __FILE__ << ":" << __LINE__ \
                  << " — got \"" << _a << "\" expected \"" << _b << "\"\n"; \
        ++g_failed; \
    } else { \
        ++g_passed; \
    } \
} while (0)

static std::string find_line(const std::string& filename, const std::string& prefix) {
    std::ifstream in(filename);
    for (std::string line; std::getline(in, line);)
        if (line.starts_with(prefix)) return line;
    return {};
}

static void expect_fail(const Options& opt) {
    try {
        compile_files(opt);
        std::cerr << "FAIL " << __func__ << " — expected compilation to fail\n";
        ++g_failed;
    } catch (const std::exception&) {
        ++g_passed;
    }
}

static void write_file(const std::string& path, const std::string& contents) {
    fs::create_directories(fs::path(path).parent_path());
    std::ofstream(path) << contents;
}

// ---- options ----

static void test_output_and_ftn_files() {
    char* argv[] = {
        const_cast<char*>("besmc"),
        const_cast<char*>("-o"), const_cast<char*>("out"),
        const_cast<char*>("test.ftn"), const_cast<char*>("main.ftn"),
    };
    auto opt = parse_args(5, argv);
    CHECK_EQ(opt.output_file, "out");
    CHECK(!opt.stop_at_object);
    CHECK_EQ(opt.files.size(), 2u);
    CHECK_EQ(opt.files[0], "test.ftn");
    CHECK_EQ(opt.files[1], "main.ftn");
}

static void test_mixed_file_types() {
    char* argv[] = {
        const_cast<char*>("besmc"),
        const_cast<char*>("-c"),
        const_cast<char*>("src.ftn"),
        const_cast<char*>("code.assem"),
        const_cast<char*>("obj.obj"),
    };
    auto opt = parse_args(5, argv);
    CHECK(opt.output_file.empty());
    CHECK(opt.stop_at_object);
    CHECK_EQ(opt.files.size(), 3u);
}

// ---- exe / obj ----

static std::string lst_of(const std::string& out) {
    return fs::path(out).replace_extension("lst").string();
}

static void test_exe(const char* src, const char* out, const char* expected_suffix) {
    Options opt;
    opt.output_file = out;
    opt.files = {src};
    compile_files(opt);
    auto line = find_line(lst_of(out), " ДЛИHA БИБЛИOTEKИ");
    CHECK_EQ(line, std::string(" ДЛИHA БИБЛИOTEKИ") + expected_suffix);
}

static void test_obj(const char* src, const char* out) {
    Options opt;
    opt.output_file = out;
    opt.files = {src};
    opt.stop_at_object = true;
    compile_files(opt);
    CHECK_EQ(find_line(lst_of(out), " ДЛИHA LIBRARY"), " ДЛИHA LIBRARY 0001 ЗOH.");
}

static void neg_exe(const std::string& contents, const std::string& src) {
    write_file(src, contents);
    Options opt;
    opt.output_file = fs::path(src).replace_extension("exe").string();
    opt.files = {src};
    expect_fail(opt);
}

static void neg_obj(const std::string& contents, const std::string& src) {
    write_file(src, contents);
    Options opt;
    opt.output_file = fs::path(src).replace_extension("obj").string();
    opt.stop_at_object = true;
    opt.files = {src};
    expect_fail(opt);
}

int main() {
    fs::create_directories("target");

    test_output_and_ftn_files();
    test_mixed_file_types();

    test_exe("examples/hello.b",       "target/hello_b.exe",       "  001 30");
    test_exe("examples/hello.algol",   "target/hello_algol.exe",   "  001 17");
    test_exe("examples/hello.assem",   "target/hello_assem.exe",   "  001 01");
    test_exe("examples/hello.bemsh",   "target/hello_bemsh.exe",   "  001 01");
    test_exe("examples/hello.forex",   "target/hello_forex.exe",   "  002 30");
    test_exe("examples/hello.fortran", "target/hello_fortran.exe", "  002 30");
    test_exe("examples/hello.ftn",     "target/hello_ftn.exe",     "  002 30");
    test_exe("examples/hello.madlen",  "target/hello_madlen.exe",  "  001 01");
    test_exe("examples/hello.pascal",  "target/hello_pascal.exe",  "  002 17");
    test_exe("examples/hello.pas",     "target/hello_pas.exe",     "  002 17");

    // C tests need b6 toolchain; skip if unavailable
    if (std::system("command -v b6parse >/dev/null 2>&1") == 0) {
        test_exe("examples/hello.c", "target/hello_c.exe", "  003 03");
        test_obj("examples/hello.c", "target/lib_c.obj");
    } else {
        std::cerr << "skip C tests (b6parse not on PATH)\n";
    }

    test_obj("examples/hello.algol",   "target/lib_algol.obj");
    test_obj("examples/hello.assem",   "target/lib_assem.obj");
    test_obj("examples/hello.bemsh",   "target/lib_bemsh.obj");
    test_obj("examples/hello.forex",   "target/lib_forex.obj");
    test_obj("examples/hello.fortran", "target/lib_fortran.obj");
    test_obj("examples/hello.ftn",     "target/lib_ftn.obj");
    test_obj("examples/hello.madlen",  "target/lib_madlen.obj");
    test_obj("examples/hello.pascal",  "target/lib_pascal.obj");

    // stdarray
    {
        Options opt;
        opt.files = {"examples/stdarray.std"};
        opt.output_file = "target/hello_std.exe";
        compile_files(opt);
        CHECK_EQ(find_line("target/hello_std.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 17");
    }
    {
        Options opt;
        opt.files = {"examples/stdarray.std"};
        opt.output_file = "target/lib_stdarray.obj";
        opt.stop_at_object = true;
        compile_files(opt);
        CHECK_EQ(find_line("target/lib_stdarray.lst", " ДЛИHA LIBRARY"), " ДЛИHA LIBRARY 0001 ЗOH.");
    }

    // pascal + fortran link
    {
        Options a{.output_file = "target/caller.obj", .stop_at_object = true,
                  .files = {"examples/caller.pascal"}};
        compile_files(a);
        Options b{.output_file = "target/callee.obj", .stop_at_object = true,
                  .files = {"examples/callee.ftn"}};
        compile_files(b);
        Options link{.output_file = "target/pascal_to_fortran.exe",
                     .files = {"target/caller.obj", "target/callee.obj"}};
        compile_files(link);
        CHECK_EQ(find_line("target/pascal_to_fortran.lst", " ДЛИHA БИБЛИOTEKИ"),
                 " ДЛИHA БИБЛИOTEKИ  004 03");
    }

    // negative exe
    neg_exe("'begin'\n    badprint(''Hello, Algol!'');\n'end'\n'eop'\n",
            "target/algol_bad_program.algol");
    neg_exe("foo\nbar\n", "target/algol_no_eop.algol");
    neg_exe("        ,end,\n", "target/assem_no_header.assem");
    neg_exe(" program: ,name,\n        ,uj, foobar\n        ,end,\n",
            "target/assem_undefined_identifier.assem");
    neg_exe(" main: ,name,\n        ,*74,\n        ,end,\n", "target/assem_absent_program.assem");
    neg_exe("        ,end,\n", "target/madlen_no_header.madlen");
    neg_exe(" program: ,name,\n        ,uj, foobar\n        ,end,\n",
            "target/madlen_undefined_identifier.madlen");
    neg_exe("ввд$$$\n        э74\n        финиш\nквч$$$\nтрн$$$\nкнц$$$\n",
            "target/bemsh_no_header.bemsh");
    neg_exe("ввд$$$\nmain    старт   512\n        пб      куда\n        финиш\nквч$$$\nтрн$$$\nкнц$$$\n",
            "target/bemsh_undefined_identifier.bemsh");
    neg_exe("        program test\n        a = b\n        end\n",
            "target/forex_undefined_variable.forex");
    neg_exe("        program test\n        goto 123\n        end\n",
            "target/forex_undefined_label.forex");
    neg_exe("        program test\n        a = b\n        end\n",
            "target/fortran_undefined_variable.fortran");
    neg_exe("        program test\n        goto 123\n        end\n",
            "target/fortran_undefined_label.fortran");
    neg_exe("        program test\n        a = b\n        end\n",
            "target/ftn_undefined_variable.ftn");
    neg_exe("        program test\n        goto 123\n        end\n",
            "target/ftn_undefined_label.ftn");
    neg_exe("_(\n    stop;\n_).\n", "target/pascal_missing_program.pascal");
    neg_exe("program main(output);\n_(\n    a = 123;\n    stop;\n_).\n",
            "target/pascal_undefined_variable.pascal");

    // negative obj
    neg_obj("'begin'\n    badprint(''Hello, Algol!'');\n'end'\n'eop'\n",
            "target/obj_algol_bad_program.algol");
    neg_obj("foo\nbar\n", "target/obj_algol_no_eop.algol");
    neg_obj("        ,end,\n", "target/obj_assem_no_header.assem");
    neg_obj(" program: ,name,\n        ,uj, foobar\n        ,end,\n",
            "target/obj_assem_undefined_identifier.assem");
    neg_obj("        ,end,\n", "target/obj_madlen_no_header.madlen");
    neg_obj(" program: ,name,\n        ,uj, foobar\n        ,end,\n",
            "target/obj_madlen_undefined_identifier.madlen");
    neg_obj("ввд$$$\n        э74\n        финиш\nквч$$$\nтрн$$$\nкнц$$$\n",
            "target/obj_bemsh_no_header.bemsh");
    neg_obj("ввд$$$\nmain    старт   512\n        пб      куда\n        финиш\nквч$$$\nтрн$$$\nкнц$$$\n",
            "target/obj_bemsh_undefined_identifier.bemsh");
    neg_obj("        program test\n        a = b\n        end\n",
            "target/obj_forex_undefined_variable.forex");
    neg_obj("        program test\n        goto 123\n        end\n",
            "target/obj_forex_undefined_label.forex");
    neg_obj("        program test\n        a = b\n        end\n",
            "target/obj_fortran_undefined_variable.fortran");
    neg_obj("        program test\n        goto 123\n        end\n",
            "target/obj_fortran_undefined_label.fortran");
    neg_obj("        program test\n        a = b\n        end\n",
            "target/obj_ftn_undefined_variable.ftn");
    neg_obj("        program test\n        goto 123\n        end\n",
            "target/obj_ftn_undefined_label.ftn");
    neg_obj("_(\n    stop;\n_).\n", "target/obj_pascal_missing_program.pascal");
    neg_obj("program main(output);\n_(\n    a = 123;\n    stop;\n_).\n",
            "target/obj_pascal_undefined_variable.pascal");

    std::cout << g_passed << " passed, " << g_failed << " failed\n";
    return g_failed ? 1 : 0;
}

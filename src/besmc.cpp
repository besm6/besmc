#include "besmc.hpp"

#include <algorithm>
#include <cctype>
#include <cstdlib>
#include <fcntl.h>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <regex>
#include <stdexcept>
#include <string>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>
#include <vector>

namespace fs = std::filesystem;

namespace {

[[noreturn]] void fail(const std::string& msg) { throw std::runtime_error(msg); }

bool ieq(std::string_view a, std::string_view b) {
    return a.size() == b.size() &&
           std::equal(a.begin(), a.end(), b.begin(),
                      [](char x, char y) { return std::tolower(static_cast<unsigned char>(x)) ==
                                                  std::tolower(static_cast<unsigned char>(y)); });
}

bool has_ext(std::string_view name, std::string_view ext) {
    if (name.size() < ext.size()) return false;
    return ieq(name.substr(name.size() - ext.size()), ext);
}

std::string ext_of(const std::string& path) {
    auto e = fs::path(path).extension().string();
    if (!e.empty() && e[0] == '.') e.erase(0, 1);
    return e;
}

std::string with_ext(const std::string& path, const char* ext) {
    return fs::path(path).replace_extension(ext).string();
}

void remove_quiet(const std::string& path) {
    std::error_code ec;
    fs::remove(path, ec);
    if (ec && ec != std::errc::no_such_file_or_directory)
        std::cout << "Cannot remove '" << path << "': " << ec.message() << '\n';
}

void make_executable(const std::string& path) {
    struct stat st{};
    if (stat(path.c_str(), &st) != 0) fail(std::format("Cannot get metadata for {}: {}", path, errno));
    if (chmod(path.c_str(), st.st_mode | 0111) != 0)
        fail(std::format("Cannot set permissions for {}: {}", path, errno));
}

void copy_contents(std::ostream& dest, const std::string& src) {
    std::ifstream in(src, std::ios::binary);
    if (!in) fail(std::format("Failed to open file '{}'", src));
    dest << in.rdbuf();
    if (!dest) fail("Failed to copy to destination");
}

void append_prefixed(std::ostream& dest, const std::string& src, std::string_view prefix) {
    dest << prefix;
    copy_contents(dest, src);
}

std::vector<std::string> share_dirs() {
    std::vector<std::string> d;
    if (const char* home = std::getenv("HOME"))
        d.push_back(std::format("{}/.local/share/besm6", home));
    d.push_back("/usr/local/share/besm6");
    d.push_back("/usr/share/besm6");
    return d;
}

std::string find_include_dir() {
    std::string probed;
    for (auto& base : share_dirs()) {
        auto dir = base + "/include";
        if (fs::is_directory(dir)) return dir;
        probed += "\n  " + dir;
    }
    fail("BESM-6 C headers not found. Looked in:" + probed);
}

std::string find_libc() {
    std::string probed;
    for (auto& base : share_dirs()) {
        auto lib = base + "/lib/libc.bin";
        if (fs::is_regular_file(lib)) return lib;
        probed += "\n  " + lib;
    }
    fail("BESM-6 libc.bin not found. Looked in:" + probed);
}

// Run program with args; optionally redirect stdout to a file. Throws on failure.
void run_pass(const std::string& prog, const std::vector<std::string>& args,
              const std::string& stdout_path = {}) {
    std::vector<char*> argv;
    argv.reserve(args.size() + 2);
    argv.push_back(const_cast<char*>(prog.c_str()));
    for (auto& a : args) argv.push_back(const_cast<char*>(a.c_str()));
    argv.push_back(nullptr);

    pid_t pid = fork();
    if (pid < 0) fail(std::format("Failed to execute {}: fork", prog));
    if (pid == 0) {
        if (!stdout_path.empty()) {
            int fd = open(stdout_path.c_str(), O_WRONLY | O_CREAT | O_TRUNC, 0644);
            if (fd < 0) _exit(127);
            dup2(fd, STDOUT_FILENO);
            close(fd);
        }
        execvp(prog.c_str(), argv.data());
        _exit(127);
    }
    int status = 0;
    if (waitpid(pid, &status, 0) < 0) fail(std::format("Failed to execute {}: wait", prog));
    if (!WIFEXITED(status) || WEXITSTATUS(status) != 0) {
        int code = WIFEXITED(status) ? WEXITSTATUS(status) : status;
        fail(std::format("{} failed with status: {}", prog, code));
    }
}

std::string create_perso(std::ostream& script, const std::string& obj, int& perso) {
    if (perso >= 060) fail(std::format("Cannot process {}: too many object files", obj));
    script << std::format("*file:pers{:o},{:o}\n", perso, perso);
    auto bin = std::format("pers{:o}.bin", perso);
    ++perso;
    std::error_code ec;
    fs::copy_file(obj, bin, fs::copy_options::overwrite_existing, ec);
    if (ec) fail(std::format("Failed to copy {} to {}: {}", obj, bin, ec.message()));
    return bin;
}

bool listing_has_errors(const std::string& path) {
    static const std::regex patterns[] = {
        std::regex(R"(БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ)"),
        std::regex(R"(HET ′EOP′)"),
        std::regex(R"(OTCYTCTBYET ИMЯ ПPOГPAMMЫ)"),
        std::regex(R"(OTCYTCTBYET ИMЯ ПOДПPOГPAMMЫ)"),
        std::regex(R"(OTCYTCTBYET ЗAГOЛOBOK ПOДПPOГPAMMЫ)"),
        std::regex(R"(OTCYTCTBYET  PROGRAM)"),
        std::regex(R"(ЗHAЧEH.* HE OПPEДEЛEHO)"),
        std::regex(R"(INCORRECT ALGOL PROGRAM)"),
        std::regex(R"(\*\*\*\*\*\*HEOПИCAHHЫЙ ИДEHTИФИKATOP)"),
        std::regex(R"(\*\*\*\*\*\* HEOПИCAHHЫЙ ИДEHTИФИKATOP:)"),
        std::regex(R"(^ \*\*\*\*\*\*\d+ )"),
        std::regex(R"(^HEOП MET )"),
        std::regex(R"(^ERROR \d+)"),
        std::regex(R"(^ ERROR \d+)"),
        std::regex(R"(OTCYTCTBYET)"),
        std::regex(R"(HEДOПYCTИMЫЙ OПEPATOP:)"),
        std::regex(R"(ДЛИHHЫЙ AДPEC B)"),
    };

    std::ifstream in(path);
    if (!in) fail(std::format("Failed to open file {}: {}", path, errno));
    bool found = false;
    for (std::string line; std::getline(in, line);) {
        for (auto& re : patterns) {
            if (std::regex_search(line, re)) {
                std::cout << line << '\n';
                found = true;
                break;
            }
        }
    }
    return found;
}

}  // namespace

Options parse_args(int argc, char* argv[]) {
    Options opt;
    for (int i = 1; i < argc; ++i) {
        std::string_view a = argv[i];
        if (a == "-h" || a == "--help") {
            std::cout <<
R"(BESM-6 compiler frontend

Usage: besmc [OPTIONS] <FILES>...

Arguments:
  <FILES>...  Input sources and object files:
              *.ftn     - Fortran-ГДP
              *.fortran - Fortran Dubna
              *.forex   - Forex
              *.algol   - Algol-ГДP
              *.pascal  - Pascal
              *.pas     - Pascal-re
              *.assem   - Assembler Madlen
              *.madlen  - Assembler Madlen-3.5
              *.bemsh   - Assembler БЕМШ
              *.b       - B language
              *.c       - C language
              *.obj     - Object Library (*perso)
              *.std     - Standard array (*punch)

Options:
  -o, --output <FILE>  Output file name
  -c, --compile        Compile only to object files
  -t, --save-temps     Keep intermediate files
  -h, --help           Print help
)";
            std::exit(0);
        } else if (a == "-c" || a == "--compile") {
            opt.stop_at_object = true;
        } else if (a == "-t" || a == "--save-temps") {
            opt.save_temps = true;
        } else if (a == "-o" || a == "--output") {
            if (++i >= argc) fail("Missing argument for -o/--output");
            opt.output_file = argv[i];
        } else if (a.starts_with("-")) {
            fail(std::format("Unknown option: {}", a));
        } else {
            opt.files.emplace_back(a);
        }
    }
    if (opt.files.empty()) fail("No input files");
    return opt;
}

void compile_files(const Options& opt) {
    const auto& first = opt.files[0];
    const auto out_base = opt.output_file.empty() ? first : opt.output_file;
    const auto output_file = with_ext(out_base, opt.stop_at_object ? "obj" : "exe");
    const auto listing_file = with_ext(out_base, "lst");
    const auto script_file = with_ext(out_base, "dub");

    std::vector<std::string> temps{"output.bin", script_file};
    auto input = opt.files;

    for (auto& file : input) {
        if (!has_ext(file, ".pas")) continue;
        auto std_file = with_ext(file, "std");
        run_pass("pascompl", {"-P", file, std_file});
        file = std_file;
        temps.push_back(std_file);
    }

    const bool has_c = std::any_of(opt.files.begin(), opt.files.end(),
                                   [](auto& f) { return has_ext(f, ".c"); });

    std::string include_dir;
    for (auto& file : input) {
        if (!has_ext(file, ".c")) continue;
        if (include_dir.empty()) include_dir = find_include_dir();
        auto i_file = file + ".i";
        auto asn_file = file + ".asn";
        auto tac_file = file + ".tac";
        auto madlen_file = file + ".madlen";
        auto inc_flag = "-I" + include_dir;
        run_pass("cpp", {"-E", "-nostdinc", inc_flag, file, i_file});
        run_pass("b6parse", {i_file, asn_file});
        run_pass("b6lower", {asn_file, tac_file});
        run_pass("b6codegen", {"--madlen", tac_file, madlen_file});
        file = madlen_file;
        for (auto& t : {i_file, asn_file, tac_file, madlen_file}) temps.push_back(t);
    }

    const bool link_libc = has_c && !opt.stop_at_object;
    if (link_libc) {
        auto libc_path = find_libc();
        remove_quiet("libc.bin");
        if (symlink(libc_path.c_str(), "libc.bin") != 0)
            fail(std::format("Failed to create libc.bin symlink to {}: {}", libc_path, errno));
        temps.emplace_back("libc.bin");
    }

    {
        std::ofstream script(script_file);
        if (!script) fail(std::format("Failed to create {}", script_file));
        script << "*name compile\n*disc:1/local\n*file:output,60,w\n";
        if (link_libc) script << "*file:libc,37\n";

        int perso = 040;
        for (auto& file : input)
            if (ext_of(file) == "obj") temps.push_back(create_perso(script, file, perso));

        if (std::any_of(input.begin(), input.end(), [](auto& f) { return has_ext(f, ".b"); }))
            script << "*tape:7/b,40\n*library:40\n";

        script << "*call setftn:one,long\n";

        perso = 040;
        for (auto& file : input) {
            auto e = ext_of(file);
            if (e == "ftn")          append_prefixed(script, file, "*ftn\n");
            else if (e == "fortran") append_prefixed(script, file, "*fortran\n");
            else if (e == "forex")   append_prefixed(script, file, "*forex\n");
            else if (e == "algol")   append_prefixed(script, file, "*algol\n");
            else if (e == "pascal")  append_prefixed(script, file, "*pascal\n");
            else if (e == "assem")   append_prefixed(script, file, "*assem\n");
            else if (e == "madlen")  append_prefixed(script, file, "*madlen\n");
            else if (e == "bemsh")   append_prefixed(script, file, "*bemsh\n");
            else if (e == "b")       append_prefixed(script, file, "*trans-main:40020\n");
            else if (e == "obj") {
                script << std::format("*call perso:{:o},cont\n", perso);
                ++perso;
            } else if (e == "std") {
                append_prefixed(script, file, "");
            } else if (e == "exe") {
                fail(std::format("Cannot process executable file: {}", file));
            } else if (e.empty()) {
                fail(std::format("Cannot process file without extension: {}", file));
            } else {
                fail(std::format("Unknown file extension: {}", file));
            }
        }

        if (opt.stop_at_object) {
            script << "*call to perso: 60\n*end file\n";
        } else {
            const char* entry = has_ext(first, ".bemsh") ? "main" : "program";
            if (link_libc) script << "*library:37\n";
            script << std::format("*library:22\n*call overlay\n{}\n*end record\n*end file\n", entry);
        }
        if (!script) fail(std::format("Failed to write {}", script_file));
    }

    run_pass("dubna", {script_file}, listing_file);

    if (listing_has_errors(listing_file))
        fail(std::format("---\nCompilation failed!\nSee details in {}", listing_file));

    {
        std::ofstream out(output_file, std::ios::binary);
        if (!out) fail(std::format("Failed to create {}", output_file));
        if (!opt.stop_at_object) out << "#!/usr/bin/env dubna\n";
        copy_contents(out, "output.bin");
    }

    if (!opt.save_temps)
        for (auto& t : temps) remove_quiet(t);

    if (!opt.stop_at_object) make_executable(output_file);
}

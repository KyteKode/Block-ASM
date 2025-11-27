#ifndef KYTEKODE_BASM
#define KYTEKODE_BASM

#if __cplusplus < 201703L
#error "basm.hpp requires C++17 or higher."
#endif


#include <vector>
#include <string>
#include <iostream>
#include <variant>
#include <sstream>
#include <unordered_map>
#include <functional>
#include <fstream>

#include <nlohmann/json.hpp>

namespace Block_ASM {
    using std::string;
    using std::vector;
    
    using json = nlohmann::json;
    
    struct BLOCK {};
    struct UID {};
    struct PARENT {};
    struct NEXT {};
    struct OPCODE {};
    struct INPUT {};
    struct FIELD {};
    struct MUTATION {};
    struct NULL_T {};
    struct STRING_T {};
    struct DOUBLE_T {};
    struct INT_T {};
    struct LIST_IDX_T {};
    struct WAIT_T {};
    struct BLOCK_PTR_T {};
    struct SUBSTACK_T {};
    struct RECIEVED_BROADCAST_T {};
    struct DATA_PTR_T {};
    struct END {};
    struct SEMICOLON {};
    struct MISC_DATA { string data; };

    using token = std::variant<
        BLOCK,
        UID,
        PARENT,
        NEXT,
        OPCODE,
        INPUT,
        FIELD,
        MUTATION,
        NULL_T,
        STRING_T,
        DOUBLE_T,
        INT_T,
        LIST_IDX_T,
        WAIT_T,
        BLOCK_PTR_T,
        SUBSTACK_T,
        RECIEVED_BROADCAST_T,
        DATA_PTR_T,
        END,
        SEMICOLON,
        MISC_DATA
    >;
    // I would've made this an enum, but MISC_DATA NEEDS TO HOLD A VALUE.
    // WHY THE FRIG DOES C++ NOT JUST HAVE RUST-LIKE ENUMS BUILT IN AAAAAAAAAAA

    inline vector<token> tokenize(string &basm_code) {
        std::stringstream ss(basm_code);
        string s_token;

        vector<token> tokens;
        const std::unordered_map<string, std::function<token()>> token_map = {
            {"block", [](){ return BLOCK{}; }},
            {"uid", [](){ return UID{}; }},
            {"parent", [](){ return PARENT{}; }},
            {"next", [](){ return NEXT{}; }},
            {"opcode", [](){ return OPCODE{}; }},
            {"input", [](){ return INPUT{}; }},
            {"field", [](){ return FIELD{}; }},
            {"mutation", [](){ return MUTATION{}; }},
            {"null", [](){ return NULL_T{}; }},
            {"string", [](){ return STRING_T{}; }},
            {"double", [](){ return DOUBLE_T{}; }},
            {"int", [](){ return INT_T{}; }},
            {"wait", [](){ return WAIT_T{}; }},
            {"list_idx", [](){ return LIST_IDX_T{}; }},
            {"block_ptr", [](){ return BLOCK_PTR_T{}; }},
            {"substack", [](){ return SUBSTACK_T{}; }},
            {"recieved_broadcast", [](){ return RECIEVED_BROADCAST_T{}; }},
            {"data_ptr", [](){ return DATA_PTR_T{}; }},
            {"end", [](){ return END{}; }},
            {"semicolon", [](){ return SEMICOLON{}; }},
        };

        while(ss >> s_token) {
            token temp;
            auto entry = token_map.find(s_token);
            if (entry != token_map.end()) {
                tokens.push_back(entry->second());
            } else {
                tokens.push_back(MISC_DATA{ s_token });
            }
        }

        return tokens;
    }
};

#endif
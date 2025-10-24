#include <vector>
#include <string>
#include <iostream>
#include <optional>
#include <any>

using std::cout;
using std::endl;
using std::nullopt;
using std::optional;
using std::ostream;
using std::string;
using std::vector;

#include "include/rapidjson/writer.h"

struct Block
{
    string uid;
    string opcode;
    vector<string> inputs;
    vector<string> fields;
    optional<string> parent;
    optional<string> next;
    bool hat;

    void reset()
    {
        uid = "";
        opcode = "";
        inputs = {};
        fields = {};
        parent = nullopt;
        next = nullopt;
        hat = true;
    }

    rapidjson::Document to_sb3()
    {
        rapidjson::Document doc;
        doc.SetObject();

        rapidjson::Document::AllocatorType &allocator = doc.GetAllocator();

        doc.AddMember("uid", uid, allocator);
        doc.AddMember("opcode", opcode, allocator);

        rapidjson::Value json_inputs(rapidjson::kArrayType);
        for (const string &el : inputs)
        {
            json_inputs.PushBack(el, allocator);
        }
        doc.AddMember("inputs", json_inputs, allocator);

        rapidjson::Value json_fields(rapidjson::kArrayType);
        for (const string &el : fields)
        {
            json_fields.PushBack(el, allocator);
        }
        doc.AddMember("fields", json_fields, allocator);

        json_parent = (parent.has_value() ? parent.value() : rapidjson::Value(rapidjson::kNullType));
        doc.AddMember("parent", json_parent, allocator);

        json_next = (next.has_value() ? next.value() : rapidjson::Value(rapidjson::kNullType));
        doc.AddMember("next", json_next, allocator);

        rapidjson::Value json_mutations(rapidjson::kArrayType);
        doc.AddMember("mutations", json_mutations, allocator);

        rapidjson::StringBuffer buffer;
        rapidjson::Writer<rapidjson::StringBuffer> writer(buffer);
        doc.Accept(writer);

        return doc;
    }
};

vector<string> tokenize(string &basm_code)
{
    if (basm_code.back() != ' ')
    {
        basm_code.push_back(' ');
    }

    vector<string> tokens;
    bool notString = true;

    string token;
    char prev_char = '\0';
    for (char c : basm_code)
    {
        bool next_token = c == ' ';
        next_token = next_token || c == '\n';
        next_token = next_token && notString;

        if (next_token)
        {
            tokens.push_back(token);
            token = "";
        }
        else
        {
            if (c == '"' && prev_char != '\\')
            {
                notString = !notString;
            }
            token.push_back(c);
        }
        prev_char = c;
    }

    return tokens;
}

vector<Block> parse(vector<string> &tokens)
{
    vector<Block> blocks = {};

    Block b;

    int token_type = 0;
    for (string token : tokens)
    {
        switch (token_type)
        {
        case 0: // Parent
            if (token != "hat")
            {
                b.hat = false;
                b.parent = token;
            }
            token_type = 1;
            break;
        case 1: // UID
            b.uid = token;
            token_type = 2;
            break;
        case 2: // Opcode
            b.opcode = token;
            token_type = 3;
            break;
        case 3: // Input
            if (token == "/")
            {
                token_type = 4;
            }
            else
            {
                char start = token.at(0);
                char end = token.back();
                if (start == '"' && end == '"')
                {
                    string data = token.substr(1, token.length() - 2);
                    b.inputs.push_back(data);
                }
            }
            break;
        case 4: // Field
            if (token == ";")
            {
                token_type = 5;
            }
            else
            {
                char start = token.at(0);
                char end = token.back();
                if (start == '"' && end == '"')
                {
                    string data = token.substr(1, token.length() - 2);
                    b.fields.push_back(data);
                }
            }
            break;
        case 5: // Next
            if (token != "null")
            {
                b.next = token;
            }
            token_type = 6;
            break;
        case 6:
            blocks.push_back(b);
            b.reset();
            token_type = 0;
            break;
        }
    }

    return blocks;
}

template <typename T>
ostream &operator<<(ostream &os, const vector<T> &vector_in)
{
    os << '{' << endl;
    for (const T &value : vector_in)
    {
        os << value;
    }
    os << '}' << std::flush;
    return os;
}

ostream &operator<<(ostream &os, const optional<string> opt_string)
{
    os << (opt_string.has_value() ? opt_string.value() : "nullopt");
    return os;
}

ostream &operator<<(ostream &os, const Block &block)
{
    os << "UID: " << block.uid << endl;
    os << "Opcode: " << block.opcode << endl;
    os << "Inputs: " << block.inputs << endl;
    os << "Fields: " << block.fields << endl;
    os << "Parent: " << block.parent << endl;
    os << "Next: " << block.next << endl;
    os << "Hat: " << block.hat << endl;
    return os;
}

int main()
{
    /*
    hat 0001 event_whenflagclicked / ; 0002 end
    0001 0002 looks_say "Hello, world!" / ; 0003 end
    0002 0003 control_stop / "all" ; null end

    should be:

    when green flag clicked
    say(Hello, world!)
    stop [all v]
    */

    string text = R"(hat 0001 event_whenflagclicked / ; 0002 end
0001 0002 looks_say "Hello, world!" / ; 0003 end
0002 0003 control_stop / "all" ; null end)";

    auto tokens = tokenize(text);

    cout << parse(tokens);
}

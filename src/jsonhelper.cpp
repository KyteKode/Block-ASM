#include <vector>
#include <string>
#include <optional>
#include <type_traits>

using std::string;
using std::vector;
using std::optional;
using std::is_same_v;

#include "include/rapidjson/document.h"
using rapidjson::Document;
using rapidjson::Value;
using rapidjson::kArrayType;
using rapidjson::kNullType;

template <typename X, typename Y>
inline bool is_type(X &data) {
    return typeid(data) == typeid(Y);
}

class JSONWrapper {
private:
    Document doc;
    Document::AllocatorType &allocator;


    Value private_add_value(const JSONWrapper &value) {
        return Value(value.json(), allocator);
    }

    template <typename T>
    Value private_add_value(const T &value) {
        if constexpr (is_same_v<T, JSONWrapper>) {
            return Value(value.json(), allocator);
        } else if (is_same_v<T, string>) {
            return Value(value.c_str(), allocator);
        } else {
            return Value(value, allocator);
        }
    }

    template <typename T>
    Value private_add_value(const optional<T> &value) {
        if (!value.has_value()) {
            return Value(kNullType);
        }

        const T &data = value.value();

        if constexpr (is_same_v<T, JSONWrapper>) {
            return Value(data.json(), allocator);
        } else if (is_same_v<T, string>) {
            return Value(data.c_str(), allocator);
        } else {
            return Value(data, allocator);
        }
    }

    template <typename T>
    Value private_add_value(const vector<T> &value) {
        Value j_value(kArrayType);
        for (const T &el : value) {
            if constexpr (is_same_v<T, JSONWrapper>) {
                j_value.PushBack(Value(el.json(), allocator), allocator);
            } else if (is_same_v<T, string>) {
                j_value.PushBack(Value(el.c_str(), allocator), allocator);
            } else {
                j_value.PushBack(Value(el.json(), allocator), allocator);
            }
        }
        return j_value;
    }

public:
    JSONWrapper() : doc(), allocator(doc.GetAllocator()) {
        doc.SetObject();
    }

    Document& json() { return doc; }

    template <typename T>
    void add_value(const string &key, const T &value) {
        Value j_key(key.c_str(), allocator);
        doc.AddMember(j_key, private_add_value(value), allocator);
    }
};
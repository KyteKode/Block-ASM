#include <vector>
#include <sstring>
#include <string>
#include <variant>

using std::string;
using std::variant;
using std::vector;

vector<string> splitString(const string &input, char delimiter)
{
    vector<string> tokens;
    std::stringstream ss(input);
    string token;

    while (std::getline(ss, token, delimiter))
    {
        tokens.push_back(token);
    }

    return tokens;
}

int main()
{
    std::string text = "apple,banana,orange";
    char delimiter = ',';

    std::vector<std::string> result = splitString(text, delimiter);

    for (const auto &token : result)
    {
        std::cout << token << std::endl;
    }
    return 0;
}
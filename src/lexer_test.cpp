#include <iostream>
#include <vector>
#include "lexer.cpp"

int main()
{

    std::string input = "myPerson: Person(firstName: String(\"John\"), lastName: String(\"Doe\"), age: Int.I32(42), color: Color.RED)";
    std::cout << "Input: " << input << std::endl;
    Lexer lexer(input);

    std::vector<Token> expected_tokens = {
        Token(TokenKind::IDENTIFIER, "myPerson", 1, 1),
        Token(TokenKind::COLON, ":", 1, 9),
        Token(TokenKind::SPACE, " ", 1, 10),
        Token(TokenKind::IDENTIFIER, "Person", 1, 11),
        Token(TokenKind::L_PAREN, "(", 1, 17),
        Token(TokenKind::IDENTIFIER, "firstName", 1, 18),
        Token(TokenKind::COLON, ":", 1, 27),
        Token(TokenKind::SPACE, " ", 1, 28),
        Token(TokenKind::IDENTIFIER, "String", 1, 29),
        Token(TokenKind::L_PAREN, "(", 1, 35),
        Token(TokenKind::STRING_LITERAL, "John", 1, 36),
        Token(TokenKind::R_PAREN, ")", 1, 41),
        Token(TokenKind::COMMA, ",", 1, 42),
        Token(TokenKind::SPACE, " ", 1, 43),
        Token(TokenKind::IDENTIFIER, "lastName", 1, 44),
        Token(TokenKind::COLON, ":", 1, 52),
        Token(TokenKind::SPACE, " ", 1, 53),
        Token(TokenKind::IDENTIFIER, "String", 1, 54),
        Token(TokenKind::L_PAREN, "(", 1, 60),
        Token(TokenKind::STRING_LITERAL, "Doe", 1, 61),
        Token(TokenKind::R_PAREN, ")", 1, 65),
        Token(TokenKind::COMMA, ",", 1, 66),
        Token(TokenKind::SPACE, " ", 1, 67),
        Token(TokenKind::IDENTIFIER, "age", 1, 68),
        Token(TokenKind::COLON, ":", 1, 71),
        Token(TokenKind::SPACE, " ", 1, 72),
        Token(TokenKind::IDENTIFIER, "Int", 1, 73),
        Token(TokenKind::DOT, ".", 1, 76),
        Token(TokenKind::IDENTIFIER, "I32", 1, 77),
        Token(TokenKind::L_PAREN, "(", 1, 80),
        Token(TokenKind::NUMBER_LITERAL, "42", 1, 81),
        Token(TokenKind::R_PAREN, ")", 1, 83),
        Token(TokenKind::COMMA, ",", 1, 84),
        Token(TokenKind::SPACE, " ", 1, 85),
        Token(TokenKind::IDENTIFIER, "color", 1, 86),
        Token(TokenKind::COLON, ":", 1, 91),
        Token(TokenKind::SPACE, " ", 1, 92),
        Token(TokenKind::IDENTIFIER, "Color", 1, 93),
        Token(TokenKind::DOT, ".", 1, 98),
        Token(TokenKind::IDENTIFIER, "RED", 1, 99),
        Token(TokenKind::R_PAREN, ")", 1, 102),
    };

    std::vector<Token> tokens;
    Token token = lexer.next_token();
    while (token.kind != TokenKind::END_OF_FILE)
    {
        tokens.push_back(token);
        token = lexer.next_token();
    }

    bool success = true;
    if (tokens.size() != expected_tokens.size())
    {
        success = false;
    }
    else
    {
        for (size_t i = 0; i < tokens.size(); ++i)
        {
            if (tokens[i].kind != expected_tokens[i].kind || tokens[i].value != expected_tokens[i].value)
            {
                success = false;
                break;
            }
        }
    }

    if (success)
    {
        std::cout << "Test passed!" << std::endl;
    }
    else
    {
        std::cout << "Test failed!" << std::endl;
        std::cout << "Expected tokens:" << std::endl;
        for (const auto &token : expected_tokens)
        {
            std::cout << "  " << static_cast<int>(token.kind) << " " << token.value << std::endl;
        }
        std::cout << "Actual tokens:" << std::endl;
        for (const auto &token : tokens)
        {
            std::cout << "  " << static_cast<int>(token.kind) << " " << token.value << std::endl;
        }
    }

    return 0;
}


#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>
#include <cctype>
#include <stdexcept>

enum class TokenKind
{
    // Literals
    STRING_LITERAL,
    NUMBER_LITERAL,
    IDENTIFIER,

    // Format
    TAB,
    NEWLINE,
    SPACE,
    COMMA,

    // Unary
    NOT,
    MINUS,
    PLUS,
    ELIPSIS,

    // Binary
    COLON,
    DOT,
    EQUAL,
    NOT_EQUAL,
    LESS,
    LESS_EQUAL,
    GREATER,
    GREATER_EQUAL,
    AND,
    OR,
    STAR,
    SLASH,
    PERCENT,
    BIT_AND,
    BIT_OR,
    BIT_XOR,
    BIT_NOT,
    BIT_LSHIFT,
    BIT_RSHIFT,

    // Grouping
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,

    // Comment
    COMMENT,

    END_OF_FILE
};

class Token
{
public:
    TokenKind kind;
    std::string value;
    size_t line;
    size_t column;

    Token(TokenKind kind, const std::string &value, size_t line, size_t column) : kind(kind), value(value), line(line), column(column) {}
};

class LexerError : public std::runtime_error
{
public:
    explicit LexerError(const std::string &message) : std::runtime_error(message) {}
};
class Lexer
{
public:
    explicit Lexer(const std::string &input) : input(input), current_position(0), line(1), column(1) {}

    std::string input;
    size_t current_position;
    size_t line;
    size_t column;

    void advance(size_t steps = 1)
    {
        for (size_t i = 0; i < steps; ++i)
        {
            if (input[current_position] == '\n')
            {
                line++;
                column = 1;
            }
            else
            {
                column++;
            }

            current_position++;
        }
    }
    Token next_token()
    {
        // The main loop for the lexer
        while (current_position < input.length())
        {
            char current_char = input[current_position];
            switch (current_char)
            {
            // Handle whitespace
            case ' ':
                advance();
                return Token(TokenKind::SPACE, " ", line, column);
            case '\t':
                advance();
                return Token(TokenKind::TAB, "\t", line, column);
            case '\n':
                advance();
                return Token(TokenKind::NEWLINE, "\n", line, column);

            // Handle other characters and tokens
            case ',':
                advance();
                return Token(TokenKind::COMMA, ",", line, column);
            case ':':
                advance();
                return Token(TokenKind::COLON, ":", line, column);
            case '.':
                advance();
                return Token(TokenKind::DOT, ".", line, column);
            case '(':
                advance();
                return Token(TokenKind::L_PAREN, "(", line, column);
            case ')':
                advance();
                return Token(TokenKind::R_PAREN, ")", line, column);
            case '{':
                advance();
                return Token(TokenKind::L_CURLY, "{", line, column);
            case '}':
                advance();
                return Token(TokenKind::R_CURLY, "}", line, column);
            case '!':
                if (peek() == '=')
                {
                    advance(2);
                    return Token(TokenKind::NOT_EQUAL, "!=", line, column);
                }
                advance();
                return Token(TokenKind::NOT, "!", line, column);
            case '-':
                advance();
                return Token(TokenKind::MINUS, "-", line, column);
            case '+':
                advance();
                return Token(TokenKind::PLUS, "+", line, column);
            case '*':
                advance();
                return Token(TokenKind::STAR, "*", line, column);
            case '/':
                if (peek() == '/')
                {
                    advance(2);
                    return lex_comment();
                }
                advance();
                return Token(TokenKind::SLASH, "/", line, column);
            case '%':
                advance();
                return Token(TokenKind::PERCENT, "%", line, column);
            case '&':
                if (peek() == '&')
                {
                    advance(2);
                    return Token(TokenKind::AND, "&&", line, column);
                }
                advance();
                return Token(TokenKind::BIT_AND, "&", line, column);
            case '|':
                if (peek() == '|')
                {
                    advance(2);
                    return Token(TokenKind::OR, "||", line, column);
                }
                advance();
                return Token(TokenKind::BIT_OR, "|", line, column);
            case '^':
                advance();
                return Token(TokenKind::BIT_XOR, "^", line, column);
            case '~':
                advance();
                return Token(TokenKind::BIT_NOT, "~", line, column);
            case '<':
                if (peek() == '<')
                {
                    advance(2);
                    return Token(TokenKind::BIT_LSHIFT, "<<", line, column);
                }
                else if (peek() == '=')
                {
                    advance(2);
                    return Token(TokenKind::LESS_EQUAL, "<=", line, column);
                }
                advance();
                return Token(TokenKind::LESS, "<", line, column);
            case '>':
                if (peek() == '>')
                {
                    advance(2);
                    return Token(TokenKind::BIT_RSHIFT, ">>", line, column);
                }
                else if (peek() == '=')
                {
                    advance(2);
                    return Token(TokenKind::GREATER_EQUAL, ">=", line, column);
                }
                advance();
                return Token(TokenKind::GREATER, ">", line, column);
            case '=':
                if (peek() == '=')
                {
                    advance(2);
                    return Token(TokenKind::EQUAL, "==", line, column);
                }

                // Single '=' is not a valid token in the provided language example
                throw LexerError("Invalid character: " + std::string(1, current_char));
            case '"':
                return lex_string();
            default:
                if (isalpha(current_char) || current_char == '_')
                {
                    return lex_identifier();
                }
                else if (isdigit(current_char))
                {
                    return lex_number();
                }
                else
                {
                    throw LexerError("Invalid character: " + std::string(1, current_char));
                }
            }
        }

        return Token(TokenKind::END_OF_FILE, "", line, column);
    }
    char peek(size_t offset = 1) const
    {
        if (current_position + offset >= input.length())
        {
            return '\0';
        }
        return input[current_position + offset];
    }
    Token lex_string() {
        advance();
        int string_start_pos = current_position;
        Token token = Token(TokenKind::STRING_LITERAL, "", line, column);
        // if \" skip 2 characters
        while (current_position < input.length() && input[current_position] != '"')
        {
            if (input[current_position] == '\\' && input[current_position + 1] == '"')
            {
                advance(2);
            }
            else
            {
                advance();
            }
        }
        int length = current_position - string_start_pos;
        std::string string_literal = input.substr(string_start_pos, length);
        token = Token(TokenKind::STRING_LITERAL, string_literal, line, column);
        advance(2);
        return token;
    }
    Token lex_comment()
    {
        int comment_start_pos = current_position;
        Token token = Token(TokenKind::COMMENT, "", line, column);

        while (current_position < input.length() && input[current_position] != '\n')
        {
            advance();
        }
        int length = current_position - comment_start_pos + 1;
        std::string comment = input.substr(comment_start_pos, length );
        token = Token(TokenKind::COMMENT, comment, line, column);
        advance();
        return token;
    }

    Token lex_identifier()
    {
        Token token = Token(TokenKind::IDENTIFIER, "", line, column);
        int identifier_start_pos = current_position;
        while (isalnum(peek()) || peek() == '_')
        {
            advance();
        }
        // advance();
        int length = current_position - identifier_start_pos + 1;
        std::string identifier = input.substr(identifier_start_pos, length );
        token = Token(TokenKind::IDENTIFIER, identifier, line, column);
        advance();
        return token;
    }

    Token lex_number()
    {
        // parse number
        // 10 -> int
        // 10e10 -> int
        // 10.0 -> float
        // 10.0e10 -> float
        // 0x10 -> int
        // 0b10 -> int
        // 0o10 -> int

        int number_start_pos = current_position;
        advance();

        // check for hex, oct, bin
        if (peek() == 'x' || peek() == 'o' || peek() == 'b')
        {
            advance();
        }
        // consume until not in {0-9, ., e, E}
        while (isdigit(peek()) || peek() == '.' || peek() == 'e')
        {
            advance();
        }
        // we can change the INT, HEX,  this in the parser
        int length = current_position - number_start_pos;
        std::string numberString = input.substr(number_start_pos, length);
        Token token = Token(TokenKind::NUMBER_LITERAL, numberString, line, column);
        advance();
        return token;
    };
};
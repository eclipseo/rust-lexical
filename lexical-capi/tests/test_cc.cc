/**
 *  Unittests for the C++ API to lexical-core.
 *
 *  License
 *  -------
 *
 *  This is free and unencumbered software released into the public domain.
 *
 *  Anyone is free to copy, modify, publish, use, compile, sell, or
 *  distribute this software, either in source code form or as a compiled
 *  binary, for any purpose, commercial or non-commercial, and by any
 *  means.
 *
 *  In jurisdictions that recognize copyright laws, the author or authors
 *  of this software dedicate any and all copyright interest in the
 *  software to the public domain. We make this dedication for the benefit
 *  of the public at large and to the detriment of our heirs and
 *  successors. We intend this dedication to be an overt act of
 *  relinquishment in perpetuity of all present and future rights to this
 *  software under copyright law.
 *
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 *  EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 *  MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 *  IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 *  OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 *  ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 *  OTHER DEALINGS IN THE SOFTWARE.
 *
 *  For more information, please refer to <http://unlicense.org/>
 */

#include <gtest/gtest.h>
#include "lexical.hpp"

using namespace lexical;

// HELPERS
// -------

template <typename T>
inline result<T> result_ok(T value)
{
    // Initialize the union.
    result_union<T> u;
    u.value = value;

    // Create the tagged union.
    result<T> r;
    r.tag = result_tag::ok;
    r.data = u;

    return r;
}

template <typename T>
inline result<T> result_err(error_code code, size_t index)
{
    // Initialize the error.
    error e;
    e.code = code;
    e.index = index;

    // Initialize the union.
    result_union<T> u;
    u.error = e;

    // Create the tagged union.
    result<T> r;
    r.tag = result_tag::err;
    r.data = u;

    return r;
}

template <typename T>
inline result<T> result_overflow(size_t index)
{
    return result_err<T>(error_code::overflow, index);
}

template <typename T>
inline result<T> result_underflow(size_t index)
{
    return result_err<T>(error_code::underflow, index);
}

template <typename T>
inline result<T> result_invalid_digit(size_t index)
{
    return result_err<T>(error_code::invalid_digit, index);
}

template <typename T>
inline result<T> result_empty(size_t index)
{
    return result_err<T>(error_code::empty, index);
}

template <typename T>
inline result<T> result_empty_fraction(size_t index)
{
    return result_err<T>(error_code::empty_fraction, index);
}

template <typename T>
inline result<T> result_empty_exponent(size_t index)
{
    return result_err<T>(error_code::empty_exponent, index);
}

template <typename T>
inline partial_result<T> partial_result_ok(T value, size_t index)
{
    // Initialize the tuple.
    partial_result_tuple<T> t;
    t.x = value;
    t.y = index;

    // Initialize the union.
    partial_result_union<T> u;
    u.value = t;

    // Create the tagged union.
    partial_result<T> r;
    r.tag = result_tag::ok;
    r.data = u;

    return r;
}

template <typename T>
inline partial_result<T> partial_result_err(error_code code, size_t index)
{
    // Initialize the error.
    error e;
    e.code = code;
    e.index = index;

    // Initialize the union.
    partial_result_union<T> u;
    u.error = e;

    // Create the tagged union.
    partial_result<T> r;
    r.tag = result_tag::err;
    r.data = u;

    return r;
}

template <typename T>
inline partial_result<T> partial_result_overflow(size_t index)
{
    return partial_result_err<T>(error_code::overflow, index);
}

template <typename T>
inline partial_result<T> partial_result_underflow(size_t index)
{
    return partial_result_err<T>(error_code::underflow, index);
}

template <typename T>
inline partial_result<T> partial_result_invalid_digit(size_t index)
{
    return partial_result_err<T>(error_code::invalid_digit, index);
}

template <typename T>
inline partial_result<T> partial_result_empty(size_t index)
{
    return partial_result_err<T>(error_code::empty, index);
}

template <typename T>
inline partial_result<T> partial_result_empty_fraction(size_t index)
{
    return partial_result_err<T>(error_code::empty_fraction, index);
}

template <typename T>
inline partial_result<T> partial_result_empty_exponent(size_t index)
{
    return partial_result_err<T>(error_code::empty_exponent, index);
}

// CONFIG TESTS
// ------------

TEST(test_get_exponent_default_char, config_tests)
{
    EXPECT_EQ(get_exponent_default_char(), 'e');
}

TEST(test_set_exponent_default_char, config_tests)
{
    set_exponent_default_char('e');
}

#ifdef HAVE_RADIX
    TEST(test_get_exponent_backup_char, config_tests)
    {
        EXPECT_EQ(get_exponent_backup_char(), '^');
    }

    TEST(test_set_exponent_backup_char, config_tests)
    {
        set_exponent_backup_char('^');
    }
#endif  // HAVE_RADIX

#ifdef HAVE_ROUNDING
    TEST(test_get_float_rounding, config_tests)
    {
        EXPECT_EQ(get_float_rounding(), rounding_kind::nearest_tie_even);
    }

    TEST(test_set_float_rounding, config_tests)
    {
        set_float_rounding(rounding_kind::nearest_tie_even);
    }
#endif  // HAVE_ROUNDING

TEST(test_get_nan_string, config_tests)
{
    EXPECT_EQ(get_nan_string(), "NaN");
}

TEST(test_set_nan_string, config_tests)
{
    set_nan_string("NaN");
}

TEST(test_get_inf_string, config_tests)
{
    EXPECT_EQ(get_inf_string(), "inf");
}

TEST(test_set_inf_string, config_tests)
{
    set_inf_string("inf");
}

TEST(test_get_infinity_string, config_tests)
{
    EXPECT_EQ(get_infinity_string(), "infinity");
}

TEST(test_set_infinity_string, config_tests)
{
    set_infinity_string("infinity");
}

// CONSTANT TESTS

TEST(test_size, constant_tests)
{
    // Check all the sizes can be used.
    EXPECT_GE(I8_FORMATTED_SIZE, 0);
    EXPECT_GE(I16_FORMATTED_SIZE, 0);
    EXPECT_GE(I32_FORMATTED_SIZE, 0);
    EXPECT_GE(I64_FORMATTED_SIZE, 0);
    EXPECT_GE(ISIZE_FORMATTED_SIZE, 0);
    EXPECT_GE(U8_FORMATTED_SIZE, 0);
    EXPECT_GE(U16_FORMATTED_SIZE, 0);
    EXPECT_GE(U32_FORMATTED_SIZE, 0);
    EXPECT_GE(U64_FORMATTED_SIZE, 0);
    EXPECT_GE(USIZE_FORMATTED_SIZE, 0);
    EXPECT_GE(F32_FORMATTED_SIZE, 0);
    EXPECT_GE(F64_FORMATTED_SIZE, 0);
    EXPECT_GE(I8_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(I16_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(I32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(I64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(ISIZE_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(U8_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(U16_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(U32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(U64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(USIZE_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(F32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(F64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(BUFFER_SIZE, 0);
}

// ERROR TESTS

TEST(test_is_overflow, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error underflow = { error_code::underflow, 0 };
    EXPECT_TRUE(overflow.is_overflow());
    EXPECT_FALSE(underflow.is_overflow());
}

TEST(test_is_underflow, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error underflow = { error_code::underflow, 0 };
    EXPECT_FALSE(overflow.is_underflow());
    EXPECT_TRUE(underflow.is_underflow());
}

TEST(test_is_invalid_digit, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error invalid_digit = { error_code::invalid_digit, 0 };
    EXPECT_FALSE(overflow.is_invalid_digit());
    EXPECT_TRUE(invalid_digit.is_invalid_digit());
}

TEST(test_is_empty, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error empty = { error_code::empty, 0 };
    EXPECT_FALSE(overflow.is_empty());
    EXPECT_TRUE(empty.is_empty());
}

TEST(test_is_empty_fraction, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error empty_fraction = { error_code::empty_fraction, 0 };
    EXPECT_FALSE(overflow.is_empty_fraction());
    EXPECT_TRUE(empty_fraction.is_empty_fraction());
}

TEST(test_is_empty_exponent, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error empty_exponent = { error_code::empty_exponent, 0 };
    EXPECT_FALSE(overflow.is_empty_exponent());
    EXPECT_TRUE(empty_exponent.is_empty_exponent());
}

// RESULT TESTS

TEST(result, result_tests)
{
    auto ok = result_ok<u8>(0);
    auto overflow = result_overflow<u8>(0);
    auto underflow = result_underflow<u8>(0);
    auto invalid_digit = result_invalid_digit<u8>(0);
    auto empty = result_empty<u8>(0);
    auto empty_fraction = result_empty_fraction<u8>(0);
    auto empty_exponent = result_empty_exponent<u8>(0);

    EXPECT_TRUE(ok.is_ok());
    EXPECT_FALSE(ok.is_err());
    EXPECT_TRUE(overflow.is_err());
    EXPECT_TRUE(underflow.is_err());
    EXPECT_TRUE(invalid_digit.is_err());
    EXPECT_TRUE(empty.is_err());
    EXPECT_TRUE(empty_fraction.is_err());
    EXPECT_TRUE(empty_exponent.is_err());

    EXPECT_EQ(ok.ok(), 0);
    EXPECT_TRUE(overflow.err().is_overflow());
    EXPECT_TRUE(underflow.err().is_underflow());
    EXPECT_TRUE(invalid_digit.err().is_invalid_digit());
    EXPECT_TRUE(empty.err().is_empty());
    EXPECT_TRUE(empty_fraction.err().is_empty_fraction());
    EXPECT_TRUE(empty_exponent.err().is_empty_exponent());
}

// PARTIAL RESULT TESTS

TEST(partial_result, partial_result_tests)
{
    auto ok = partial_result_ok<u8>(0, 1);
    auto overflow = partial_result_overflow<u8>(0);
    auto underflow = partial_result_underflow<u8>(0);
    auto invalid_digit = partial_result_invalid_digit<u8>(0);
    auto empty = partial_result_empty<u8>(0);
    auto empty_fraction = partial_result_empty_fraction<u8>(0);
    auto empty_exponent = partial_result_empty_exponent<u8>(0);

    EXPECT_TRUE(ok.is_ok());
    EXPECT_FALSE(ok.is_err());
    EXPECT_TRUE(overflow.is_err());
    EXPECT_TRUE(underflow.is_err());
    EXPECT_TRUE(invalid_digit.is_err());
    EXPECT_TRUE(empty.is_err());
    EXPECT_TRUE(empty_fraction.is_err());
    EXPECT_TRUE(empty_exponent.is_err());

    EXPECT_EQ(ok.ok(), std::make_tuple(0, 1));
    EXPECT_TRUE(overflow.err().is_overflow());
    EXPECT_TRUE(underflow.err().is_underflow());
    EXPECT_TRUE(invalid_digit.err().is_invalid_digit());
    EXPECT_TRUE(empty.err().is_empty());
    EXPECT_TRUE(empty_fraction.err().is_empty_fraction());
    EXPECT_TRUE(empty_exponent.err().is_empty_exponent());
}

// TO STRING TESTS

#define TO_STRING_TEST(t)                                                       \
    EXPECT_EQ("10", to_string<t>(10))

#define TO_STRING_FLOAT_TEST(t)                                                 \
    EXPECT_EQ("10.5", to_string<t>(10.5))

TEST(to_string, api_tests)
{
    TO_STRING_TEST(u8);
    TO_STRING_TEST(u16);
    TO_STRING_TEST(u32);
    TO_STRING_TEST(u64);
    TO_STRING_TEST(usize);
    TO_STRING_TEST(i8);
    TO_STRING_TEST(i16);
    TO_STRING_TEST(i32);
    TO_STRING_TEST(i64);
    TO_STRING_TEST(isize);
    TO_STRING_FLOAT_TEST(f32);
    TO_STRING_FLOAT_TEST(f64);
}

#ifdef HAVE_RADIX
    #define TO_STRING_RADIX_TEST(t)                                             \
        EXPECT_EQ("1010", to_string_radix<t>(10, 2));                           \
        EXPECT_EQ("A", to_string_radix<t>(10, 16));                             \
        EXPECT_EQ("10", to_string_radix<t>(10, 10))

    #define TO_STRING_RADIX_FLOAT_TEST(t)                                       \
        EXPECT_EQ("1010.1", to_string_radix<t>(10.5, 2));                       \
        EXPECT_EQ("A.8", to_string_radix<t>(10.5, 16));                         \
        EXPECT_EQ("10.5", to_string_radix<t>(10.5, 10))

    TEST(to_string_radix, api_tests)
    {
        TO_STRING_RADIX_TEST(u8);
        TO_STRING_RADIX_TEST(u16);
        TO_STRING_RADIX_TEST(u32);
        TO_STRING_RADIX_TEST(u64);
        TO_STRING_RADIX_TEST(usize);
        TO_STRING_RADIX_TEST(i8);
        TO_STRING_RADIX_TEST(i16);
        TO_STRING_RADIX_TEST(i32);
        TO_STRING_RADIX_TEST(i64);
        TO_STRING_RADIX_TEST(isize);
        TO_STRING_RADIX_FLOAT_TEST(f32);
        TO_STRING_RADIX_FLOAT_TEST(f64);
    }
#endif  // HAVE_RADIX

// PARSE TESTS

#define PARSE_TEST(t)                                                           \
    EXPECT_EQ(result_ok<t>(10), parse<t>("10"));                                \
    EXPECT_EQ(result_invalid_digit<t>(2), parse<t>("10a"));                     \
    EXPECT_EQ(result_empty<t>(0), parse<t>(""))

#define PARSE_FLOAT_TEST(t)                                                     \
    PARSE_TEST(t);                                                              \
    EXPECT_EQ(result_ok<t>(10.5), parse<t>("10.5"));                            \
    EXPECT_EQ(result_ok<t>(10e5), parse<t>("10e5"));                            \
    EXPECT_EQ(result_empty_fraction<t>(0), parse<t>("."));                      \
    EXPECT_EQ(result_empty_fraction<t>(0), parse<t>("e5"));                     \
    EXPECT_EQ(result_empty_exponent<t>(4), parse<t>("10e+"))

TEST(parse, api_tests)
{
    PARSE_TEST(u8);
    PARSE_TEST(u16);
    PARSE_TEST(u32);
    PARSE_TEST(u64);
    PARSE_TEST(usize);
    PARSE_TEST(i8);
    PARSE_TEST(i16);
    PARSE_TEST(i32);
    PARSE_TEST(i64);
    PARSE_TEST(isize);
    PARSE_FLOAT_TEST(f32);
    PARSE_FLOAT_TEST(f64);
}

#ifdef HAVE_RADIX
    #define PARSE_RADIX_TEST(t)                                                 \
        EXPECT_EQ(result_ok<t>(10), parse_radix<t>("1010", 2));                 \
        EXPECT_EQ(result_ok<t>(10), parse_radix<t>("10", 10));                  \
        EXPECT_EQ(result_ok<t>(10), parse_radix<t>("A", 16));                   \
        EXPECT_EQ(result_invalid_digit<t>(4), parse_radix<t>("10102", 2));      \
        EXPECT_EQ(result_invalid_digit<t>(2), parse_radix<t>("10a", 10));       \
        EXPECT_EQ(result_invalid_digit<t>(1), parse_radix<t>("AG", 16));        \
        EXPECT_EQ(result_empty<t>(0), parse_radix<t>("", 10))

    #define PARSE_RADIX_FLOAT_TEST(t)                                           \
        PARSE_RADIX_TEST(t);                                                    \
        EXPECT_EQ(result_ok<t>(10.5), parse_radix<t>("1010.1", 2));             \
        EXPECT_EQ(result_ok<t>(10.5), parse_radix<t>("10.5", 10));              \
        EXPECT_EQ(result_ok<t>(10.5), parse_radix<t>("A.8", 16));               \
        EXPECT_EQ(result_empty_fraction<t>(0), parse_radix<t>(".", 10));        \
        EXPECT_EQ(result_empty_fraction<t>(0), parse_radix<t>("e5", 10));       \
        EXPECT_EQ(result_empty_exponent<t>(4), parse_radix<t>("10e+", 10))

    TEST(parse_radix, api_tests)
    {
        PARSE_RADIX_TEST(u8);
        PARSE_RADIX_TEST(u16);
        PARSE_RADIX_TEST(u32);
        PARSE_RADIX_TEST(u64);
        PARSE_RADIX_TEST(usize);
        PARSE_RADIX_TEST(i8);
        PARSE_RADIX_TEST(i16);
        PARSE_RADIX_TEST(i32);
        PARSE_RADIX_TEST(i64);
        PARSE_RADIX_TEST(isize);
        PARSE_RADIX_FLOAT_TEST(f32);
        PARSE_RADIX_FLOAT_TEST(f64);
    }
#endif  // HAVE_RADIX

#define PARSE_PARTIAL_TEST(t)                                                   \
    EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial<t>("10"));             \
    EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial<t>("10a"));            \
    EXPECT_EQ(partial_result_empty<t>(0), parse_partial<t>(""))

#define PARSE_PARTIAL_FLOAT_TEST(t)                                             \
    PARSE_PARTIAL_TEST(t);                                                      \
    EXPECT_EQ(partial_result_ok<t>(10.5, 4), parse_partial<t>("10.5"));         \
    EXPECT_EQ(partial_result_ok<t>(10e5, 4), parse_partial<t>("10e5"));         \
    EXPECT_EQ(partial_result_empty_fraction<t>(0), parse_partial<t>("."));      \
    EXPECT_EQ(partial_result_empty_fraction<t>(0), parse_partial<t>("e5"));     \
    EXPECT_EQ(partial_result_empty_exponent<t>(4), parse_partial<t>("10e+"))

TEST(parse_partial, api_tests)
{
    PARSE_PARTIAL_TEST(u8);
    PARSE_PARTIAL_TEST(u16);
    PARSE_PARTIAL_TEST(u32);
    PARSE_PARTIAL_TEST(u64);
    PARSE_PARTIAL_TEST(usize);
    PARSE_PARTIAL_TEST(i8);
    PARSE_PARTIAL_TEST(i16);
    PARSE_PARTIAL_TEST(i32);
    PARSE_PARTIAL_TEST(i64);
    PARSE_PARTIAL_TEST(isize);
    PARSE_PARTIAL_FLOAT_TEST(f32);
    PARSE_PARTIAL_FLOAT_TEST(f64);
}

#ifdef HAVE_RADIX
    #define PARSE_PARTIAL_RADIX_TEST(t)                                                     \
        EXPECT_EQ(partial_result_ok<t>(10, 4), parse_partial_radix<t>("1010", 2));          \
        EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_radix<t>("10", 10));           \
        EXPECT_EQ(partial_result_ok<t>(10, 1), parse_partial_radix<t>("A", 16));            \
        EXPECT_EQ(partial_result_ok<t>(10, 4), parse_partial_radix<t>("10102", 2));         \
        EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_radix<t>("10a", 10));          \
        EXPECT_EQ(partial_result_ok<t>(10, 1), parse_partial_radix<t>("AG", 16));           \
        EXPECT_EQ(partial_result_empty<t>(0), parse_partial_radix<t>("", 10))

    #define PARSE_PARTIAL_RADIX_FLOAT_TEST(t)                                               \
        PARSE_PARTIAL_RADIX_TEST(t);                                                        \
        EXPECT_EQ(partial_result_ok<t>(10.5, 6), parse_partial_radix<t>("1010.1", 2));      \
        EXPECT_EQ(partial_result_ok<t>(10.5, 4), parse_partial_radix<t>("10.5", 10));       \
        EXPECT_EQ(partial_result_ok<t>(10.5, 3), parse_partial_radix<t>("A.8", 16));        \
        EXPECT_EQ(partial_result_empty_fraction<t>(0), parse_partial_radix<t>(".", 10));    \
        EXPECT_EQ(partial_result_empty_fraction<t>(0), parse_partial_radix<t>("e5", 10));   \
        EXPECT_EQ(partial_result_empty_exponent<t>(4), parse_partial_radix<t>("10e+", 10))

    TEST(parse_partial_radix, api_tests)
    {
        PARSE_PARTIAL_RADIX_TEST(u8);
        PARSE_PARTIAL_RADIX_TEST(u16);
        PARSE_PARTIAL_RADIX_TEST(u32);
        PARSE_PARTIAL_RADIX_TEST(u64);
        PARSE_PARTIAL_RADIX_TEST(usize);
        PARSE_PARTIAL_RADIX_TEST(i8);
        PARSE_PARTIAL_RADIX_TEST(i16);
        PARSE_PARTIAL_RADIX_TEST(i32);
        PARSE_PARTIAL_RADIX_TEST(i64);
        PARSE_PARTIAL_RADIX_TEST(isize);
        PARSE_PARTIAL_RADIX_FLOAT_TEST(f32);
        PARSE_PARTIAL_RADIX_FLOAT_TEST(f64);
    }
#endif  // HAVE_RADIX

// PARSE LOSSY TESTS

#define PARSE_LOSSY_TEST(t)                                                     \
    EXPECT_EQ(result_ok<t>(10), parse_lossy<t>("10"));                          \
    EXPECT_EQ(result_invalid_digit<t>(2), parse_lossy<t>("10a"));               \
    EXPECT_EQ(result_empty<t>(0), parse_lossy<t>(""))

#define PARSE_LOSSY_FLOAT_TEST(t)                                               \
    PARSE_LOSSY_TEST(t);                                                        \
    EXPECT_EQ(result_ok<t>(10.5), parse_lossy<t>("10.5"));                      \
    EXPECT_EQ(result_ok<t>(10e5), parse_lossy<t>("10e5"));                      \
    EXPECT_EQ(result_empty_fraction<t>(0), parse_lossy<t>("."));                \
    EXPECT_EQ(result_empty_fraction<t>(0), parse_lossy<t>("e5"));               \
    EXPECT_EQ(result_empty_exponent<t>(4), parse_lossy<t>("10e+"))

TEST(parse_lossy, api_tests)
{
    PARSE_LOSSY_FLOAT_TEST(f32);
    PARSE_LOSSY_FLOAT_TEST(f64);
}

#ifdef HAVE_RADIX
    #define PARSE_LOSSY_RADIX_TEST(t)                                               \
        EXPECT_EQ(result_ok<t>(10), parse_lossy_radix<t>("10", 10));                \
        EXPECT_EQ(result_invalid_digit<t>(2), parse_lossy_radix<t>("10a", 10));     \
        EXPECT_EQ(result_empty<t>(0), parse_lossy_radix<t>("", 10))

    #define PARSE_LOSSY_RADIX_FLOAT_TEST(t)                                         \
        PARSE_LOSSY_TEST(t);                                                        \
        EXPECT_EQ(result_ok<t>(10.5), parse_lossy_radix<t>("10.5", 10));            \
        EXPECT_EQ(result_ok<t>(10e5), parse_lossy_radix<t>("10e5", 10));            \
        EXPECT_EQ(result_empty_fraction<t>(0), parse_lossy_radix<t>(".", 10));      \
        EXPECT_EQ(result_empty_fraction<t>(0), parse_lossy_radix<t>("e5", 10));     \
        EXPECT_EQ(result_empty_exponent<t>(4), parse_lossy_radix<t>("10e+", 10))

    TEST(parse_lossy_radix, api_tests)
    {
        PARSE_LOSSY_RADIX_FLOAT_TEST(f32);
        PARSE_LOSSY_RADIX_FLOAT_TEST(f64);
    }
#endif // HAVE_RADIX

#define PARSE_PARTIAL_LOSSY_TEST(t)                                                         \
    EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_lossy<t>("10"));                   \
    EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_lossy<t>("10a"));                  \
    EXPECT_EQ(partial_result_empty<t>(0), parse_partial_lossy<t>(""))

#define PARSE_PARTIAL_LOSSY_FLOAT_TEST(t)                                                   \
    PARSE_PARTIAL_LOSSY_TEST(t);                                                            \
    EXPECT_EQ(partial_result_ok<t>(10.5, 4), parse_partial_lossy<t>("10.5"));               \
    EXPECT_EQ(partial_result_ok<t>(10e5, 4), parse_partial_lossy<t>("10e5"));               \
    EXPECT_EQ(partial_result_empty_fraction<t>(0), parse_partial_lossy<t>("."));            \
    EXPECT_EQ(partial_result_empty_fraction<t>(0), parse_partial_lossy<t>("e5"));           \
    EXPECT_EQ(partial_result_empty_exponent<t>(4), parse_partial_lossy<t>("10e+"))

TEST(parse_partial_lossy, api_tests)
{
    PARSE_PARTIAL_LOSSY_FLOAT_TEST(f32);
    PARSE_PARTIAL_LOSSY_FLOAT_TEST(f64);
}

#ifdef HAVE_RADIX
    #define PARSE_PARTIAL_LOSSY_RADIX_TEST(t)                                                   \
        EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_lossy_radix<t>("10", 10));         \
        EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_lossy_radix<t>("10a", 10));        \
        EXPECT_EQ(partial_result_empty<t>(0), parse_partial_lossy_radix<t>("", 10))

    #define PARSE_PARTIAL_LOSSY_RADIX_FLOAT_TEST(t)                                             \
        PARSE_PARTIAL_LOSSY_RADIX_TEST(t);                                                      \
        EXPECT_EQ(partial_result_ok<t>(10.5, 4), parse_partial_lossy_radix<t>("10.5", 10));     \
        EXPECT_EQ(partial_result_ok<t>(10e5, 4), parse_partial_lossy_radix<t>("10e5", 10));     \
        EXPECT_EQ(partial_result_empty_fraction<t>(0), parse_partial_lossy_radix<t>(".", 10));  \
        EXPECT_EQ(partial_result_empty_fraction<t>(0), parse_partial_lossy_radix<t>("e5", 10)); \
        EXPECT_EQ(partial_result_empty_exponent<t>(4), parse_partial_lossy_radix<t>("10e+", 10))

    TEST(parse_partial_lossy_radix, api_tests)
    {
        PARSE_PARTIAL_LOSSY_RADIX_FLOAT_TEST(f32);
        PARSE_PARTIAL_LOSSY_RADIX_FLOAT_TEST(f64);
    }
#endif // HAVE_RADIX

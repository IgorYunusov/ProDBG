#include <stdarg.h>
#include <stddef.h>
#include <setjmp.h>
#include <cmocka.h>
#include <stdio.h>
#include "api/include/pd_script.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// TODO: Add more tests

void test_script_create(void**)
{
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

int main()
{
    const UnitTest tests[] =
    {
        unit_test(test_script_create),
    };

    return run_tests(tests);
}


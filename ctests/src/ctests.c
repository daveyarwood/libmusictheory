#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "musictheory.h"

void run_test(bool (test)(), const char* name, bool* passed) {
  if (test()) {
    fprintf(stderr, "PASSED: %s\n\n", name);
  } else {
    *passed = false;
    fprintf(stderr, "FAILED: %s\n\n", name);
  }
}

char* test_name;
bool passed;

void testing(char* name) {
  test_name = name;
  passed = true;
}

void test_int_equal_impl(int expected, int actual, int lineno) {
  if (actual != expected) {
    fprintf(stderr, "[%s (%s:%d)] Expected %d, got %d.\n",
            test_name, __FILE__, lineno, expected, actual);
    passed = false;
  }
}

void test_str_equal_impl(void* expected, void* actual, int lineno) {
  if (strcmp(expected, actual) != 0) {
    fprintf(stderr, "[%s (%s:%d)] Expected %s, got %s.\n",
            test_name, __FILE__, lineno, expected, actual);
    passed = false;
  }
}

#define test_int_equal(e, a) test_int_equal_impl(e, a, __LINE__)
#define test_str_equal(e, a) test_str_equal_impl(e, a, __LINE__)

bool test_note_number() {
  testing("note_number");

  test_int_equal(61, musictheory_note_number("C#4"));

  return passed;
}

bool test_spell_note() {
  testing("spell_note");

  test_str_equal("C#4", musictheory_spell_note(61, 'C'));
  test_str_equal("Db4", musictheory_spell_note(61, 'D'));

  return passed;
}

int main() {
  bool passed = true;

  run_test(test_note_number, "test_note_number", &passed);
  run_test(test_spell_note, "test_spell_note", &passed);

  return passed ? 0 : 1;
}

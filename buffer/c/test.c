#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "buffer.h"

void equal(const char *a, const char *b)
{
  if (strcmp(a, b))
  {
    printf("\n");
    printf("  expected: '%s'\n", a);
    printf("    actual: '%s'\n", b);
    printf("\n");
    exit(1);
  }
}

void test_buffer_new()
{
  buffer_t *buf = buffer_new();
  assert(BUFFER_DEFAULT_SIZE == buffer_size(buf));
  assert(0 == buffer_length(buf));
  buffer_free(buf);
  printf("new ok\n");
}

void test_buffer_new_with_size()
{
  buffer_t *buf = buffer_new_with_size(1024);
  assert(1024 == buffer_size(buf));
  assert(0 == buffer_length(buf));
  buffer_free(buf);
  printf("new size ok\n");
}

void test_free(){
  buffer_t *buf = buffer_new_with_copy("Hello");
  equal("Hello", buffer_string(buf));
  buffer_free(buf);
  char* s;
  sprintf(s,"%s",(char*)buf);
  equal("",s);
  printf("free ok\n");
}

void test_buffer_append()
{
  buffer_t *buf = buffer_new();
  assert(0 == buffer_append(buf, "Hello"));
  assert(0 == buffer_append(buf, " World"));
  assert(strlen("Hello World") == buffer_length(buf));
  equal("Hello World", buffer_string(buf));
  buffer_free(buf);
  printf("append ok\n");
}

void test_buffer_append_n()
{
  buffer_t *buf = buffer_new();
  assert(0 == buffer_append_n(buf, "subway", 3));
  assert(0 == buffer_append_n(buf, "marines", 6));
  assert(strlen("submarine") == buffer_length(buf));
  equal("submarine", buffer_string(buf));
  buffer_free(buf);
  printf("append_n ok\n");
}

void test_buffer_append_grow()
{
  buffer_t *buf = buffer_new_with_size(10);
  assert(0 == buffer_append(buf, "Hello"));
  assert(0 == buffer_append(buf, " tobi"));
  assert(0 == buffer_append(buf, " was"));
  assert(0 == buffer_append(buf, " here"));

  char *str = "Hello tobi was here";
  equal(str, buffer_string(buf));
  assert(1024 == buffer_size(buf));
  assert(strlen(str) == buffer_length(buf));
  buffer_free(buf);
  printf("append_grow ok\n");
}

void test_buffer_prepend()
{
  buffer_t *buf = buffer_new();
  assert(0 == buffer_append(buf, " World"));
  assert(0 == buffer_prepend(buf, "Hello"));
  assert(strlen("Hello World") == buffer_length(buf));
  equal("Hello World", buffer_string(buf));
  buffer_free(buf);
  printf("prepend ok\n");
}

void test_buffer_slice()
{
  buffer_t *buf = buffer_new();
  buffer_append(buf, "Tobi Ferret");
  buffer_t *a = buffer_slice(buf, 2, 8);
  equal("Tobi Ferret", buffer_string(buf));
  equal("bi Fer", buffer_string(a));
  buffer_free(buf);
  buffer_free(a);
  printf("slice ok\n");
}

void test_buffer_slice_range_error()
{
  buffer_t *buf = buffer_new_with_copy("Tobi Ferret");
  buffer_t *a = buffer_slice(buf, 10, 2);
  assert(NULL == a);
  buffer_free(buf);
  printf("slice_range_error ok\n");
}

void test_buffer_slice_end()
{
  buffer_t *buf = buffer_new_with_copy("Tobi Ferret");
  buffer_t *a = buffer_slice(buf, 5, -1);
  equal("Tobi Ferret", buffer_string(buf));
  equal("Ferret", buffer_string(a));
  buffer_t *b = buffer_slice(buf, 5, -3);
  equal("Ferr", buffer_string(b));
  buffer_t *c = buffer_slice(buf, 8, -1);
  equal("ret", buffer_string(c));
  buffer_free(buf);
  buffer_free(a);
  buffer_free(b);
  buffer_free(c);
  printf("slice_range_end ok\n");
}

void test_buffer_slice_end_overflow()
{
  buffer_t *buf = buffer_new_with_copy("Tobi Ferret");
  buffer_t *a = buffer_slice(buf, 5, 1000);
  equal("Tobi Ferret", buffer_string(buf));
  equal("Ferret", buffer_string(a));
  buffer_free(a);
  buffer_free(buf);
  printf("slice_range_end_overflow ok\n");
}

void test_buffer_equals()
{
  buffer_t *a = buffer_new_with_copy("Hello");
  buffer_t *b = buffer_new_with_copy("Hello");
  assert(1 == buffer_equals(a, b));
  buffer_append(b, " World");
  assert(0 == buffer_equals(a, b));
  buffer_free(a);
  buffer_free(b);
  printf("equal ok\n");
}

void test_buffer_indexof()
{
  buffer_t *buf = buffer_new_with_copy("Tobi is a ferret");
  ssize_t i = buffer_indexof(buf, "is");
  assert(5 == i);
  i = buffer_indexof(buf, "a");
  assert(8 == i);
  i = buffer_indexof(buf, "something");
  assert(-1 == i);
  buffer_free(buf);
  printf("indexof ok\n");
}

void test_buffer_fill()
{
  buffer_t *buf = buffer_new_with_copy("Hello");
  assert(5 == buffer_length(buf));
  buffer_fill(buf, 0);
  assert(0 == buffer_length(buf));
  buffer_free(buf);
  printf("fill ok\n");
}

void test_buffer_clear()
{
  buffer_t *buf = buffer_new_with_copy("Hello");
  assert(5 == buffer_length(buf));
  buffer_clear(buf);
  assert(0 == buffer_length(buf));
  buffer_free(buf);
  printf("clear ok\n");
}

void test_buffer_trim()
{
  buffer_t *buf = buffer_new_with_copy("  Hello\n\n ");
  buffer_trim(buf);
  equal("Hello", buffer_string(buf));
  buffer_free(buf);
  buf = buffer_new_with_copy("  Hello\n\n ");
  buffer_trim_left(buf);
  equal("Hello\n\n ", buffer_string(buf));
  buffer_free(buf);
  buf = buffer_new_with_copy("  Hello\n\n ");
  buffer_trim_right(buf);
  equal("  Hello", buffer_string(buf));
  buffer_free(buf);
  printf("trim ok\n");
}

void test_buffer_compact()
{
  buffer_t *buf = buffer_new_with_copy("  Hello\n\n ");
  buffer_trim(buf);
  assert(5 == buffer_length(buf));
  assert(10 == buffer_size(buf));
  ssize_t removed = buffer_compact(buf);
  assert(5 == removed);
  assert(5 == buffer_length(buf));
  assert(5 == buffer_size(buf));
  equal("Hello", buffer_string(buf));
  buffer_free(buf);
  printf("compact ok\n");
}

int main()
{
  test_buffer_new();
  test_buffer_new_with_size();
  //test_free();
  test_buffer_append();
  test_buffer_append_n();
  test_buffer_append_grow();
  test_buffer_prepend();
  test_buffer_slice();
  test_buffer_slice_range_error();
  test_buffer_slice_end();
  test_buffer_slice_end_overflow();
  test_buffer_equals();
  test_buffer_indexof();
  test_buffer_fill();
  test_buffer_clear();
  test_buffer_trim();
  test_buffer_compact();
  printf("all test ok\n");
  return 0;
}

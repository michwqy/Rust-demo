
#include <sys/types.h>

#define BUFFER_DEFAULT_SIZE 64

typedef struct
{
  size_t len;
  char *alloc;
  char *data;
} buffer_t;

buffer_t *buffer_new();

buffer_t *buffer_new_with_size(size_t n);

buffer_t *buffer_new_with_copy(const char *str);

int buffer_append(buffer_t *self, const char *str);

int buffer_append_n(buffer_t *self, const char *str, size_t len);

int buffer_prepend(buffer_t *self, const char *str);

buffer_t *buffer_slice(buffer_t *self, size_t from, ssize_t to);

size_t buffer_size(buffer_t *self);

size_t buffer_length(buffer_t *self);

const char *buffer_string(buffer_t *self);

void buffer_free(buffer_t *self);

int buffer_equals(buffer_t *self, buffer_t *other);

ssize_t buffer_indexof(buffer_t *self, const char *str);

buffer_t *buffer_slice(buffer_t *self, size_t from, ssize_t to);

ssize_t buffer_compact(buffer_t *self);

void buffer_fill(buffer_t *self, char c);

void buffer_clear(buffer_t *self);

void buffer_trim_left(buffer_t *self);

void buffer_trim_right(buffer_t *self);

void buffer_trim(buffer_t *self);
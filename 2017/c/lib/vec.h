#ifndef VEC_H
#define VEC_H

// Hmm yeah should I be writing this in zig?

#define VEC(TYPE) struct __VEC_ ## TYPE

#define VEC_DEFINE_NEW(TYPE) VEC(TYPE) vec_ ## TYPE ## _new() { \
    VEC(TYPE) res = { \
        .buffer = NULL, \
        .len = 0, \
        .cap = 0, \
    }; \
    return res; \
}

#define VEC_DEFINE_PUSH(TYPE) void vec_ ## TYPE ## _push(VEC(TYPE) *v, TYPE item) { \
    if (v->len == v->cap) { \
        v->cap = v->cap == 0 ? 8 : v->cap * 2; \
        v->buffer = realloc(v->buffer, v->cap * sizeof(TYPE)); \
    } \
    v->buffer[v->len] = item; \
    v->len += 1; \
}

#define VEC_DEFINE_REMOVE(TYPE) void vec_ ## TYPE ## _remove(VEC(TYPE) *v, size_t index) { \
    for (size_t i = index + 1; i < v->len; v++) { \
        v->buffer[i - 1] = v->buffer[i]; \
    } \
    v->len -= 1; \
}

#define VEC_DEFINE_FREE(TYPE) void vec_ ## TYPE ## _free(VEC(TYPE) v) { \
    free(v.buffer); \
}

#define VEC_DEFINE_AT(TYPE) TYPE *vec_ ## TYPE ## _at(VEC(TYPE) v, size_t i) { \
    if (i < 0 || i >= v.len) { \
        return NULL; \
    } else { \
        return &v.buffer[i]; \
    } \
}

#define VEC_DEFINE(TYPE) VEC(TYPE)  { \
    TYPE *buffer; \
    size_t len; \
    size_t cap; \
}; \
VEC_DEFINE_NEW(TYPE); \
VEC_DEFINE_PUSH(TYPE); \
VEC_DEFINE_REMOVE(TYPE); \
VEC_DEFINE_FREE(TYPE); \
VEC_DEFINE_AT(TYPE);

#endif

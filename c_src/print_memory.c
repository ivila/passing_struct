#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct {
	/* Implementation defined */
	struct {
		int fd;
		bool reg_mem;
		bool memref_null;
	} imp;
} TEEC_Context;

typedef struct {
	/* Implementation defined */
	struct {
		TEEC_Context *ctx;
		uint32_t session_id;
	} imp;
} TEEC_Session;

void print_memory(uint32_t size, uint8_t* value) {
  volatile unsigned char *p = (volatile unsigned char *)value;
  for (uint32_t i = 0; i < size; i++) {
      uint8_t val = p[i];
      printf("i: %d, val: %d\n", i, val);
  }
}

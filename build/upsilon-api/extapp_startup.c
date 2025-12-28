#include <string.h>
#include <errno.h>
#include <setjmp.h>
#include "extapp_api.h"

void * __dso_handle = 0;

typedef void (*cxx_constructor)(void);

extern char _data_section_start_flash;
extern char _data_section_start_ram;
extern char _data_section_end_ram;
extern char _bss_section_start_ram;
extern char _bss_section_end_ram;
extern cxx_constructor _init_array_start;
extern cxx_constructor _init_array_end;

extern void extapp_main(void);

void *_heap_base, *_heap_ptr;
const void *_api_base;
uint32_t _heap_size;
jmp_buf oom_jmp_buf;

uint32_t _extapp_start(const uint32_t api_version, const void * api_base, void * heap, const uint32_t heap_size) {

  if(api_version != API_VERSION) {
    return 1;
  }

  size_t dataSectionLength = (&_data_section_end_ram - &_data_section_start_ram);
  memcpy(&_data_section_start_ram, &_data_section_start_flash, dataSectionLength);
  size_t bssSectionLength = (&_bss_section_end_ram - &_bss_section_start_ram);
  memset(&_bss_section_start_ram, 0, bssSectionLength);

  _api_base = api_base;
  _heap_base = heap;
  _heap_ptr = heap;
  _heap_size = heap_size;

  int result = setjmp(oom_jmp_buf);
  if(result == 0) {
    for (cxx_constructor * c = &_init_array_start; c<&_init_array_end; c++) {
      (*c)();
    }
    extapp_main();
  }

  return result;
}

void *_sbrk (int nbytes) {
  if ((_heap_ptr - _heap_base) + nbytes <= _heap_size) {
    void *base = _heap_ptr;
    _heap_ptr += nbytes;
    return base;
  } else {
    longjmp(oom_jmp_buf, 2);
  }
}

uint32_t (*entrypoint)(const uint32_t, const void *, void *, const uint32_t) __attribute__((section(".entrypoint"))) __attribute__((used)) = _extapp_start;

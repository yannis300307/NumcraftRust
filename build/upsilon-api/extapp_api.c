#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

extern void (* const *_api_base)(void);

uint64_t extapp_millis() {
  return ((uint64_t (*)(void))_api_base[0])();
}

void extapp_msleep(uint32_t ms) {
  ((void (*)(uint32_t))_api_base[1])(ms);
}

uint64_t extapp_scanKeyboard() {
  return ((uint64_t (*)(void))_api_base[2])();
}

void extapp_pushRect(int16_t x, int16_t y, uint16_t w, uint16_t h, const uint16_t * pixels) {
  ((void (*)(int16_t, int16_t, uint16_t, uint16_t, const uint16_t *))_api_base[3])(x, y, w, h, pixels);
}

void extapp_pushRectUniform(int16_t x, int16_t y, uint16_t w, uint16_t h, uint16_t color) {
  ((void (*)(int16_t, int16_t, uint16_t, uint16_t, uint16_t))_api_base[4])(x, y, w, h, color);
}

void extapp_pullRect(int16_t x, int16_t y, uint16_t w, uint16_t h, uint16_t * pixels) {
  ((void (*)(int16_t, int16_t, uint16_t, uint16_t, uint16_t *))_api_base[5])(x, y, w, h, pixels);
}

int16_t extapp_drawTextLarge(const char *text, int16_t x, int16_t y, uint16_t fg, uint16_t bg, bool fake) {
  return ((int16_t (*)(const char *, int16_t, int16_t, uint16_t, uint16_t, bool))_api_base[6])(text, x, y, fg, bg, fake);
}

int16_t extapp_drawTextSmall(const char *text, int16_t x, int16_t y, uint16_t fg, uint16_t bg, bool fake) {
  return ((int16_t (*)(const char *, int16_t, int16_t, uint16_t, uint16_t, bool))_api_base[7])(text, x, y, fg, bg, fake);
}

bool extapp_waitForVBlank(void) {
  return ((bool (*)(void))_api_base[8])();
}

void extapp_clipboardStore(const char *text) {
  ((void (*)(const char *))_api_base[9])(text);
}

const char * extapp_clipboardText(void) {
  return ((const char * (*)(void))_api_base[10])();
}

int extapp_fileListWithExtension(const char ** filenames, int maxrecords, const char * extension, int storage) {
  return ((int (*)(const char **, int, const char *, int))_api_base[11])(filenames, maxrecords, extension, storage);
}

bool extapp_fileExists(const char * filename, int storage) {
  return ((bool (*)(const char *, int))_api_base[12])(filename, storage);
}

bool extapp_fileErase(const char * filename, int storage) {
  return ((bool (*)(const char *, int))_api_base[13])(filename, storage);
}

const char * extapp_fileRead(const char * filename, size_t *len, int storage) {
  return ((const char * (*)(const char *, size_t *, int))_api_base[14])(filename, len, storage);
}

bool extapp_fileWrite(const char * filename, const char * content, size_t len, int storage) {
  return ((bool (*)(const char *, const char *, size_t, int))_api_base[15])(filename, content, len, storage);
}

void extapp_lockAlpha() {
  ((void (*)(void))_api_base[16])();
}

void extapp_resetKeyboard() {
  ((void (*)(void))_api_base[17])();
}

int extapp_getKey(bool allowSuspend, bool *alphaWasActive) {
  return ((int (*)(bool, bool *))_api_base[18])(allowSuspend, alphaWasActive);
}

bool extapp_isKeydown(int key){
  return ((bool (*)(int))_api_base[19])(key);
}

int extapp_restorebackup(int mode){
  return ((int (*)(int))_api_base[20])(mode);
}

bool extapp_erasesector(void * ptr){
  return ((bool (*)(void *))_api_base[21])(ptr);
}

bool extapp_writememory(unsigned char * dest,const unsigned char * data,size_t length){
  return ((bool (*)(unsigned char *, const unsigned char *, size_t))_api_base[22])(dest,data,length);
}

bool extapp_inexammode(){
  return ((bool (*)(void ))_api_base[23])();
}

// Code from https://framagit.org/Yaya.Cout/numworks-extapp-storage
#include <stdbool.h>
#include "storage.h"
#include <stdint.h>
#include <string.h>

// Taken from https://codereview.stackexchange.com/questions/151049/endianness-conversion-in-c/151070#151070
// I could convert the endianness manually, but it's less readable.
inline uint32_t reverse32(uint32_t value)
{
  return (((value & 0x000000FF) << 24) |
          ((value & 0x0000FF00) << 8) |
          ((value & 0x00FF0000) >> 8) |
          ((value & 0xFF000000) >> 24));
}

int strcmp(const char *s1, const char *s2)
{
  while (*s1 && (*s1 == *s2))
  {
    s1++;
    s2++;
  }
  return *(const unsigned char *)s1 - *(const unsigned char *)s2;
}

char *
strrchr(register const char *s, int c)
{
  char *rtnval = 0;

  do
  {
    if (*s == c)
      rtnval = (char *)s;
  } while (*s++);
  return (rtnval);
}

// This function takes extension for compatibility reasons, but ignores it
int extapp_fileList(const char **filename, int maxrecord, const char *extension)
{
  uint32_t storageAddress = extapp_address();
  char *offset = (char *)storageAddress;
  const char *endAddress = (char *)extapp_size() + storageAddress;

  if (!extapp_isValid((const uint32_t *)offset))
  {
    // Storage is invalid
    return -1;
  }

  offset += 4;
  int currentRecord = 0;

  while ((currentRecord < maxrecord) && offset < endAddress)
  {
    uint16_t size = *(uint16_t *)offset;
    if (size == 0)
    {
      break;
    }
    char *name = offset + 2;
    filename[currentRecord] = name;

    offset += size;
    currentRecord++;
  }

  return currentRecord;
}

int extapp_fileListWithExtension(const char **filename, int maxrecord, const char *extension_to_match)
{
  uint32_t storageAddress = extapp_address();
  char *offset = (char *)storageAddress;
  const char *endAddress = (char *)extapp_size() + storageAddress;

  if (!extapp_isValid((const uint32_t *)offset))
  {
    // Storage is invalid
    return -1;
  }

  offset += 4;
  int currentRecord = 0;

  while ((currentRecord < maxrecord) && offset < endAddress)
  {
    uint16_t size = *(uint16_t *)offset;
    if (size == 0)
    {
      break;
    }
    char *name = offset + 2;

    char *extension = strrchr(name, '.') + 1;
    if (strcmp(extension, extension_to_match) == 0)
    {
      filename[currentRecord] = name;
      currentRecord++;
    }

    offset += size;
  }

  return currentRecord;
}

bool extapp_fileExists(const char *filename)
{
  uint32_t storageAddress = extapp_address();
  char *offset = (char *)storageAddress;
  const char *endAddress = (char *)(extapp_size() + storageAddress);

  if (!extapp_isValid((const uint32_t *)offset))
  {
    // Storage is invalid
    return false;
  }

  offset += 4;
  int currentRecord = 0;

  while (offset < endAddress)
  {
    uint16_t size = *(uint16_t *)offset;
    if (size == 0)
    {
      break;
    }
    char *name = offset + 2;

    if (strcmp(name, filename) == 0)
    {
      // File was found
      return true;
    }

    offset += size;
    currentRecord++;
  }

  return false;
}

const char *extapp_fileRead(const char *filename, size_t *len)
{
  uint32_t storageAddress = extapp_address();
  char *offset = (char *)storageAddress;
  const char *endAddress = (char *)extapp_size() + storageAddress;

  if (!extapp_isValid((const uint32_t *)offset))
  {
    // Storage is invalid
    return NULL;
  }

  offset += 4;

  while (offset < endAddress)
  {
    uint16_t size = *(uint16_t *)offset;
    if (size == 0)
    {
      break;
    }
    char *name = offset + 2;

    if (strcmp(name, filename) == 0)
    {
      // filename + \0
      uint16_t nameSize = strlen(name) + 1;
      // Size contains size + filename + real content. Here, we only want the
      // content
      *len = size - 2 - nameSize;
      //     offset + size + filename
      return offset + 2 + nameSize;
    }

    offset += size;
  }

  // File not found
  return NULL;
}

bool extapp_fileWrite(const char *filename, const char *content, size_t len)
{
  // Check if we have enough free space
  const uint32_t *recordStartPointer = extapp_nextFree();
  if (recordStartPointer == 0x0)
  {
    // If recordStartPointer returns an error, the storage is invalid
    return false;
  }

  //                                                          Start Address  + size +     filename     + \0 + content
  const uint32_t *recordEndPointer = (uint32_t *)((char *)recordStartPointer + strlen(filename) + 1 + len);
  const uint32_t *storageEndPointer = (uint32_t *)(size_t)((void *)extapp_address() + extapp_size());

  // In case where we have overflown storage, we return an error
  if (storageEndPointer < recordEndPointer)
  {
    return false;
  }

  char *writableRecordStartPointer = (char *)extapp_nextFree();

  // We have enough storage, so we can write the data
  // Write size :
  // size + filename + \0 + content
  const uint16_t totalSize = 2 + strlen(filename) + 1 + len;
  *(uint16_t *)writableRecordStartPointer = totalSize;

  // Write filename:
  memcpy(writableRecordStartPointer + 2, filename, strlen(filename) + 1);

  // Write content:
  memcpy(writableRecordStartPointer + 2 + strlen(filename) + 1, content, len);

  // Overwrite the rest of the storage with zeroes
  memset(writableRecordStartPointer + 2 + strlen(filename) + 1 + len, 0, ((char *)extapp_address() + extapp_size()) - (writableRecordStartPointer + 2 + strlen(filename) + 1 + len));

  // The record is now written, so we can return
  return true;
}

bool extapp_fileErase(const char *filename)
{
  uint32_t storageAddress = extapp_address();
  char *offset = (char *)storageAddress;
  const char *endAddress = (char *)extapp_size() + storageAddress;

  if (!extapp_isValid((const uint32_t *)offset))
  {
    // Storage is invalid
    return false;
  }

  offset += 4;

  // Locate the record address
  char *recordAddress = NULL;
  while (offset < endAddress)
  {
    uint16_t size = *(uint16_t *)offset;
    if (size == 0)
    {
      break;
    }
    char *name = offset + 2;

    if (strcmp(name, filename) == 0)
    {
      recordAddress = offset;
      break;
    }

    offset += size;
  }

  // File not found
  if (recordAddress == NULL)
  {
    return false;
  }

  // Get the file size
  const uint16_t len = *(uint16_t *)offset;

  // Move the rest of the data
  char *nextFree = (char *)extapp_nextFree();
  memmove(offset, offset + len, nextFree - offset);

  // Overwrite the rest of the storage with zeroes
  memset(nextFree - len, 0, len);

  return true;
}

uint32_t extapp_address()
{
  return *(uint32_t *)((extapp_userlandAddress()) + 0xC);
}

const uint32_t extapp_size()
{
  return *(uint32_t *)((extapp_userlandAddress()) + 0x10);
}

const uint32_t *extapp_nextFree()
{
  uint32_t storageAddress = extapp_address();
  char *offset = (char *)storageAddress;
  const char *endAddress = (char *)extapp_size() + storageAddress;

  if (!extapp_isValid((const uint32_t *)offset))
  {
    // Storage is invalid
    return NULL;
  }

  offset += 4;

  while (offset < endAddress)
  {
    uint16_t size = *(uint16_t *)offset;
    if (size == 0)
    {
      // Here, we are at the place where new records should start
      return (const uint32_t *)offset;
    }

    offset += size;
  }

  // If we exited the loop, it mean that we have gone out of the storage
  return (uint32_t *)endAddress;
}

const uint32_t extapp_used()
{
  return (uint32_t)extapp_nextFree() - extapp_address();
}

bool extapp_isValid(const uint32_t *address)
{
  return *address == reverse32(0xBADD0BEE);
}

const uint8_t extapp_calculatorModel()
{
  // To guess the storage size without reading forbidden addresses, we try to
  // get the storage address from the userland header

  uint32_t *userlandMagicSlotAN0110 = *(uint32_t **)0x90010000;
  uint32_t *userlandMagicSlotBN0110 = *(uint32_t **)0x90410000;
  uint32_t *userlandMagicSlotAN0120 = *(uint32_t **)0x90020000;
  uint32_t *userlandMagicSlotBN0120 = *(uint32_t **)0x90420000;

  // On N0110, RAM start is at 0x20000000 and end is 0x20040000
  // On N0120, RAM start is at 0x20040000
  bool userlandMagicSlotAN0110IsValid = reverse32(0xfeedc0de) == (uint32_t)userlandMagicSlotAN0110;
  bool userlandMagicSlotBN0110IsValid = reverse32(0xfeedc0de) == (uint32_t)userlandMagicSlotBN0110;
  // TODO: Check the end address on N0120 (should be working, but good to check anyway)
  bool userlandMagicSlotAN0120IsValid = reverse32(0xfeedc0de) == (uint32_t)userlandMagicSlotAN0120;
  bool userlandMagicSlotBN0120IsValid = reverse32(0xfeedc0de) == (uint32_t)userlandMagicSlotBN0120;

  int N0110Counter = userlandMagicSlotAN0110IsValid + userlandMagicSlotBN0110IsValid;
  int N0120Counter = userlandMagicSlotAN0120IsValid + userlandMagicSlotBN0120IsValid;

  // At least one slot indicate N0110 and none N0120
  if ((N0110Counter > 0) && (N0120Counter == 0))
  {
    return 1;
  }

  // At least one slot indicate N0120 and none N0110
  if ((N0120Counter > 0) && (N0110Counter == 0))
  {
    return 2;
  }

  // In case where both matched, choose the one with most matches (for example,
  // if slot data made a false positive (should not happen unless someone flash
  // the wrong firmware on a calculator))
  if (N0110Counter > N0120Counter)
  {
    return 1;
  }
  if (N0120Counter > N0110Counter)
  {
    return 2;
  }

  // The remaining cases is equality (no match or as much matches). In both
  // cases, we cannot know
  return 0;
}

const uint32_t extapp_userlandAddress()
{
  // Get the model
  const uint8_t model = extapp_calculatorModel();

  if (model == 1)
  {
    return (*(uint32_t *)0x20000004) + 0x10000 - 0x8;
    // return *(uint32_t *)0x20000008;
  }
  if (model == 2)
  {
    return (*(uint32_t *)0x24000004) + 0x20000 - 0x8;
    // return *(uint32_t *)0x24000008;
  }

  // In case we couldn't determine the model, assume N0120 as it seems to be the
  // only model still produced
  return (*(uint32_t *)0x24000004) + 0x20000 - 0x8;
  // return *(uint32_t *)0x24000008;
}

CC=arm-none-eabi-gcc
OBJCOPY=arm-none-eabi-objcopy
AR=arm-none-eabi-ar
CFLAGS=-Os -mcpu=cortex-m7 -mthumb -mfpu=fpv5-sp-d16 -mfloat-abi=hard -fno-common -fdata-sections -ffunction-sections -fno-exceptions

BUILD_DIR=target/upsilon_api
SRCS_DIR=build/upsilon-api
SRCS=$(SRCS_DIR)/extapp_api.c $(SRCS_DIR)/extapp_startup.c
OBJS=$(SRCS:$(SRCS_DIR)/%.c=$(BUILD_DIR)/%.o)
NAME=target/upsilon_api/libapi.a


$(NAME): $(OBJS)
	$(AR) rcs $@ $^

$(OBJS): $(BUILD_DIR)/%.o: $(SRCS_DIR)/%.c
	$(CC) $(CFLAGS) -c $< -o $@

all: $(NAME)

.PHONY: all
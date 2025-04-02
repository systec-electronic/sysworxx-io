#***************************************************************************#
#                                                                           #
#  (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2       #
#  www.systec-electronic.com                                                #
#                                                                           #
#  File:         Makefile                                                   #
#  Description:  Makefile for CTR-700 Demo Application                      #
#                                                                           #
#  -----------------------------------------------------------------------  #
#                                                                           #
#  Revision History:                                                        #
#                                                                           #
#  2018/03/22 -ad:   Initial Version                                        #
#                                                                           #
#****************************************************************************

# --------- Project Settings ---------

ifeq ('$(TARGET_CFG)','')
#	TARGET_CFG	= RELEASE
	TARGET_CFG	= DEBUG
endif

# --------- Compile Settings ---------
EXEC		= demo
OBJS		= demo.o
LIBS		=
LDLIBS		= -lctr700drv \
			  -lsensors

INST_DIR	= /tftpboot/


INCLUDES	= -I. -I../../include
CDEFS		=

#           Select here between debug and release settings
ifeq ($(TARGET_CFG),RELEASE)
	DBG_MODE = NDEBUG
else
	DBG_MODE = _DEBUG
endif

#           C-Compiler settings
CDEFS		= -D$(DBG_MODE)

#           Toolchain command line settings
CFLAGS		+= $(ARM_CTR700_CFLAGS) -O0 -g -Wall -Wextra -Wno-pointer-sign -Wno-enum-compare $(INCLUDES) $(CDEFS)
LDFLAGS		+= $(ARM_CTR700_LDFLAGS)

CROSS		= $(ARM_CTR700_CC_PREFIX)
LD_LIB_PATH	=
CC			= $(CROSS)gcc
STRIP		= $(CROSS)strip
AR			= $(CROSS)ar

# --------- Default-Target ---------
all:		$(INST_DIR) print_settings $(EXEC) copy

# --------- Print Settings ---------
print_settings:
			@echo
			@echo "Make Settings"
			@echo "   CFLAGS  = '$(CFLAGS)'"
			@echo "   LDFLAGS = '$(LDFLAGS)'"
			@echo "   LDLIBS  = '$(LDLIBS)'"
			@echo

# --------- Compile single Source ---------

$(EXEC).o:	Makefile $(EXEC).c
			@echo "Compiling '$(notdir $*.c)'..."
			@$(CC) $(CFLAGS) -c $(notdir $*.c) -o $*.o

# --------- Link Executeable ---------
$(EXEC):	Makefile $(OBJS) $(LIBS)
			@echo "Linking '$(EXEC)'..."
			@$(CC) $(LDFLAGS) -o $@ $(OBJS) $(LIBS) $(LDLIBS)
ifeq ($(TARGET_CFG),RELEASE)
			@echo "Stripping '$(EXEC)'..."
			@$(STRIP) $@
endif
			@echo "Done."
			@echo

# --------- Install Executeable ---------
$(INST_DIR):
			@echo
			@echo "Create install directory '$(INST_DIR)'...   "
			mkdir -p $(INST_DIR)
			@echo "done."
			@echo

copy:		$(EXEC)
			@echo -n "Copy executeable '$(EXEC)' to destination '$(INST_DIR)':   "
			@cp $(EXEC) $(INST_DIR)
			@echo "done."
			@echo

# --------- Clean Project ---------
clean:
			rm -f *.bak
			rm -f *.tmp
#			rm -f $(EXEC)
			rm -f *.o
			rm -f *.elf
			rm -f *.gdb

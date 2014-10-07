#pragma once

#ifdef __cplusplus
extern "C" {
#endif

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

enum
{
    // File

    PRODBG_MENU_NEW = 0x1000,
    PRODBG_MENU_SUB_MENU,
    PRODBG_MENU_SEPARATOR,

    // Plugins

    PRODBG_MENU_SOURCECODE,
    PRODBG_MENU_DISASSEMBLY,
    PRODBG_MENU_LOCALS,
    PRODBG_MENU_WATCH,
    PRODBG_MENU_CALLSTACK,
    PRODBG_MENU_REGISTERS,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

enum
{
    PRODBG_KEY_COMMAND = 1 << 1,
    PRODBG_KEY_CTRL = 1 << 2,
    PRODBG_KEY_SHIFT = 1 << 3,
    PRODBG_KEY_ALT = 1 << 8,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct MenuDescriptor
{
    const char* name;
    int id;
    int key;
    int macMod;
    int winMod;
} MenuDescriptor;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern MenuDescriptor g_pluginsMenu[];

#ifdef __cplusplus
}
#endif

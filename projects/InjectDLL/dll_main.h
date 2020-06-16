#pragma once
#include <pe_module.h>
#include <constants.h>

extern InjectDLL::PEModule* csharp_lib;

void on_fixed_update(__int64 thisPointer);
std::string pretty_time();
bool is_in_shop_screen();
Game_Characters_StaticFields* get_characters();
Game_UI_c* get_UI();
SeinCharacter_o* get_sein();

void send_trace(MessageType type, int level, std::string const& group, std::string const& message);
void log(std::string message);
void error(std::string message);
void debug(std::string message);

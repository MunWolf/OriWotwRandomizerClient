#include <pch.h>
#include <interception_macros.h>
#include <dll_main.h>
#include <WinNetwork/ext.h>

#include <string>
#include <codecvt> 

BINDING(30218144, System_Char_array*, System_String__ToCharArray, (System_String_o* this_ptr))
BINDING(13878816, System_String_o*, MessageProvider__ToString, (MessageProvider_o* this_ptr))
BINDING(11718672, Quest_o*, QuestsController__GetQuestFromRuntimeQuest, (QuestsController_o* this_ptr, RuntimeQuest_o* quest))

void LogQuest(Quest_o* quest, std::string prefix)
{
    auto csharp_str = MessageProvider__ToString(quest->NameMessageProvider);
    auto str = convert_csstring(csharp_str);
    send_trace(MessageType::Debug, 5, "Quest", format("'%s: %s'", prefix.c_str(), str.c_str()));
}

INTERCEPT(12647040, void, RuntimeQuest__set_State, (RuntimeQuest_o* this_ptr, int32_t value), {
    RuntimeQuest__set_State(this_ptr, value);
    // Quest.QuestState Completed = 2;
    if (value == 2)
    {
        auto class_def = (*reinterpret_cast<QuestsController_c**>(resolve_rva(71355296)));
        auto quest_controller = class_def->static_fields->Instance;
        LogQuest(QuestsController__GetQuestFromRuntimeQuest(quest_controller, this_ptr), "Runtime");
    }
})

INTERCEPT(11715888, void, QuestsController__CompleteQuest, (QuestsController_o* this_ptr, Quest_o* quest), {
    QuestsController__CompleteQuest(this_ptr, quest);
    LogQuest(quest, "Controller");
})

#pragma once

#include "CoreMinimal.h"
#include "Modules/ModuleInterface.h"

/**
 * NoCheat module interface
 */
class NOCHEAT_API INoCheatModule : public IModuleInterface
{
public:
    /**
     * Singleton-like access to this module's interface
     * @return Returns singleton instance, loading the module on demand if needed
     */
    static inline INoCheatModule& Get()
    {
        return FModuleManager::LoadModuleChecked<INoCheatModule>("NoCheat");
    }

    /**
     * Checks to see if this module is loaded and ready
     * @return True if the module is loaded and ready to use
     */
    static inline bool IsAvailable()
    {
        return FModuleManager::Get().IsModuleLoaded("NoCheat");
    }
};
#include "CoreMinimal.h"
#include "Modules/ModuleManager.h"

// Define the third-party library module
class FNoCheatLibraryModule : public IModuleInterface
{
    virtual void StartupModule() override
    {
        // This code will execute after the module is loaded into memory
    }

    virtual void ShutdownModule() override
    {
        // This function may be called during shutdown to clean up the module
    }
};

IMPLEMENT_MODULE(FNoCheatLibraryModule, NoCheatLibrary);
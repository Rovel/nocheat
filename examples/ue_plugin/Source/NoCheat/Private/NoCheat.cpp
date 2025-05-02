#include "NoCheat.h"
#include "Modules/ModuleManager.h"

class FNoCheatModule : public IModuleInterface
{
public:
    /** IModuleInterface implementation */
    virtual void StartupModule() override
    {
        // This code will execute after your module is loaded into memory;
        // the exact timing is specified in the .uplugin file per-module
        
        // Load the NoCheat library when the module starts
        #if PLATFORM_WINDOWS
            const FString BaseDir = FPaths::Combine(*FPaths::ProjectPluginsDir(), TEXT("NoCheat"), TEXT("ThirdParty"), TEXT("NoCheatLib"), TEXT("lib"), TEXT("Win64"));
            const FString LibraryPath = FPaths::Combine(*BaseDir, TEXT("nocheat.dll"));
        #elif PLATFORM_MAC
            const FString BaseDir = FPaths::Combine(*FPaths::ProjectPluginsDir(), TEXT("NoCheat"), TEXT("ThirdParty"), TEXT("NoCheatLib"), TEXT("lib"), TEXT("Mac"));
            const FString LibraryPath = FPaths::Combine(*BaseDir, TEXT("libnocheat.dylib"));
        #elif PLATFORM_LINUX
            const FString BaseDir = FPaths::Combine(*FPaths::ProjectPluginsDir(), TEXT("NoCheat"), TEXT("ThirdParty"), TEXT("NoCheatLib"), TEXT("lib"), TEXT("Linux"));
            const FString LibraryPath = FPaths::Combine(*BaseDir, TEXT("libnocheat.so"));
        #endif

        NoCheatLibraryHandle = !LibraryPath.IsEmpty() ? FPlatformProcess::GetDllHandle(*LibraryPath) : nullptr;

        // Output an error if we couldn't load the library
        if (NoCheatLibraryHandle == nullptr)
        {
            UE_LOG(LogTemp, Error, TEXT("Failed to load NoCheat library from %s"), *LibraryPath);
        }
        else
        {
            UE_LOG(LogTemp, Log, TEXT("NoCheat library loaded successfully from %s"), *LibraryPath);
        }
    }

    virtual void ShutdownModule() override
    {
        // This function may be called during shutdown to clean up your module.
        // Free the dll handle
        if (NoCheatLibraryHandle != nullptr)
        {
            FPlatformProcess::FreeDllHandle(NoCheatLibraryHandle);
            NoCheatLibraryHandle = nullptr;
        }
    }

private:
    void* NoCheatLibraryHandle = nullptr;
};

IMPLEMENT_MODULE(FNoCheatModule, NoCheat)
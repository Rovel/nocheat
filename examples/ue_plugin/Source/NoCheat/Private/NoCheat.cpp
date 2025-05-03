#include "NoCheat.h"
#include "Modules/ModuleManager.h"

THIRD_PARTY_INCLUDES_START
#include "nocheat.h" // Include the third-party header
THIRD_PARTY_INCLUDES_END

class FNoCheatModule : public IModuleInterface
{
public:
    /** IModuleInterface implementation */
    virtual void StartupModule() override
    {
        // This code will execute after your module is loaded into memory
        
        // Load the NoCheat library when the module starts
        #if PLATFORM_WINDOWS
            const FString BaseDir = FPaths::Combine(*FPaths::ProjectPluginsDir(), TEXT("NoCheat"), TEXT("Source"), TEXT("ThirdParty"), TEXT("NoCheatLibrary"), TEXT("lib"), TEXT("Win64"));
            const FString LibraryPath = FPaths::Combine(*BaseDir, TEXT("nocheat.dll"));
        #elif PLATFORM_MAC
            const FString BaseDir = FPaths::Combine(*FPaths::ProjectPluginsDir(), TEXT("NoCheat"), TEXT("Source"), TEXT("ThirdParty"), TEXT("NoCheatLibrary"), TEXT("lib"), TEXT("Mac"));
            const FString LibraryPath = FPaths::Combine(*BaseDir, TEXT("libnocheat.dylib"));
        #elif PLATFORM_LINUX
            const FString BaseDir = FPaths::Combine(*FPaths::ProjectPluginsDir(), TEXT("NoCheat"), TEXT("Source"), TEXT("ThirdParty"), TEXT("NoCheatLibrary"), TEXT("lib"), TEXT("Linux"));
            const FString LibraryPath = FPaths::Combine(*BaseDir, TEXT("libnocheat.so"));
        #endif

        // Verify the file exists before trying to load it
        if (FPaths::FileExists(LibraryPath))
        {
            NoCheatLibraryHandle = FPlatformProcess::GetDllHandle(*LibraryPath);
            
            if (NoCheatLibraryHandle == nullptr)
            {
                UE_LOG(LogTemp, Error, TEXT("Failed to load NoCheat library from %s"), *LibraryPath);
            }
            else
            {
                UE_LOG(LogTemp, Log, TEXT("NoCheat library loaded successfully from %s"), *LibraryPath);
            }
        }
        else
        {
            UE_LOG(LogTemp, Error, TEXT("NoCheat library not found at %s"), *LibraryPath);
        }
    }

    virtual void ShutdownModule() override
    {
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
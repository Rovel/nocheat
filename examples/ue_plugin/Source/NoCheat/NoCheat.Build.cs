using UnrealBuildTool;
using System;
using System.IO;

public class NoCheat : ModuleRules
{
    public NoCheat(ReadOnlyTargetRules Target) : base(Target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;

        // Add the NoCheatLibrary module as a dependency
        PublicDependencyModuleNames.AddRange(new string[] {
            "Core",
            "CoreUObject",
            "Engine",
            "Json",
            "JsonUtilities"
        });

        // Add the NoCheatLibrary as a private dependency
        PrivateDependencyModuleNames.AddRange(new string[] { "NoCheatLibrary" });

        // We don't need these anymore as they're handled by the NoCheatLibrary module
        // But we do need to define the include path for header files
        PublicIncludePaths.Add(Path.Combine(ModuleDirectory, "../../ThirdParty/NoCheatLibrary/include"));
    }
}
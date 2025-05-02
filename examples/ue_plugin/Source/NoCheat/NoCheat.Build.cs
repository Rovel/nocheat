using UnrealBuildTool;
using System;
using System.IO;

public class NoCheat : ModuleRules
{
    private string ThirdPartyPath
    {
        get { return Path.GetFullPath(Path.Combine(ModuleDirectory, "../../ThirdParty/")); }
    }

    public NoCheat(ReadOnlyTargetRules Target) : base(Target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;

        PublicIncludePaths.Add(Path.Combine(ThirdPartyPath, "NoCheatLib/include"));

        PublicDependencyModuleNames.AddRange(new string[] {
            "Core",
            "CoreUObject",
            "Engine",
            "Json",
            "JsonUtilities"
        });

        if (Target.Platform == UnrealTargetPlatform.Win64)
        {
            PublicAdditionalLibraries.Add(Path.Combine(ThirdPartyPath, "NoCheatLib/lib/Win64/nocheat.lib"));
            PublicDelayLoadDLLs.Add("nocheat.dll");
            RuntimeDependencies.Add(Path.Combine("$(BinaryOutputDir)", "nocheat.dll"), 
                Path.Combine(ThirdPartyPath, "NoCheatLib/lib/Win64/nocheat.dll"));
        }
        else if (Target.Platform == UnrealTargetPlatform.Mac)
        {
            PublicAdditionalLibraries.Add(Path.Combine(ThirdPartyPath, "NoCheatLib/lib/Mac/libnocheat.dylib"));
            RuntimeDependencies.Add(Path.Combine("$(BinaryOutputDir)", "libnocheat.dylib"), 
                Path.Combine(ThirdPartyPath, "NoCheatLib/lib/Mac/libnocheat.dylib"));
        }
        else if (Target.Platform == UnrealTargetPlatform.Linux)
        {
            PublicAdditionalLibraries.Add(Path.Combine(ThirdPartyPath, "NoCheatLib/lib/Linux/libnocheat.so"));
            RuntimeDependencies.Add(Path.Combine("$(BinaryOutputDir)", "libnocheat.so"), 
                Path.Combine(ThirdPartyPath, "NoCheatLib/lib/Linux/libnocheat.so"));
        }
    }
}
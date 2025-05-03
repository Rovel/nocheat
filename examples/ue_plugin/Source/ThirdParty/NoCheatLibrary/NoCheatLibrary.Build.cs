using System;
using System.IO;
using UnrealBuildTool;

public class NoCheatLibrary : ModuleRules
{
    public NoCheatLibrary(ReadOnlyTargetRules Target) : base(Target)
    {
        Type = ModuleType.External;
        
        PublicDefinitions.Add("WITH_NOCHEAT_LIBRARY=1");
        
        // Add include paths
        PublicIncludePaths.Add(Path.Combine(ModuleDirectory, "include"));
        
        // Platform specific setup
        if (Target.Platform == UnrealTargetPlatform.Win64)
        {
            // Add the import library
            PublicAdditionalLibraries.Add(Path.Combine(ModuleDirectory, "lib", "Win64", "nocheat.lib"));
            
            // Add the dynamic library for runtime
            PublicDelayLoadDLLs.Add("nocheat.dll");
            
            // Add runtime dependency for the DLL
            RuntimeDependencies.Add("$(BinaryOutputDir)/nocheat.dll", Path.Combine(ModuleDirectory, "lib", "Win64", "nocheat.dll"));
        }
        else if (Target.Platform == UnrealTargetPlatform.Mac)
        {
            // Add the dylib
            PublicAdditionalLibraries.Add(Path.Combine(ModuleDirectory, "lib", "Mac", "libnocheat.dylib"));
            
            // Add runtime dependency for the dylib
            RuntimeDependencies.Add("$(BinaryOutputDir)/libnocheat.dylib", Path.Combine(ModuleDirectory, "lib", "Mac", "libnocheat.dylib"));
        }
        else if (Target.Platform == UnrealTargetPlatform.Linux)
        {
            // Add the shared object
            PublicAdditionalLibraries.Add(Path.Combine(ModuleDirectory, "lib", "Linux", "libnocheat.so"));
            
            // Add runtime dependency for the so
            RuntimeDependencies.Add("$(BinaryOutputDir)/libnocheat.so", Path.Combine(ModuleDirectory, "lib", "Linux", "libnocheat.so"));
        }
    }
}
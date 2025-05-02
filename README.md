# NoCheat

NoCheat is a fast, machine learning-based anti-cheat library designed to detect suspicious player behavior in multiplayer games. It uses a RandomForest classifier to analyze player statistics and identify potential cheaters in real-time.

## Features

- **ML-powered detection**: Uses RandomForest classification to identify suspicious patterns
- **Lightweight**: Small memory footprint and fast execution
- **Language-agnostic**: C-compatible FFI interface for integration with any engine
- **Privacy-focused**: Analyzes gameplay statistics rather than invasive system monitoring
- **Customizable**: Easily tune detection thresholds to match your game's mechanics

## How It Works

NoCheat analyzes player statistics such as:
- Hit rate (hits/shots)
- Headshot rate (headshots/hits)
- Shot timing patterns

The system outputs:
- A suspicion score for each player (0.0 to 1.0)
- Flags for specific suspicious behaviors detected (e.g., "HighHeadshotRatio", "AimSnap")

## Integration with Unreal Engine 5

### Prerequisites

- Unreal Engine 5.0 or newer
- Visual Studio 2019 or newer
- Rust 1.52.0 or newer (for building the library)

### Setup as a C++ Plugin

#### Step 1: Building the NoCheat Library

1. Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

2. Build the library in release mode:
   ```bash
   cd /path/to/nocheat
   cargo build --release
   ```

3. The compiled library will be available at:
   - Windows: `target/release/nocheat.dll`
   - macOS: `target/release/libnocheat.dylib`
   - Linux: `target/release/libnocheat.so`

#### Step 2: Creating the UE5 Plugin

1. Create a new C++ plugin in your UE5 project:
   - In UE5 Editor: `Edit > Plugins > New Plugin > C++ Plugin > Module`
   - Name it "NoCheat"

2. Set up the plugin directory structure:
   ```
   YourProject/Plugins/NoCheat/
   ├── Source/
   │   ├── NoCheat/
   │   │   ├── Private/
   │   │   ├── Public/
   │   │   └── NoCheat.Build.cs
   ├── Resources/
   │   └── cheat_model.bin
   ├── ThirdParty/
   │   └── NoCheatLib/
   │       ├── include/
   │       │   └── nocheat.h
   │       └── lib/
   │           ├── Win64/
   │           ├── Mac/
   │           └── Linux/
   └── NoCheat.uplugin
   ```

3. Copy the compiled library to the appropriate platform folder in `ThirdParty/NoCheatLib/lib/`

4. Create a C interface header in `ThirdParty/NoCheatLib/include/nocheat.h`:
   ```c
   #pragma once

   #ifdef __cplusplus
   extern "C" {
   #endif

   #ifdef _WIN32
       #define NOCHEAT_API __declspec(dllimport)
   #else
       #define NOCHEAT_API
   #endif

   typedef struct PlayerStats {
       const char* player_id;
       const char* stats_json; // JSON string with weapon stats, hits, etc.
   } PlayerStats;

   typedef struct AnalysisResult {
       const char* result_json; // JSON string with suspicion score and flags
   } AnalysisResult;

   // Analyze player stats and return a suspicion score
   NOCHEAT_API int analyze_round(
       const unsigned char* stats_json_ptr,
       size_t stats_json_len,
       unsigned char** out_json_ptr,
       size_t* out_json_len
   );

   // Free memory allocated by the library
   NOCHEAT_API void free_buffer(
       unsigned char* ptr,
       size_t len
   );

   #ifdef __cplusplus
   }
   #endif
   ```

5. Update `NoCheat.Build.cs` to include the library:
   ```csharp
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
   ```

6. Update `NoCheat.uplugin`:
   ```json
   {
       "FileVersion": 3,
       "Version": 1,
       "VersionName": "1.0",
       "FriendlyName": "NoCheat",
       "Description": "Machine learning-based anti-cheat system",
       "Category": "AntiCheat",
       "CreatedBy": "Your Name",
       "CreatedByURL": "",
       "DocsURL": "",
       "MarketplaceURL": "",
       "SupportURL": "",
       "CanContainContent": true,
       "IsBetaVersion": false,
       "IsExperimentalVersion": false,
       "Installed": false,
       "Modules": [
           {
               "Name": "NoCheat",
               "Type": "Runtime",
               "LoadingPhase": "Default"
           }
       ]
   }
   ```

#### Step 3: Creating the C++ Interface Classes

1. Create a NoCheat manager class in your plugin:

   `Public/NoCheatManager.h`:
   ```cpp
   #pragma once

   #include "CoreMinimal.h"
   #include "GameFramework/Actor.h"
   #include "Dom/JsonObject.h"
   #include "NoCheatManager.generated.h"

   USTRUCT(BlueprintType)
   struct FPlayerAnalysisResult
   {
       GENERATED_BODY()

       UPROPERTY(BlueprintReadOnly, Category="AntiCheat")
       FString PlayerID;

       UPROPERTY(BlueprintReadOnly, Category="AntiCheat")
       float SuspicionScore;

       UPROPERTY(BlueprintReadOnly, Category="AntiCheat")
       TArray<FString> Flags;
   };

   UCLASS(Blueprintable)
   class NOCHEAT_API ANoCheatManager : public AActor
   {
       GENERATED_BODY()

   public:
       ANoCheatManager();

       UFUNCTION(BlueprintCallable, Category="AntiCheat")
       TArray<FPlayerAnalysisResult> AnalyzePlayerStats(const TMap<FString, FString>& PlayersStatsJson);

   private:
       TSharedPtr<FJsonObject> ParseAnalysisResponse(const FString& JsonResponse);
   };
   ```

   `Private/NoCheatManager.cpp`:
   ```cpp
   #include "NoCheatManager.h"
   #include "nocheat.h"
   #include "JsonObjectConverter.h"
   #include "Serialization/JsonReader.h"
   #include "Serialization/JsonSerializer.h"

   ANoCheatManager::ANoCheatManager()
   {
       PrimaryActorTick.bCanEverTick = false;
   }

   TArray<FPlayerAnalysisResult> ANoCheatManager::AnalyzePlayerStats(const TMap<FString, FString>& PlayersStatsJson)
   {
       TArray<FPlayerAnalysisResult> Results;
       
       // Prepare input JSON
       TSharedPtr<FJsonObject> JsonObject = MakeShared<FJsonObject>();
       TArray<TSharedPtr<FJsonValue>> PlayerStatsArray;
       
       for (const auto& Pair : PlayersStatsJson)
       {
           // Parse the JSON string for each player
           TSharedPtr<FJsonObject> PlayerStatsJson;
           TSharedRef<TJsonReader<>> Reader = TJsonReaderFactory<>::Create(Pair.Value);
           if (FJsonSerializer::Deserialize(Reader, PlayerStatsJson))
           {
               // Add player ID to the parsed stats
               PlayerStatsJson->SetStringField("player_id", Pair.Key);
               PlayerStatsArray.Add(MakeShared<FJsonValueObject>(PlayerStatsJson));
           }
       }
       
       // Convert to JSON string
       FString JsonString;
       TSharedRef<TJsonWriter<>> Writer = TJsonWriterFactory<>::Create(&JsonString);
       FJsonSerializer::Serialize(PlayerStatsArray, Writer);
       
       // Convert to UTF-8
       FTCHARToUTF8 Converter(*JsonString);
       const int32 Length = Converter.Length();
       const char* Buffer = Converter.Get();
       
       // Call the NoCheat library
       unsigned char* OutJsonPtr = nullptr;
       size_t OutJsonLen = 0;
       
       int Result = analyze_round(
           reinterpret_cast<const unsigned char*>(Buffer),
           Length,
           &OutJsonPtr,
           &OutJsonLen
       );
       
       if (Result == 0 && OutJsonPtr != nullptr)
       {
           // Convert the result back to FString
           FString ResponseJson = FString(UTF8_TO_TCHAR(OutJsonPtr));
           
           // Parse the response
           TSharedPtr<FJsonObject> ResponseObject = ParseAnalysisResponse(ResponseJson);
           if (ResponseObject.IsValid())
           {
               const TArray<TSharedPtr<FJsonValue>>* ResultsArray;
               if (ResponseObject->TryGetArrayField("results", ResultsArray))
               {
                   for (const auto& ResultValue : *ResultsArray)
                   {
                       const TSharedPtr<FJsonObject>* ResultObj;
                       if (ResultValue->TryGetObject(ResultObj))
                       {
                           FPlayerAnalysisResult AnalysisResult;
                           
                           // Extract player_id
                           (*ResultObj)->TryGetStringField("player_id", AnalysisResult.PlayerID);
                           
                           // Extract suspicion_score
                           double Score = 0.0;
                           if ((*ResultObj)->TryGetNumberField("suspicion_score", Score))
                           {
                               AnalysisResult.SuspicionScore = static_cast<float>(Score);
                           }
                           
                           // Extract flags
                           const TArray<TSharedPtr<FJsonValue>>* FlagsArray;
                           if ((*ResultObj)->TryGetArrayField("flags", FlagsArray))
                           {
                               for (const auto& FlagValue : *FlagsArray)
                               {
                                   FString Flag;
                                   if (FlagValue->TryGetString(Flag))
                                   {
                                       AnalysisResult.Flags.Add(Flag);
                                   }
                               }
                           }
                           
                           Results.Add(AnalysisResult);
                       }
                   }
               }
           }
           
           // Free the memory allocated by the library
           free_buffer(OutJsonPtr, OutJsonLen);
       }
       
       return Results;
   }

   TSharedPtr<FJsonObject> ANoCheatManager::ParseAnalysisResponse(const FString& JsonResponse)
   {
       TSharedPtr<FJsonObject> JsonObject = MakeShared<FJsonObject>();
       TSharedRef<TJsonReader<>> Reader = TJsonReaderFactory<>::Create(JsonResponse);
       FJsonSerializer::Deserialize(Reader, JsonObject);
       return JsonObject;
   }
   ```

#### Step 4: Using the Plugin in Your Game

1. Enable the plugin in your UE5 project:
   - In UE5 Editor: `Edit > Plugins`
   - Find "NoCheat" in the list and check "Enabled"
   - Restart the editor when prompted

2. Copy the `cheat_model.bin` file to your project's `Plugins/NoCheat/Resources/` directory.

3. Add the NoCheat manager to your game:
   ```cpp
   // In your GameMode or dedicated server code
   #include "NoCheatManager.h"
   
   void AYourGameMode::BeginPlay()
   {
       Super::BeginPlay();
       
       // Spawn the NoCheat manager
       NoCheatManager = GetWorld()->SpawnActor<ANoCheatManager>();
   }
   
   void AYourGameMode::EndOfRound()
   {
       // Collect player statistics
       TMap<FString, FString> PlayerStats;
       
       for (APlayerController* PC : PlayerControllers)
       {
           AYourPlayerState* PS = PC->GetPlayerState<AYourPlayerState>();
           if (PS)
           {
               // Create JSON representation of player stats
               FString StatsJson = PS->GetStatsAsJson();
               PlayerStats.Add(PS->GetPlayerID(), StatsJson);
           }
       }
       
       // Analyze player stats
       TArray<FPlayerAnalysisResult> Results = NoCheatManager->AnalyzePlayerStats(PlayerStats);
       
       // Process results
       for (const FPlayerAnalysisResult& Result : Results)
       {
           if (Result.SuspicionScore > 0.7f)
           {
               // Take action against potential cheater
               FString Flags = FString::Join(Result.Flags, TEXT(", "));
               UE_LOG(LogTemp, Warning, TEXT("Potential cheater detected: %s (Score: %f, Flags: %s)"),
                   *Result.PlayerID, Result.SuspicionScore, *Flags);
               
               // You can kick, ban, or flag for review
           }
       }
   }
   ```

## Best Practices

1. **Collect Sufficient Data**: The more player statistics you can collect, the more accurate the cheat detection will be.

2. **Tune for Your Game**: Adjust the suspicion thresholds based on your game's mechanics. For example, a sniper-focused game will naturally have higher headshot ratios.

3. **Update Regularly**: Keep your cheat model updated as new cheating methods emerge. Consider implementing a system to update the model remotely.

4. **Combine with Other Systems**: NoCheat works best as part of a layered anti-cheat approach:
   - Server-side movement validation
   - Client-side integrity checks
   - Statistical analysis (NoCheat)
   - Community reporting

## Customizing Detection

To customize the detection thresholds for your specific game, you can modify:

1. **Model Parameters**: Train your own RandomForest model with specific weights for your game's mechanics
2. **Flag Thresholds**: Adjust the thresholds in the `do_analysis` function in `src/lib.rs`

## License

[Include your license information here]

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.
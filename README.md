# NoCheat

NoCheat is a fast, machine learning-based anti-cheat library designed to detect suspicious player behavior in multiplayer games. It uses a RandomForest classifier to analyze player statistics and identify potential cheaters in real-time.

## Features

- Fast, real-time analysis of player statistics
- Machine learning-based detection of suspicious patterns
- Support for training custom models with your own data
- C-compatible FFI for integration with game engines
- UE5 plugin integration ready
- DataFrame-based feature engineering
- **Generic struct support for custom data analysis**

## How It Works

NoCheat uses a RandomForest classifier trained on player statistics to identify suspicious behavior patterns. The system:

1. Collects player statistics (shots, hits, headshots, etc.)
2. Extracts meaningful features (accuracy rates, headshot ratios)
3. Passes these features to a pre-trained model
4. Returns suspicion scores and specific behavioral flags

## Training Custom Models

NoCheat now includes functionality to train your own custom cheat detection models:

### Option 1: Generate a Default Model

The quickest way to get started is to generate a default model based on synthetic data:

```rust
use nocheat::generate_default_model;

// Generate a default model with synthetic training data
generate_default_model("cheat_model.bin").expect("Failed to generate default model");
```

This will create a model file that's ready to use for basic cheat detection.

### Option 2: Train a Custom Model with Your Data

For better results, you can train a model with your own labeled data:

```rust
use nocheat::{train_model};
use nocheat::types::{DefaultPlayerData, PlayerStats};
use std::collections::HashMap;

// Prepare your training data
let mut training_data = Vec::new();
let mut labels = Vec::new();

// Example: Add a legitimate player
let mut shots = HashMap::new();
shots.insert("rifle".to_string(), 100);
let mut hits = HashMap::new();
hits.insert("rifle".to_string(), 50);

let player_data = DefaultPlayerData {
    shots_fired: shots.clone(),
    hits: hits.clone(),
    headshots: 10,
    shot_timestamps_ms: None,
    training_label: None,
};

training_data.push(PlayerStats::new(
    "normal_player".to_string(),
    player_data
));
labels.push(0.0); // Not a cheater

// Example: Add a cheating player
let mut shots = HashMap::new();
shots.insert("rifle".to_string(), 100);
let mut hits = HashMap::new();
hits.insert("rifle".to_string(), 95); // Suspiciously high accuracy

let cheater_data = DefaultPlayerData {
    shots_fired: shots,
    hits: hits,
    headshots: 70, // Very high headshot ratio
    shot_timestamps_ms: None,
    training_label: None,
};

training_data.push(PlayerStats::new(
    "cheater".to_string(),
    cheater_data
));
labels.push(1.0); // Labeled as a cheater

// Train and save model
train_model(training_data, labels, "cheat_model.bin").expect("Failed to train model");
```

## Using Generic Types for Custom Data Analysis

NoCheat now supports generic data structures, giving you the flexibility to work with any JSON structure for player statistics. This is particularly useful when:

1. Your game has unique metrics to track
2. You want to analyze different aspects of player behavior
3. You need to integrate with existing analytics systems

### Example: Creating a Custom Data Structure

```rust
use nocheat::types::{PlayerStats, PlayerResult, AnalysisResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define a custom data structure for your game
#[derive(Clone, Debug, Deserialize, Serialize)]
struct CustomPlayerData {
    accuracy: f32,
    reaction_time_ms: Vec<u32>,
    movement_patterns: HashMap<String, u32>,
    mouse_acceleration: Option<f32>,
}

// Create stats with your custom data structure
let mut movement = HashMap::new();
movement.insert("jumps".to_string(), 50);
movement.insert("crouches".to_string(), 30);

let custom_data = CustomPlayerData {
    accuracy: 0.75,
    reaction_time_ms: vec![250, 220, 230, 210, 240],
    movement_patterns: movement,
    mouse_acceleration: Some(1.5),
};

// Create a PlayerStats instance with custom data
let custom_stats = PlayerStats::new("custom_player".to_string(), custom_data);
```

### Custom Result Types

You can also define custom result types for your analysis:

```rust
#[derive(Debug, PartialEq, Serialize)]
struct CustomAnalysisResult {
    cheating_probability: f32,
    abnormal_patterns: Vec<String>,
    confidence_score: f32,
    recommended_action: String,
}

// Create a custom result
let custom_result = CustomAnalysisResult {
    cheating_probability: 0.85,
    abnormal_patterns: vec!["AimSnap".to_string(), "RecoilControl".to_string()],
    confidence_score: 0.92,
    recommended_action: "Review gameplay footage".to_string(),
};

let player_result = PlayerResult::new("custom_player".to_string(), custom_result);
```

For more detailed examples, see the [Generic Usage Guide](docs/generic_usage.md).

## Integration with Unreal Engine 5

### Prerequisites

- Unreal Engine 5.0 or newer
- Visual Studio 2019 or newer (for Windows) or appropriate IDE for your platform
- Rust 1.52.0 or newer (for building the library)

### Setup as a Third-Party UE5 Plugin

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
1. Create a new plugin in your UE5 project:
   - Open your project in UE5.
   - Go to `Edit > Plugins`.
   - Click on `New Plugin` and select `Third-Party Plugin Template`.
   - Name it `NoCheat`.

1. UE will create a plugin with the following directory structure in your UE project:
   ```
   YourProject/Plugins/NoCheat/
   ├── NoCheat.uplugin
   ├── Resources/
   │   └── Icon128.png
   ├── Source/
   │   └── NoCheat/
   │       ├── Private/
   │       │   ├── NoCheat.cpp
   │       │   └── NoCheatManager.cpp (Not Included)
   │       ├── Public/
   │       │   ├── NoCheat.h
   │       │   └── NoCheatManager.h (Not Included)
   │       └── NoCheat.Build.cs
   └── ThirdParty/
       └── NoCheatLibrary/
           ├── include/ (Not Included)
           │   └── nocheat.h (Not Included)
           └── lib/
               ├── Win64/
               │   ├── nocheat.dll (Not Included)
               │   └── nocheat.lib (Not Included)
               ├── Mac/
               │   └── libnocheat.dylib (Not Included)
               └── Linux/
                   └── libnocheat.so (Not Included)
   ```

2. Copy the compiled library to the appropriate platform folder in `ThirdParty/NoCheatLibrary/lib/`

3. Create a C interface header in `ThirdParty/NoCheatLibrary/include/nocheat.h` or generate one with `cbindgen --config cbindgen.toml --crate nocheat --output ue_plugin\ThirdParty\NoCheatLib\include\nocheat.h`:
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

4. Create the `NoCheat.Build.cs` file to configure how the plugin is built:
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

           PublicIncludePaths.Add(Path.Combine(ThirdPartyPath, "NoCheatLibrary/include"));

           PublicDependencyModuleNames.AddRange(new string[] {
               "Core",
               "CoreUObject",
               "Engine",
               "Json",
               "JsonUtilities"
           });

           if (Target.Platform == UnrealTargetPlatform.Win64)
           {
               PublicAdditionalLibraries.Add(Path.Combine(ThirdPartyPath, "NoCheatLibrary/lib/Win64/nocheat.lib"));
               PublicDelayLoadDLLs.Add("nocheat.dll");
               RuntimeDependencies.Add(Path.Combine("$(BinaryOutputDir)", "nocheat.dll"), 
                   Path.Combine(ThirdPartyPath, "NoCheatLibrary/lib/Win64/nocheat.dll"));
           }
           else if (Target.Platform == UnrealTargetPlatform.Mac)
           {
               PublicAdditionalLibraries.Add(Path.Combine(ThirdPartyPath, "NoCheatLibrary/lib/Mac/libnocheat.dylib"));
               RuntimeDependencies.Add(Path.Combine("$(BinaryOutputDir)", "libnocheat.dylib"), 
                   Path.Combine(ThirdPartyPath, "NoCheatLibrary/lib/Mac/libnocheat.dylib"));
           }
           else if (Target.Platform == UnrealTargetPlatform.Linux)
           {
               PublicAdditionalLibraries.Add(Path.Combine(ThirdPartyPath, "NoCheatLibrary/lib/Linux/libnocheat.so"));
               RuntimeDependencies.Add(Path.Combine("$(BinaryOutputDir)", "libnocheat.so"), 
                   Path.Combine(ThirdPartyPath, "NoCheatLibrary/lib/Linux/libnocheat.so"));
           }
       }
   }
   ```

5. Create the `NoCheat.uplugin` file:
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

2. **Train With Real Data**: While the default model provides a starting point, training with real data from your specific game will yield much better results.

3. **Tune for Your Game**: Adjust the suspicion thresholds based on your game's mechanics. For example, a sniper-focused game will naturally have higher headshot ratios.

4. **Update Regularly**: Keep your cheat model updated as new cheating methods emerge. Consider implementing a system to update the model remotely.

5. **Combine with Other Systems**: NoCheat works best as part of a layered anti-cheat approach:
   - Server-side movement validation
   - Client-side integrity checks
   - Statistical analysis (NoCheat)
   - Community reporting

## Customizing Detection

To customize the detection for your specific game, you can:

1. **Train Your Own Model**: Use the `train_model` function with your own labeled dataset
2. **Generate a Starter Model**: Use `generate_default_model` and fine-tune it later
3. **Adjust Flag Thresholds**: Modify the thresholds in the `do_analysis` function in `src/lib.rs`

## License

[Include your license information here]

## Contributing

[Include contribution guidelines here]
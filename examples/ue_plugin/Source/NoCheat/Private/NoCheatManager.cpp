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
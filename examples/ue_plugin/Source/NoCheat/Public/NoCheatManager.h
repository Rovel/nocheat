#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "Dom/JsonObject.h"
#include "NoCheatManager.generated.h"

/**
 * Results from player behavior analysis
 */
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

/**
 * Manages the NoCheat anti-cheat system integration with Unreal Engine
 */
UCLASS(Blueprintable)
class NOCHEAT_API ANoCheatManager : public AActor
{
    GENERATED_BODY()

public:
    ANoCheatManager();

    /**
     * Analyzes player statistics from a game round to detect possible cheating
     * 
     * @param PlayersStatsJson Map of player IDs to their statistics in JSON format
     * @return Array of analysis results for each player
     */
    UFUNCTION(BlueprintCallable, Category="AntiCheat")
    TArray<FPlayerAnalysisResult> AnalyzePlayerStats(const TMap<FString, FString>& PlayersStatsJson);

private:
    /**
     * Helper method to parse the JSON response from the NoCheat library
     */
    TSharedPtr<FJsonObject> ParseAnalysisResponse(const FString& JsonResponse);
};
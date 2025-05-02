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

/**
 * Analyzes player statistics for suspicious behavior
 * @param stats_json_ptr Pointer to UTF-8 encoded JSON buffer containing player stats
 * @param stats_json_len Length of the JSON buffer in bytes
 * @param out_json_ptr Pointer to a location where output buffer pointer will be stored
 * @param out_json_len Pointer to a location where output buffer length will be stored
 * @return 0 on success, negative values on error
 */
NOCHEAT_API int analyze_round(
    const unsigned char* stats_json_ptr,
    size_t stats_json_len,
    unsigned char** out_json_ptr,
    size_t* out_json_len
);

/**
 * Frees memory allocated by analyze_round
 * @param ptr Pointer to the buffer to free
 * @param len Length of the buffer
 */
NOCHEAT_API void free_buffer(
    unsigned char* ptr,
    size_t len
);

#ifdef __cplusplus
}
#endif
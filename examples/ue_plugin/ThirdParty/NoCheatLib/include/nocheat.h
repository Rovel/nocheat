#pragma once

#ifdef __cplusplus
extern "C" {
    /// FFI: analyze a JSON buffer of PlayerStats; returns JSON buffer
///
/// This function provides a C-compatible interface for the cheat detection system.
/// It takes a JSON buffer containing player statistics, analyzes them, and returns
/// the results as a JSON buffer.
///
/// # Safety
///
/// This function is unsafe because it deals with raw pointers and memory allocation
/// across the FFI boundary. The caller is responsible for:
///
/// - Ensuring the input pointers are valid and properly aligned
/// - Freeing the returned buffer using the `free_buffer` function
///
/// # Arguments
///
/// * `stats_json_ptr` - Pointer to a UTF-8 encoded JSON buffer
/// * `stats_json_len` - Length of the JSON buffer in bytes
/// * `out_json_ptr` - Pointer to a location where the output buffer pointer will be stored
/// * `out_json_len` - Pointer to a location where the output buffer length will be stored
///
/// # Returns
///
/// * `0` on success
/// * Negative values on various errors:
///   * `-1` - Null pointer provided
///   * `-2` - JSON parsing error
///   * `-3` - Analysis error
///   * `-4` - Serialization error
///   * `-5` - Memory allocation error
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

/**
 * Set a custom path to load the model from
 * @param path_ptr Pointer to a UTF-8 encoded path string
 * @param path_len Length of the path string in bytes
 * @return 0 on success, negative values on error:
 *         -1: Null path provided
 *         -2: Invalid UTF-8 path
 *         -3: File doesn't exist
 *         -4: Model couldn't be deserialized
 */
NOCHEAT_API int set_model_path(
    const unsigned char* path_ptr,
    size_t path_len
);

#ifdef __cplusplus
}
#endif
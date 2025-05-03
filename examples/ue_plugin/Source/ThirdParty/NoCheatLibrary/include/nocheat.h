#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#ifdef _WIN32
    #define NOCHEAT_API __declspec(dllimport)
#else
    #define NOCHEAT_API
#endif

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
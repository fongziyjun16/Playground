#pragma once

#ifdef LIVE_PUSH_EXPORTS
#define LIVE_PUSH_LIB_API __declspec(dllexport)
#else
#define LIVE_PUSH_LIB_API __declspec(dllimport)
#endif

extern "C" LIVE_PUSH_LIB_API void start_push(void (*push)(const unsigned char* buffer, int size));

extern "C" LIVE_PUSH_LIB_API void stop_push();

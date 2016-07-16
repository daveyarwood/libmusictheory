
#ifndef cheddar_generated_musictheory_h
#define cheddar_generated_musictheory_h


#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>



/// Given a string describing a note in scientific pitch (e.g. "C#5", "Dbb4",
/// "E0"), returns an integer representing the note as an unbounded MIDI note
/// number.
intptr_t musictheory_note(char* note);



#ifdef __cplusplus
}
#endif


#endif

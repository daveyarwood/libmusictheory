#include <stdio.h>
#include "../include/musictheory.h"

void test_musictheory_note() {
  char* note = "C#4";

  int note_out = musictheory_note(note);
  printf("musictheory_note(%s) is %d\n", note, note_out);
}

void test_musictheory_spell_note() {
  char* csharp = musictheory_spell_note(61, 'C');
  printf("musictheory_spell_note(61, 'C') is %s\n", csharp);
  char* dflat = musictheory_spell_note(61, 'D');
  printf("musictheory_spell_note(61, 'D') is %s\n", dflat);
}

int main() {
  test_musictheory_note();

  printf("%s\n", "-------------------------------------");
  test_musictheory_spell_note();
}

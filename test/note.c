#include <stdio.h>
#include "../include/musictheory.h"

int main() {
  char* note = "C#4";
  printf("note is %s\n", note);

  int note_out = musictheory_note(note);
  printf("musictheory_note(note) is %d", note_out);
}

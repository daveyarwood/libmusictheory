#include <stdio.h>

int main() {
  char* note = "C#4";
  printf("note is %s\n", note);

  int note_out = music_note(note);
  printf("music_note(note) is %d", note_out);
}

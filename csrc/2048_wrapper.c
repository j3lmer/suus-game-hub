// csrc/2048_wrapper.c
#include "2048.h" // This must be the first include for your project's headers

// Now these calls should resolve because init_game, key_event, and draw_screen
// are declared in 2048.h
void game2048_init() {
    init_game();
}

void game2048_restart() {
    init_game(); // Reusing init_game for restart
}

void game2048_handle_input(int key) {
    key_event(key);
}

void game2048_render() {
    draw_screen();
}

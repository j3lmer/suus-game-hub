// csrc/2048.h
#ifndef C2048_H
#define C2048_H

#include <stdint.h> // For uint8_t, uint32_t
#include <stdbool.h> // For bool

// Define SIZE if it's not already defined for the header
#ifndef SIZE
#define SIZE 4
#endif

// All functions defined in 2048.c that are called by other files
void getColors(uint8_t value, uint8_t scheme, uint8_t *foreground, uint8_t *background);
uint8_t getDigitCount(uint32_t number);
void drawBoard(uint8_t board[SIZE][SIZE], uint8_t scheme, uint32_t score);
bool slideArray(uint8_t array[SIZE], uint32_t *score);
void rotateBoard(uint8_t board[SIZE][SIZE]);
bool moveUp(uint8_t board[SIZE][SIZE], uint32_t *score);
bool moveLeft(uint8_t board[SIZE][SIZE], uint32_t *score);
bool moveDown(uint8_t board[SIZE][SIZE], uint32_t *score);
bool moveRight(uint8_t board[SIZE][SIZE], uint32_t *score);
bool findPairDown(uint8_t board[SIZE][SIZE]);
uint8_t countEmpty(uint8_t board[SIZE][SIZE]);
bool gameEnded(uint8_t board[SIZE][SIZE]);
void addRandom(uint8_t board[SIZE][SIZE]);
void initBoard(uint8_t board[SIZE][SIZE]);
void setBufferedInput(bool enable);
bool testSucceed();
void signal_callback_handler(int signum);

// Functions that are part of the 'exported API' for the wrapper (and are implemented in 2048.c)
void init_game(void);    // <--- Ensure this is present
void key_event(int c);   // <--- Ensure this is present
void draw_screen(void);  // <--- Ensure this is present

// Global variables declared in 2048.c that are accessed by other files (e.g., wrapper)
extern uint8_t game_board[SIZE][SIZE];
extern uint32_t game_score;
extern uint8_t game_scheme;

int start_2048(void);

// Wrapper functions (declared here, implemented in 2048_wrapper.c)
void game2048_init();
void game2048_restart();
void game2048_handle_input(int key);
void game2048_render();

#endif // C2048_H
